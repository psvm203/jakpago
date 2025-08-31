use crate::api;
use crate::calculator;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};
use yew::{Callback, Event, Html, KeyboardEvent, MouseEvent, function_component, html};
use yew_hooks::{UseLocalStorageHandle, UseMapHandle, use_effect_once, use_local_storage, use_map};

mod constants {
    pub const FIELD_DATA: &str = include_str!("field.yaml");
    pub const FIELD_STORAGE_KEY: &str = "field";
    pub const KEY_ENTER: &str = "Enter";

    pub mod texts {
        pub const FIELD_DATA_ERROR_MESSAGE: &str = "필드 데이터 오류:";
        pub const POTENTIAL_LEGEND: &str = "확률 정보";
        pub const EQUIPMENT_LEGEND: &str = "장비 정보";
        pub const PRICE_LEGEND: &str = "시세 정보";
        pub const CHARACTER_SEARCH_PLACEHOLDER: &str = "캐릭터 검색";
        pub const CALCULATE: &str = "계산";
    }

    pub mod skills {
        pub const ENHANCE_MASTERY: &str = "강화의 달인";
        pub const UPGRADE_SALVATION: &str = "실패를 두려워 않는";
    }
}

#[allow(clippy::wildcard_imports)]
use constants::*;

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
enum FieldId {
    Handicraft,
    EnhancementMastery,
    UpgradeSalvation,
    UpgradeableCount,
    TraceRequired,
    TracePrice,
}

impl FieldId {
    fn get_tooltip(self, value: u32) -> Option<String> {
        #[allow(clippy::enum_glob_use)]
        use FieldId::*;

        match self {
            Handicraft => {
                Some(format!("성공 확률 {}%p 증가", calculator::handicraft_effect(value)))
            }
            EnhancementMastery => {
                Some(format!("성공 확률 {}%p 증가", calculator::enhance_mastery_effect(value)))
            }
            UpgradeSalvation => Some(format!(
                "실패 시 {}% 확률로 횟수 차감 방지",
                calculator::upgrade_salvation_effect(value)
            )),
            _ => None,
        }
    }
}

#[derive(Clone, Deserialize)]
struct Field {
    id: FieldId,
    label: &'static str,
    placeholder: &'static str,
    min: u32,
    max: u32,
}

impl Field {
    const fn is_valid_value(&self, value: u32) -> bool {
        self.min <= value && value <= self.max
    }
}

type FieldMap = HashMap<FieldId, u32>;

#[derive(Clone)]
struct State {
    map: UseMapHandle<FieldId, u32>,
    storage: UseLocalStorageHandle<FieldMap>,
}

impl State {
    const fn new(
        map: UseMapHandle<FieldId, u32>,
        storage: UseLocalStorageHandle<FieldMap>,
    ) -> Self {
        Self {
            map,
            storage,
        }
    }

    fn insert(&self, key: FieldId, value: u32) {
        self.map.insert(key, value);
    }

    fn get(&self, key: FieldId) -> Option<u32> {
        self.map.current().get(&key).copied()
    }

    fn filtered(&self, fields: &FieldRegistry) -> FieldMap {
        self.map
            .current()
            .iter()
            .filter(|&(&id, &value)| fields.get(id).min <= value && value <= fields.get(id).max)
            .map(|(&id, &value)| (id, value))
            .collect()
    }

    fn save_to_storage(&self, value: FieldMap) {
        self.storage.set(value);
    }

    async fn fetch_character_data(states: Self, character_name: String) {
        let handicraft_level =
            api::get_handicraft_level_by_character_name(character_name.clone()).await;

        if let Ok(handicraft_level) = handicraft_level {
            states.insert(FieldId::Handicraft, handicraft_level);
        }

        let enhance_mastery_level = api::get_guild_skill_level_by_character_name(
            character_name.clone(),
            skills::ENHANCE_MASTERY,
        )
        .await;

        if let Ok(enhance_mastery_level) = enhance_mastery_level {
            states.insert(FieldId::EnhancementMastery, enhance_mastery_level);
        }

        let upgrade_salvation_level =
            api::get_guild_skill_level_by_character_name(character_name, skills::UPGRADE_SALVATION)
                .await;

        if let Ok(upgrade_salvation_level) = upgrade_salvation_level {
            states.insert(FieldId::UpgradeSalvation, upgrade_salvation_level);
        }
    }

    fn search_character(&self) -> Callback<KeyboardEvent> {
        let states = self.clone();

        Callback::from(move |event: KeyboardEvent| {
            if event.key() == KEY_ENTER {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                if let Some(character_name) = input {
                    let states = states.clone();
                    let name = character_name.value();

                    spawn_local(async move {
                        Self::fetch_character_data(states, name).await;
                    });
                }
            }
        })
    }

    fn character_search_item(&self) -> Html {
        let onkeydown = self.search_character();

        html! {
            <label class={"input"}>
                <svg
                    class={"h-[1em] opacity-50"}
                    xmlns={"http://www.w3.org/2000/svg"}
                    viewBox={"0 0 24 24"}
                >
                    <g
                        stroke-linejoin={"round"}
                        stroke-linecap={"round"}
                        stroke-width={"2.5"}
                        fill={"none"}
                        stroke={"currentColor"}
                    >
                        <circle cx={"11"} cy={"11"} r={"8"} />
                        <path d={"m21 21-4.3-4.3"} />
                    </g>
                </svg>
                <input
                    type={"search"}
                    class={"grow"}
                    placeholder={texts::CHARACTER_SEARCH_PLACEHOLDER}
                    {onkeydown}
                />
                <kbd class="kbd kbd-sm">
                    { "↵" }
                </kbd>
            </label>
        }
    }

    fn tooltip_item(&self, field_id: FieldId) -> Html {
        let value = self.get(field_id).unwrap_or(0);
        let tooltip = field_id.get_tooltip(value);

        html! {
            <div>
                { tooltip }
            </div>
        }
    }
}

struct FieldRegistry {
    map: HashMap<FieldId, Field>,
}

impl FieldRegistry {
    fn load() -> Self {
        let map = serde_yaml::from_str::<Vec<Field>>(FIELD_DATA)
            .inspect_err(|err| {
                gloo_console::error!(texts::FIELD_DATA_ERROR_MESSAGE, err.to_string());
            })
            .unwrap_or_default()
            .into_iter()
            .map(|field: Field| (field.id, field))
            .collect::<HashMap<FieldId, Field>>();

        Self {
            map,
        }
    }

    fn get(&self, id: FieldId) -> &Field {
        self.map.get(&id).unwrap()
    }
}

fn on_field_change(states: &State, field: &Field) -> Callback<Event> {
    let states = states.clone();
    let field = field.clone();
    let id = field.id;

    Callback::from(move |event: Event| {
        let target = event.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input
            && let Ok(value) = input.value().parse::<u32>()
            && field.is_valid_value(value)
        {
            states.insert(id, value);
        }
    })
}

fn field_item(states: &State, field: &Field) -> Html {
    let value = states.get(field.id).map(|x| x.to_string());
    let min = field.min.to_string();
    let max = field.max.to_string();
    let onchange = on_field_change(states, field);

    html! {
        <div>
            <label class={"label"} for={field.label}>
                { field.label }
            </label>
            <input
                type={"number"}
                id={field.label}
                class={"input validator"}
                required={true}
                placeholder={field.placeholder}
                {value}
                {min}
                {max}
                {onchange}
            />
        </div>
    }
}

fn fieldset_item(legend: &'static str, contents: Html) -> Html {
    html! {
        <fieldset class={"fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4"}>
            <legend class={"fieldset-legend"}>
                { legend }
            </legend>
            { contents }
        </fieldset>
    }
}

fn calculate(fields: &FieldRegistry, states: &State) -> Callback<MouseEvent> {
    let states = states.clone();
    let value = states.filtered(fields);

    Callback::from({
        move |_| {
            states.save_to_storage(value.clone());
        }
    })
}

fn calculate_button(fields: &FieldRegistry, states: &State) -> Html {
    let onclick = calculate(fields, states);

    html! {
        <button class={"btn btn-primary"} {onclick}>
            { texts::CALCULATE }
        </button>
    }
}

#[function_component]
pub fn InputSection() -> Html {
    let map = use_map(FieldMap::new());
    let storage = use_local_storage::<FieldMap>(FIELD_STORAGE_KEY.to_owned());

    {
        let map = map.clone();
        let storage = storage.clone();

        use_effect_once(move || {
            if let Some(storage) = storage.as_ref() {
                map.set(storage.clone());
            }

            || {}
        });
    }

    let states = State::new(map, storage);
    let fields = FieldRegistry::load();

    let potential_fieldset: Html =
        [FieldId::Handicraft, FieldId::EnhancementMastery, FieldId::UpgradeSalvation]
            .iter()
            .map(|&id| {
                html! {
                    <div>
                        { field_item(&states, fields.get(id)) }
                        { states.tooltip_item( id) }
                    </div>
                }
            })
            .collect();

    let potential_fieldset = html! {
        <div>
            { states.character_search_item() }
            { potential_fieldset }
        </div>
    };

    let item_fieldset: Html = [FieldId::UpgradeableCount, FieldId::TraceRequired]
        .iter()
        .map(|&id| field_item(&states, fields.get(id)))
        .collect();

    let price_fieldset = field_item(&states, fields.get(FieldId::TracePrice));

    html! {
        <div class={"grid grid-cols-6 gap-48 p-16"}>
            { fieldset_item(texts::POTENTIAL_LEGEND, potential_fieldset) }
            { fieldset_item(texts::EQUIPMENT_LEGEND, item_fieldset) }
            { fieldset_item(texts::PRICE_LEGEND, price_fieldset) }
            { calculate_button(&fields, &states) }
        </div>
    }
}

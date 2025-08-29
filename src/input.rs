use crate::api;
use crate::calculator;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};
use yew::{Callback, Event, Html, KeyboardEvent, MouseEvent, function_component, html};
use yew_hooks::{UseLocalStorageHandle, UseMapHandle, use_effect_once, use_local_storage, use_map};

const FIELD_DATA: &str = include_str!("field.yaml");
const FIELD_DATA_ERROR_MESSAGE: &str = "필드 데이터 오류:";
const FIELD_STORAGE_KEY: &str = "field";
const POTENTIAL_LEGEND: &str = "확률 정보";
const EQUIPMENT_LEGEND: &str = "장비 정보";
const PRICE_LEGEND: &str = "시세 정보";
const CHARACTER_SEARCH_PLACEHOLDER: &str = "캐릭터 검색";
const KEY_ENTER: &str = "Enter";
const CALCULATE: &str = "계산";
const SKILL_NAME_ENHANCE_MASTERY: &str = "강화의 달인";
const SKILL_NAME_UPGRADE_SALVATION: &str = "실패를 두려워 않는";

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
enum FieldId {
    Handicraft,
    EnhancementMastery,
    UpgradeSalvation,
    UpgradeableCount,
    TraceRequired,
    TracePrice,
}

#[derive(Deserialize)]
struct Field {
    id: FieldId,
    label: &'static str,
    placeholder: &'static str,
    min: u32,
    max: u32,
}

fn load_fields() -> Vec<Field> {
    match serde_yaml::from_str(FIELD_DATA) {
        Ok(fields) => fields,
        Err(err) => {
            gloo_console::error!(FIELD_DATA_ERROR_MESSAGE, err.to_string());
            vec![]
        }
    }
}

fn search_character(field_states: &UseMapHandle<FieldId, u32>) -> Callback<KeyboardEvent> {
    let field_states = field_states.clone();

    Callback::from(move |event: KeyboardEvent| {
        if event.key() == KEY_ENTER {
            let target = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(character_name) = input {
                let field_states = field_states.clone();

                spawn_local(async move {
                    let handicraft_level =
                        api::get_handicraft_level_by_character_name(character_name.value()).await;

                    if let Ok(handicraft_level) = handicraft_level {
                        field_states.insert(FieldId::Handicraft, handicraft_level);
                    }

                    let enhance_mastery_level = api::get_guild_skill_level_by_character_name(
                        character_name.value(),
                        SKILL_NAME_ENHANCE_MASTERY,
                    )
                    .await;

                    if let Ok(enhance_mastery_level) = enhance_mastery_level {
                        field_states.insert(FieldId::EnhancementMastery, enhance_mastery_level);
                    }

                    let upgrade_salvation_level = api::get_guild_skill_level_by_character_name(
                        character_name.value(),
                        SKILL_NAME_UPGRADE_SALVATION,
                    )
                    .await;

                    if let Ok(upgrade_salvation_level) = upgrade_salvation_level {
                        field_states.insert(FieldId::UpgradeSalvation, upgrade_salvation_level);
                    }
                });
            }
        }
    })
}

fn character_search_item(field_states: &UseMapHandle<FieldId, u32>) -> Html {
    let onkeydown = search_character(field_states);

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
                placeholder={CHARACTER_SEARCH_PLACEHOLDER}
                {onkeydown}
            />
            <kbd class="kbd kbd-sm">
                { "↵" }
            </kbd>
        </label>
    }
}

fn on_field_change(field_states: &UseMapHandle<FieldId, u32>, field: &Field) -> Callback<Event> {
    let states = field_states.clone();
    let min = field.min;
    let max = field.max;
    let id = field.id;

    Callback::from(move |event: Event| {
        let target = event.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input
            && let Ok(value) = input.value().parse::<u32>()
        {
            if value < min {
                return;
            }

            if value > max {
                return;
            }

            states.insert(id, value);
        }
    })
}

fn field_item(field_states: &UseMapHandle<FieldId, u32>, field: &Field) -> Html {
    let value = field_states.current().get(&field.id).map(u32::to_string);
    let min = field.min.to_string();
    let max = field.max.to_string();
    let onchange = on_field_change(field_states, field);

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

fn get_tooltip(field_id: FieldId, value: u32) -> Option<String> {
    match field_id {
        FieldId::Handicraft => {
            Some(format!("성공 확률 {}%p 증가", calculator::handicraft_effect(value)))
        }
        FieldId::EnhancementMastery => {
            Some(format!("성공 확률 {}%p 증가", calculator::enhance_mastery_effect(value)))
        }
        FieldId::UpgradeSalvation => Some(format!(
            "실패 시 {}% 확률로 횟수 차감 방지",
            calculator::upgrade_salvation_effect(value)
        )),
        _ => None,
    }
}

fn tooltip_item(field_states: &UseMapHandle<FieldId, u32>, field_id: FieldId) -> Html {
    let value = *field_states.current().get(&field_id).unwrap_or(&0);
    let tooltip = get_tooltip(field_id, value);

    html! {
        <div>
            { tooltip }
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

fn calculate(
    fields: &HashMap<FieldId, Field>,
    field_states: &UseMapHandle<FieldId, u32>,
    field_storage: &UseLocalStorageHandle<HashMap<FieldId, u32>>,
) -> Callback<MouseEvent> {
    let storage = field_storage.clone();

    let map: HashMap<FieldId, u32> = field_states
        .current()
        .iter()
        .filter(|&(id, &value)| fields[id].min <= value && value <= fields[id].max)
        .map(|(&id, &value)| (id, value))
        .collect();

    Callback::from(move |_| {
        storage.set(map.clone());
    })
}

fn calculate_button(
    fields: &HashMap<FieldId, Field>,
    field_states: &UseMapHandle<FieldId, u32>,
    field_storage: &UseLocalStorageHandle<HashMap<FieldId, u32>>,
) -> Html {
    let onclick = calculate(fields, field_states, field_storage);

    html! {
        <button class={"btn btn-primary"} {onclick}>
            { CALCULATE }
        </button>
    }
}

#[function_component]
pub fn InputSection() -> Html {
    let field_states = use_map(HashMap::<FieldId, u32>::new());
    let field_storage = use_local_storage::<HashMap<FieldId, u32>>(FIELD_STORAGE_KEY.to_owned());

    {
        let field_storage = field_storage.clone();
        let field_states = field_states.clone();

        use_effect_once(move || {
            if let Some(storage) = field_storage.as_ref() {
                field_states.set(storage.clone());
            }

            || {}
        });
    }

    let fields: HashMap<FieldId, Field> =
        load_fields().into_iter().map(|field| (field.id, field)).collect();

    let potential_fieldset: Html =
        [FieldId::Handicraft, FieldId::EnhancementMastery, FieldId::UpgradeSalvation]
            .iter()
            .map(|id| {
                html! {
                    <div>
                        { field_item(&field_states, &fields[id]) }
                        { tooltip_item(&field_states, *id) }
                    </div>
                }
            })
            .collect();

    let potential_fieldset = html! {
        <div>
            { character_search_item(&field_states) }
            { potential_fieldset }
        </div>
    };

    let item_fieldset: Html = [FieldId::UpgradeableCount, FieldId::TraceRequired]
        .iter()
        .map(|id| field_item(&field_states, &fields[id]))
        .collect();

    let price_fieldset = field_item(&field_states, &fields[&FieldId::TracePrice]);

    html! {
        <div class={"grid grid-cols-6 gap-48 p-16"}>
            { fieldset_item(POTENTIAL_LEGEND, potential_fieldset) }
            { fieldset_item(EQUIPMENT_LEGEND, item_fieldset) }
            { fieldset_item(PRICE_LEGEND, price_fieldset) }
            { calculate_button(&fields, &field_states, &field_storage) }
        </div>
    }
}

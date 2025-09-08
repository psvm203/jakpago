use crate::{State, strategy};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};
use yew::{Callback, Event, Html, MouseEvent, html};

mod constants {
    pub const FIELD_DATA: &str = include_str!("assets/data/field.yaml");

    pub mod texts {
        pub const FIELD_DATA_ERROR_MESSAGE: &str = "필드 데이터 오류:";
        pub const POTENTIAL_LEGEND: &str = "확률 정보";
        pub const EQUIPMENT_LEGEND: &str = "장비 정보";
        pub const PRICE_LEGEND: &str = "시세 정보";
        pub const CALCULATE: &str = "계산";
    }
}

#[allow(clippy::wildcard_imports)]
use constants::*;

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum FieldId {
    Handicraft,
    EnhancementMastery,
    UpgradeSalvation,
    UpgradeableCount,
    TraceRequired,
    TracePrice,
    BaseSuccessRate,
    UpgradePrice,
}

impl FieldId {
    pub fn get_tooltip(self, value: u32) -> Option<String> {
        #[allow(clippy::enum_glob_use)]
        use FieldId::*;

        match self {
            Handicraft => Some(format!("성공 확률 {}%p 증가", strategy::handicraft_effect(value))),
            EnhancementMastery => {
                Some(format!("성공 확률 {}%p 증가", strategy::enhance_mastery_effect(value)))
            }
            UpgradeSalvation => Some(format!(
                "실패 시 {}% 확률로 횟수 차감 방지",
                strategy::upgrade_salvation_effect(value)
            )),
            _ => None,
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct Field {
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

    pub fn get_min(&self) -> u32 {
        self.min
    }

    pub fn get_max(&self) -> u32 {
        self.min
    }
}

pub type FieldMap = HashMap<FieldId, u32>;

pub(crate) struct FieldRegistry {
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

    pub fn get(&self, id: FieldId) -> &Field {
        self.map.get(&id).unwrap()
    }
}

fn on_field_change(state: &State, field: &Field) -> Callback<Event> {
    let state = state.clone();
    let field = field.clone();
    let id = field.id;

    Callback::from(move |event: Event| {
        let target = event.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input
            && let Ok(value) = input.value().parse::<u32>()
            && field.is_valid_value(value)
        {
            state.insert(id, value);
        }
    })
}

fn field_item(state: &State, field: &Field) -> Html {
    let value = state.get(field.id).map(|x| x.to_string());
    let min = field.min.to_string();
    let max = field.max.to_string();
    let onchange = on_field_change(state, field);

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

fn calculate(fields: &FieldRegistry, state: &State) -> Callback<MouseEvent> {
    let state = state.clone();
    let value = state.filtered(fields);
    let _ = strategy::optimized_strategy;

    Callback::from({
        move |_| {
            state.save_to_storage(value.clone());
        }
    })
}

fn calculate_button(fields: &FieldRegistry, state: &State) -> Html {
    let onclick = calculate(fields, state);

    html! {
        <button class={"btn btn-primary"} {onclick}>
            { texts::CALCULATE }
        </button>
    }
}

pub fn input_section(state: &State) -> Html {
    let fields = FieldRegistry::load();

    let potential_fieldset: Html =
        [FieldId::Handicraft, FieldId::EnhancementMastery, FieldId::UpgradeSalvation]
            .iter()
            .map(|&id| {
                html! {
                    <div>
                        { field_item(&state, fields.get(id)) }
                        { state.tooltip_item( id) }
                    </div>
                }
            })
            .collect();

    let potential_fieldset = html! {
        <div>
            { state.character_search_item() }
            { potential_fieldset }
        </div>
    };

    let item_fieldset: Html = [FieldId::UpgradeableCount, FieldId::TraceRequired]
        .iter()
        .map(|&id| field_item(&state, fields.get(id)))
        .collect();

    let price_fieldset = field_item(&state, fields.get(FieldId::TracePrice));

    html! {
        <div class={"grid grid-cols-6 gap-48 p-16"}>
            { fieldset_item(texts::POTENTIAL_LEGEND, potential_fieldset) }
            { fieldset_item(texts::EQUIPMENT_LEGEND, item_fieldset) }
            { fieldset_item(texts::PRICE_LEGEND, price_fieldset) }
            { calculate_button(&fields, &state) }
        </div>
    }
}

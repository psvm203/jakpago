use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;
use strum_macros::{Display, EnumString};
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};
use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component]
fn App() -> Html {
    let theme_controller = {
        const THEME_STORAGE_KEY: &str = "theme";
        const THEME_DEFAULT_VALUE: &str = "default";
        const THEME_LABEL: &str = "테마";

        #[derive(Clone, Deserialize)]
        struct Theme {
            value: String,
            name: &'static str,
        }

        let theme_state = use_local_storage::<String>(THEME_STORAGE_KEY.to_owned());

        {
            let theme_state = theme_state.clone();

            use_effect_once(move || {
                if theme_state.is_none() {
                    theme_state.set(THEME_DEFAULT_VALUE.to_owned());
                }

                || {}
            });
        }

        let theme_item = |theme: &Theme| -> Html {
            let on_theme_change = {
                let theme_state = theme_state.clone();
                let theme_value = theme.value.clone();

                move |_| {
                    theme_state.set(theme_value.clone());
                }
            };

            let current_theme = theme_state.as_deref().unwrap();

            html! {
                <li key={theme.value.clone()}>
                    <input
                        type={"radio"}
                        name={"theme-dropdown"}
                        class={"theme-controller w-full btn btn-sm btn-block btn-ghost justify-start"}
                        aria-label={theme.name}
                        value={theme.value.clone()}
                        checked={theme.value == current_theme}
                        onchange={on_theme_change}
                    />
                </li>
            }
        };

        let theme_data = include_str!("theme.yaml");
        let themes: Vec<Theme> = serde_yaml::from_str(theme_data).unwrap();
        let theme_items: Html = themes.into_iter().map(|theme| theme_item(&theme)).collect();

        html! {
            <div class={"dropdown mb-72 absolute right-48"}>
                <div tabindex={"0"} role={"button"} class={"btn m-1"}>
                    { THEME_LABEL }
                    <svg
                        width={"12px"}
                        height={"12px"}
                        class={"inline-block h-2 w-2 fill-current opacity-60"}
                        xmlns={"http://www.w3.org/2000/svg"}
                        viewBox={"0 0 2048 2048"}
                    >
                        <path d={"M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"} />
                    </svg>
                </div>
                <ul
                    tabindex={"0"}
                    class={"dropdown-content bg-base-300 rounded-box z-1 w-52 p-2 shadow-2xl"}
                >
                    { theme_items }
                </ul>
            </div>
        }
    };

    let fieldsets = {
        const CHARACTER_SEARCH: &str = "캐릭터 검색";
        const EQUIPMENT_SEARCH: &str = "장비 검색";
        const CHARACTER_INFORMATION: &str = "캐릭터 정보";
        const EQUIPMENT_INFORMATION: &str = "장비 정보";
        const PRICE_INFORMATION: &str = "시세 정보";
        const ADDITIONAL_CONFIGURATION: &str = "추가 설정";

        #[derive(Clone, Display, EnumString, Eq, Hash, PartialEq)]
        enum FieldId {
            Diligence,
            EnhancementMastery,
            UpgradeSalvation,
            UpgradeableCount,
            TraceRequired,
            TracePrice,
            InnocencePrice,
            ArkInnocencePrice,
            WhitePrice,
        }

        #[derive(Deserialize)]
        struct Field {
            id: &'static str,
            label: &'static str,
            placeholder: &'static str,
            min: Option<usize>,
            max: Option<usize>,
        }

        impl Field {
            fn onchange(&self, state: UseStateHandle<usize>) -> Callback<Event> {
                let min = self.min;
                let max = self.max;

                Callback::from(move |event: Event| {
                    let target = event.target();
                    let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                    if let Some(input) = input
                        && let Ok(value) = input.value().parse::<usize>()
                    {
                        if let Some(min) = min
                            && value < min
                        {
                            return;
                        }

                        if let Some(max) = max
                            && value > max
                        {
                            return;
                        }

                        state.set(value);
                    }
                })
            }
        }

        let field_data = include_str!("field.yaml");
        let fields: Vec<Field> = serde_yaml::from_str(field_data).unwrap();

        let states = HashMap::from([
            (FieldId::Diligence, use_state(|| 0_usize)),
            (FieldId::EnhancementMastery, use_state(|| 0_usize)),
            (FieldId::UpgradeSalvation, use_state(|| 0_usize)),
            (FieldId::UpgradeableCount, use_state(|| 0_usize)),
            (FieldId::TraceRequired, use_state(|| 0_usize)),
            (FieldId::TracePrice, use_state(|| 0_usize)),
            (FieldId::InnocencePrice, use_state(|| 0_usize)),
            (FieldId::ArkInnocencePrice, use_state(|| 0_usize)),
            (FieldId::WhitePrice, use_state(|| 0_usize)),
        ]);

        let get_description = |id: FieldId, x: usize| -> Option<String> {
            match id {
                FieldId::Diligence => {
                    Some(format!("성공 확률 {}%p 증가", (x / 5 * 5) as f64 / 10.0))
                }
                FieldId::EnhancementMastery => Some(format!("성공 확률 {x}%p 증가")),
                FieldId::UpgradeSalvation => Some(format!("실패 시 {x}% 확률로 횟수 차감 방지")),
                _ => None,
            }
        };

        let field_item = |field: &Field| -> Html {
            let min = field.min.map(|min| min.to_string());
            let max = field.max.map(|max| max.to_string());
            let id = FieldId::from_str(field.id).unwrap();
            let onchange = field.onchange((*states.get(&id).unwrap()).clone());
            let value = (**states.get(&id).unwrap()).to_string();
            let description = get_description(id.clone(), **states.get(&id).unwrap());

            html! {
                <div>
                    <label class={"label"} for={field.id}>
                        { field.label }
                    </label>
                    <input
                        type={"number"}
                        id={field.id}
                        class={"input validator"}
                        required={true}
                        placeholder={field.placeholder}
                        {value}
                        {min}
                        {max}
                        {onchange}
                    />
                    <div>
                        { description }
                    </div>
                </div>
            }
        };

        let fieldset_item = |legend: &str, contents: Html| -> Html {
            html! {
                <fieldset
                    class={"fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4"}
                >
                    <legend class={"fieldset-legend"}>
                        { legend }
                    </legend>
                    { contents }
                </fieldset>
            }
        };

        let search_input = |placeholder: &str,
                            onchange: Option<Callback<Event>>,
                            list_id: Option<&'static str>,
                            contents: Option<Html>|
         -> Html {
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
                        placeholder={placeholder.to_owned()}
                        {onchange}
                        list={list_id}
                    />
                    { contents }
                </label>
            }
        };

        let field_map: HashMap<FieldId, &Field> =
            fields.iter().map(|field| (FieldId::from_str(field.id).unwrap(), field)).collect();

        let character_fieldset = {
            let fields =
                [FieldId::Diligence, FieldId::EnhancementMastery, FieldId::UpgradeSalvation];

            let fields: Vec<&Field> =
                fields.iter().map(|field| *field_map.get(field).unwrap()).collect();

            let field_items: Html = fields.into_iter().map(&field_item).collect();

            let keyboard = html! {
                <kbd class={"kbd kbd-sm"}>
                    { "↵" }
                </kbd>
            };

            let contents = html! {
                <div>
                    { search_input(CHARACTER_SEARCH, None, None, Some(keyboard)) }
                    { field_items }
                </div>
            };

            fieldset_item(CHARACTER_INFORMATION, contents)
        };

        let button_item = |label: &str| -> Html {
            html! {
                <button class={"btn btn-info"}>
                    { label }
                </button>
            }
        };

        let equipment_fieldset = {
            const EQUIPMENT_LIST_ID: &str = "equipments";
            const EQUIPMENT_NONE: &str = "없음";

            #[allow(dead_code)]
            #[derive(Clone, Deserialize)]
            struct Equipment {
                name: &'static str,
                alias: Option<Vec<&'static str>>,
                class: &'static str,
                level: usize,
                count: usize,
            }

            let equipment_data = include_str!("equipment.yaml");
            let equipments: Vec<Equipment> = serde_yaml::from_str(equipment_data).unwrap();

            let equipment_map: HashMap<&str, Equipment> =
                equipments.iter().map(|equipment| (equipment.name, equipment.clone())).collect();

            let equipment_search_state = use_state(|| EQUIPMENT_NONE.to_owned());

            let equipment_search = {
                let state = equipment_search_state.clone();

                let upgradeable_count_state =
                    states.get(&FieldId::UpgradeableCount).unwrap().clone();

                Callback::from(move |event: Event| {
                    let target = event.target();
                    let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                    if let Some(input) = input {
                        let value = &input.value();

                        if let Some(equipment) = equipment_map.get(value.as_str()) {
                            state.set(equipment.name.to_owned());
                            upgradeable_count_state.set(equipment.count);
                        } else {
                            state.set(EQUIPMENT_NONE.to_owned());
                        }
                    }
                })
            };

            let suggestion_item = |name: &'static str| -> Html {
                html! { <option value={name} /> }
            };

            let suggestion_items: Html =
                equipments.iter().map(|equipment| suggestion_item(equipment.name)).collect();

            let suggestion = html! {
                <datalist id={EQUIPMENT_LIST_ID}>
                    { suggestion_items }
                </datalist>
            };

            let buttons = ["100%", "70%", "30%", "15%"];
            let button_items: Html = buttons.into_iter().map(button_item).collect();

            let fields = [FieldId::UpgradeableCount, FieldId::TraceRequired];

            let fields: Vec<&Field> =
                fields.iter().map(|field| *field_map.get(field).unwrap()).collect();

            let field_items: Html = fields.into_iter().map(&field_item).collect();

            let contents = html! {
                <div>
                    { search_input(EQUIPMENT_SEARCH, Some(equipment_search), Some(EQUIPMENT_LIST_ID), None) }
                    { suggestion }
                    { button_items }
                    { field_items }
                </div>
            };

            fieldset_item(EQUIPMENT_INFORMATION, contents)
        };

        let price_fieldset = {
            let fields = [
                FieldId::TracePrice,
                FieldId::InnocencePrice,
                FieldId::ArkInnocencePrice,
                FieldId::WhitePrice,
            ];

            let fields: Vec<&Field> =
                fields.iter().map(|field| *field_map.get(field).unwrap()).collect();

            let field_items: Html = fields.into_iter().map(&field_item).collect();

            let contents = html! {
                <div>
                    { field_items }
                </div>
            };

            fieldset_item(PRICE_INFORMATION, contents)
        };

        let additional_fieldset = {
            let fever_buttons: Html = {
                let buttons = ["피버 타임 O", "피버 타임 X"];
                buttons.into_iter().map(&button_item).collect()
            };

            let innocence_buttons: Html = {
                let buttons = ["이노센트 주문서", "아크 이노센트 주문서"];
                buttons.into_iter().map(&button_item).collect()
            };

            let sunday_button: Html =
                { button_item("썬데이 메이플: 강화 비용 50% 할인") };

            let contents = html! {
                <div>
                    { fever_buttons }
                    { innocence_buttons }
                    { sunday_button }
                </div>
            };

            fieldset_item(ADDITIONAL_CONFIGURATION, contents)
        };

        html! {
            <div class={"grid grid-cols-6 gap-48 p-16"}>
                { character_fieldset }
                { equipment_fieldset }
                { price_fieldset }
                { additional_fieldset }
            </div>
        }
    };

    html! {
        <div>
            { theme_controller }
            { fieldsets }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

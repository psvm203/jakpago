use gloo_storage::{LocalStorage, Storage};
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;
use strum_macros::{Display, EnumString};
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let theme_controller = {
        const THEME_STORAGE_KEY: &str = "theme";
        const THEME_DEFAULT_VALUE: &str = "default";

        #[derive(Clone, Deserialize)]
        struct Theme {
            value: String,
            name: String,
        }

        let initial_theme = LocalStorage::get::<String>(THEME_STORAGE_KEY)
            .unwrap_or_else(|_| THEME_DEFAULT_VALUE.to_owned());

        let theme_state = use_mut_ref(|| initial_theme);

        let theme_item = |theme: &Theme| -> Html {
            let on_theme_change = {
                let theme_state = theme_state.clone();
                let theme_value = theme.value.clone();

                move |_| {
                    *theme_state.borrow_mut() = theme_value.clone();
                    LocalStorage::set(THEME_STORAGE_KEY, theme_value.clone()).unwrap();
                }
            };

            html! {
                <li key={theme.value.clone()}>
                    <input
                        type={"radio"}
                        name={"theme-dropdown"}
                        class={"theme-controller w-full btn btn-sm btn-block btn-ghost justify-start"}
                        aria-label={theme.name.clone()}
                        value={theme.value.clone()}
                        checked={theme.value == *theme_state.borrow()}
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
                    { "테마" }
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
        #[derive(Clone, Display, EnumString, Eq, Hash, PartialEq)]
        enum FieldId {
            Diligence,
            EnhancementMastery,
            UpgradeSalvation,
            EnhancementSlots,
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
            fn onchange(&self, states: HashMap<FieldId, UseStateHandle<usize>>) -> Callback<Event> {
                let state = states.get(&FieldId::from_str(self.id).unwrap()).unwrap().clone();
                let min = self.min;
                let max = self.max;

                Callback::from(move |event: Event| {
                    let target = event.target();
                    let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                    if let Some(input) = input {
                        match input.value().parse::<usize>() {
                            Ok(value) => {
                                if let Some(min) = min {
                                    if value < min {
                                        return;
                                    }
                                }

                                if let Some(max) = max {
                                    if value > max {
                                        return;
                                    }
                                }

                                state.set(value);
                            }
                            Err(_) => return,
                        }
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
            (FieldId::EnhancementSlots, use_state(|| 0_usize)),
            (FieldId::TraceRequired, use_state(|| 0_usize)),
            (FieldId::TracePrice, use_state(|| 0_usize)),
            (FieldId::InnocencePrice, use_state(|| 0_usize)),
            (FieldId::ArkInnocencePrice, use_state(|| 0_usize)),
            (FieldId::WhitePrice, use_state(|| 0_usize)),
        ]);

        let descript = |id: FieldId, x: usize| -> Option<String> {
            match id {
                FieldId::Diligence => {
                    Some(format!("성공 확률 {}%p 증가", (x / 5 * 5) as f64 / 10.0))
                }
                FieldId::EnhancementMastery => Some(format!("성공 확률 {}%p 증가", x)),
                FieldId::UpgradeSalvation => Some(format!("실패 시 {}% 확률로 횟수 차감 방지", x)),
                _ => None,
            }
        };

        let field_item = |field: &Field| -> Html {
            let min = field.min.map(|min| min.to_string());
            let max = field.max.map(|max| max.to_string());
            let onchange = field.onchange(states.clone());
            let id = FieldId::from_str(field.id).unwrap();
            let description = descript(id.clone(), **states.get(&id).unwrap());

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

        let search_input = |placeholder: &str, contents: Html| -> Html {
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
                    <input type={"search"} class={"grow"} placeholder={placeholder.to_owned()} />
                    { contents }
                </label>
            }
        };

        let field_map: HashMap<FieldId, &Field> =
            fields.iter().map(|field| (FieldId::from_str(field.id).unwrap(), field)).collect();

        let character_fieldset = {
            let fields =
                vec![FieldId::Diligence, FieldId::EnhancementMastery, FieldId::UpgradeSalvation];

            let fields: Vec<&Field> =
                fields.iter().map(|field| *field_map.get(field).unwrap()).collect();

            let field_items: Html = fields.into_iter().map(|field| field_item(field)).collect();

            let keyboard = html! {
                <kbd class={"kbd kbd-sm"}>
                    { "↵" }
                </kbd>
            };

            let contents = html! {
                <div>
                    { search_input("캐릭터 검색", keyboard) }
                    { field_items }
                </div>
            };

            fieldset_item("캐릭터 정보", contents)
        };

        let item_fieldset = {
            let buttons = vec!["100%", "70%", "30%", "15%"];

            let button_item = |label: &str| -> Html {
                html! {
                    <button class={"btn btn-info"}>
                        { label }
                    </button>
                }
            };

            let button_items: Html = buttons.into_iter().map(button_item).collect();

            let fields = vec![FieldId::EnhancementSlots, FieldId::TraceRequired];

            let fields: Vec<&Field> =
                fields.iter().map(|field| *field_map.get(field).unwrap()).collect();

            let field_items: Html = fields.into_iter().map(|field| field_item(field)).collect();

            let contents = html! {
                <div>
                    { search_input("아이템 검색", html!{}) }
                    { button_items }
                    { field_items }
                </div>
            };

            fieldset_item("아이템 정보", contents)
        };

        let price_fieldset = {
            let fields = vec![
                FieldId::TracePrice,
                FieldId::InnocencePrice,
                FieldId::ArkInnocencePrice,
                FieldId::WhitePrice,
            ];

            let fields: Vec<&Field> =
                fields.iter().map(|field| *field_map.get(field).unwrap()).collect();

            let field_items: Html = fields.into_iter().map(|field| field_item(field)).collect();

            let contents = html! {
                <div>
                    { field_items }
                </div>
            };

            fieldset_item("시세 정보", contents)
        };

        html! {
            <div class={"grid grid-cols-6 gap-48 p-16"}>
                { character_fieldset }
                { item_fieldset }
                { price_fieldset }
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

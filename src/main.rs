use gloo_storage::{LocalStorage, Storage};
use serde::Deserialize;
use std::collections::HashMap;
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
        const DILIGENCE: &str = "diligence";
        const ENHANCEMENT_MASTERY: &str = "enhancement_mastery";
        const UPGRADE_SALVATION: &str = "upgrade_salvation";
        const ENHANCEMENT_SLOTS: &str = "enhancement_slots";
        const TRACES_REQUIRED: &str = "traces_required";
        const TRACES_PRICE: &str = "traces_price";
        const INNOCENCE_PRICE: &str = "innocence_price";
        const ARK_INNOCENCE_PRICE: &str = "ark_innocence_price";
        const WHITE_PRICE: &str = "white_price";

        #[derive(Deserialize)]
        struct Field {
            id: &'static str,
            label: &'static str,
            placeholder: &'static str,
            min: Option<usize>,
            max: Option<usize>,
        }

        impl Field {
            fn onchange(
                &self,
                states: HashMap<&'static str, UseStateHandle<usize>>,
            ) -> Callback<Event> {
                let state = states.get(self.id).unwrap().clone();
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
            (DILIGENCE, use_state(|| 0_usize)),
            (ENHANCEMENT_MASTERY, use_state(|| 0_usize)),
            (UPGRADE_SALVATION, use_state(|| 0_usize)),
            (ENHANCEMENT_SLOTS, use_state(|| 0_usize)),
            (TRACES_REQUIRED, use_state(|| 0_usize)),
            (TRACES_PRICE, use_state(|| 0_usize)),
            (INNOCENCE_PRICE, use_state(|| 0_usize)),
            (ARK_INNOCENCE_PRICE, use_state(|| 0_usize)),
            (WHITE_PRICE, use_state(|| 0_usize)),
        ]);

        let descript_diligence: Box<dyn Fn(usize) -> String> =
            Box::new(|diligence: usize| -> String {
                format!("성공 확률 {}%p 증가", (diligence / 5 * 5) as f64 / 10.0)
            });

        let descript_enhancement_mastery: Box<dyn Fn(usize) -> String> =
            Box::new(|enhancement_mastery: usize| -> String {
                format!("성공 확률 {}%p 증가", enhancement_mastery)
            });

        let descript_upgrade_salvation: Box<dyn Fn(usize) -> String> =
            Box::new(|upgrade_salvation: usize| -> String {
                format!("실패 시 {}% 확률로 횟수 차감 방지", upgrade_salvation)
            });

        let descriptions = HashMap::from([
            (DILIGENCE, descript_diligence),
            (ENHANCEMENT_MASTERY, descript_enhancement_mastery),
            (UPGRADE_SALVATION, descript_upgrade_salvation),
        ]);

        let field_item = |field: &Field| -> Html {
            let min = field.min.map(|min| min.to_string());
            let max = field.max.map(|max| max.to_string());
            let onchange = field.onchange(states.clone());

            let description = descriptions
                .get(field.id)
                .map(|descript| descript(*states.get(field.id).unwrap().clone()));

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

        let field_map: HashMap<&'static str, &Field> =
            fields.iter().map(|field| (field.id, field)).collect();

        let character_fieldset = {
            let fields = vec![DILIGENCE, ENHANCEMENT_MASTERY, UPGRADE_SALVATION];

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
            let fields = vec![ENHANCEMENT_SLOTS, TRACES_REQUIRED];

            let fields: Vec<&Field> =
                fields.iter().map(|field| *field_map.get(field).unwrap()).collect();

            let field_items: Html = fields.into_iter().map(|field| field_item(field)).collect();

            let contents = html! {
                <div>
                    { search_input("아이템 검색", html!{}) }
                    { field_items }
                </div>
            };

            fieldset_item("아이템 정보", contents)
        };

        let price_fieldset = {
            let fields = vec![TRACES_PRICE, INNOCENCE_PRICE, ARK_INNOCENCE_PRICE, WHITE_PRICE];

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

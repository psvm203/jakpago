use serde::Deserialize;
use yew::prelude::*;
use yew_hooks::prelude::*;

const THEME_STORAGE_KEY: &str = "theme";
const THEME_DEFAULT_VALUE: &str = "default";
const THEME_LABEL: &str = "테마";

#[derive(Clone, Deserialize)]
struct Theme {
    value: &'static str,
    name: &'static str,
}

fn on_theme_change(
    theme_state: &UseLocalStorageHandle<String>,
    theme_value: &'static str,
) -> Callback<Event> {
    let theme_state = theme_state.clone();

    Callback::from(move |_| {
        theme_state.set(theme_value.to_owned());
    })
}

fn theme_item(theme_state: &UseLocalStorageHandle<String>, theme: &Theme) -> Html {
    let current_theme = theme_state.as_deref().unwrap_or(THEME_DEFAULT_VALUE);
    let onchange = on_theme_change(&theme_state, theme.value);

    html! {
        <li key={theme.value}>
            <input
                type={"radio"}
                name={"theme-dropdown"}
                class={"theme-controller w-full btn btn-sm btn-block btn-ghost justify-start"}
                aria-label={theme.name}
                value={theme.value}
                checked={theme.value == current_theme}
                {onchange}
            />
        </li>
    }
}

#[function_component]
pub fn ThemeController() -> Html {
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

    let theme_data = include_str!("theme.yaml");
    let themes: Vec<Theme> = serde_yaml::from_str(theme_data).unwrap();

    let theme_items: Html =
        themes.into_iter().map(|theme| theme_item(&theme_state, &theme)).collect();

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
}

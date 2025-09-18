use crate::view_models::theme_view_model_sycamore::{Theme, ThemeViewModel};
use sycamore::prelude::*;

mod constants {
    pub const THEME_LABEL: &str = "테마";
}

#[component]
pub fn ThemeView() -> View {
    let view_model = ThemeViewModel::new();
    provide_context(view_model);

    view! { ThemeController() }
}

#[component]
fn ThemeController() -> View {
    view! {
        div(class="dropdown mb-72 absolute right-48") {
            div(tabindex="0", role="button", class="btn m-1") {
                (constants::THEME_LABEL)
                svg(
                    width="12px",
                    height="12px",
                    class="inline-block h-2 w-2 fill-current opacity-60",
                    xmlns="http://www.w3.org/2000/svg",
                    viewBox="0 0 2048 2048"
                ) {
                    path(d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z") {}
                }
            }
            ul(
                tabindex="0",
                class="dropdown-content bg-base-300 rounded-box z-1 w-52 p-2 shadow-2xl"
            ) {
                (theme_options)
            }
        }
    }
}

fn theme_options() -> Vec<View> {
    let view_model = use_context::<ThemeViewModel>();

    view_model.theme_collection.themes.iter().map(theme_option).collect()
}

fn theme_option(theme_data: &Theme) -> View {
    let view_model = use_context::<ThemeViewModel>();
    let label = theme_data.label;
    let value = theme_data.value;
    let checked = value == view_model.get_theme();
    let onchange = move |_event: web_sys::Event| view_model.theme_change_callback(value);

    view! {
        li {
            input(
                r#type="radio",
                name="theme-dropdown",
                class="theme-controller w-full btn btn-sm btn-block btn-ghost justify-start",
                aria-label=label,
                value=value,
                checked=checked,
                on:change=onchange
            ) {}
        }
    }
}

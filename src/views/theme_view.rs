use crate::viewmodels::theme_viewmodel::{ThemeData, ThemeViewModel};
use yew::{Callback, ContextProvider, Event, Html, function_component, html, use_context};

mod constants {
    pub const THEME_LABEL: &str = "테마";
}

#[function_component]
pub fn Contents() -> Html {
    let theme = ThemeViewModel::use_theme();
    let theme_viewmodel = ThemeViewModel::new(theme);

    html! {
        <ContextProvider<ThemeViewModel> context={theme_viewmodel}>
            <ThemeController />
        </ContextProvider<ThemeViewModel>>
    }
}

#[function_component]
fn ThemeController() -> Html {
    html! {
        <div class={"dropdown mb-72 absolute right-48"}>
            <div tabindex={"0"} role={"button"} class={"btn m-1"}>
                { constants::THEME_LABEL }
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
                <ThemeOptions />
            </ul>
        </div>
    }
}

#[function_component]
fn ThemeOptions() -> Html {
    let theme_viewmodel = use_context::<ThemeViewModel>().unwrap();
    let current_theme = theme_viewmodel.current_theme();

    let create_theme_change_callback = |theme_value: &'static str| -> Callback<Event> {
        let theme_viewmodel = theme_viewmodel.clone();
        Callback::from(move |_| {
            theme_viewmodel.set_current_theme(theme_value);
        })
    };

    let theme_option = |theme_data: &ThemeData| -> Html {
        let onchange = create_theme_change_callback(theme_data.value());

        html! {
            <li key={theme_data.value()}>
                <input
                    type={"radio"}
                    name={"theme-dropdown"}
                    class={"theme-controller w-full btn btn-sm btn-block btn-ghost justify-start"}
                    aria-label={theme_data.name()}
                    value={theme_data.value()}
                    checked={theme_data.value() == current_theme}
                    {onchange}
                />
            </li>
        }
    };

    theme_viewmodel.themes_data().iter().map(theme_option).collect()
}

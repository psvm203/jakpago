use crate::{State, input, theme};

use yew::prelude::*;

pub fn contents(state: &State) -> Html {
    html! {
        <div>
            <theme::ThemeController />
            { input::input_section(&state) }
        </div>
    }
}

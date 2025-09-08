use crate::{State, input, views::theme_view};
use yew::prelude::*;

pub fn contents(state: &State) -> Html {
    html! {
        <div>
            <theme_view::Contents />
            { input::input_section(&state) }
        </div>
    }
}

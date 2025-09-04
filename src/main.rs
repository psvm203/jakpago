mod api;
mod input;
mod state;
mod strategy;
mod theme;

use state::State;
use yew::{Html, function_component, html};
use yew_hooks::{use_effect_once, use_local_storage, use_map};

const FIELD_STORAGE_KEY: &str = "fields";

#[function_component]
fn App() -> Html {
    let map = use_map(input::FieldMap::new());
    let storage = use_local_storage::<input::FieldMap>(FIELD_STORAGE_KEY.to_owned());

    {
        let map = map.clone();
        let storage = storage.clone();

        use_effect_once(move || {
            if let Some(storage) = storage.as_ref() {
                map.set(storage.clone());
            }

            || {}
        });
    }

    let state = State::new(map, storage);

    html! {
        <div>
            <theme::ThemeController />
            { input::input_section(&state) }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

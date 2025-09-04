mod api;
mod strategy;
mod input;
mod theme;

use yew::{Html, function_component, html};

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <theme::ThemeController />
            <input::InputSection />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

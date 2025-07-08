use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            { "Hello, World!" }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

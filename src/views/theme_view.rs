use crate::views::theme_view_sycamore;
use sycamore::web::document;
use yew::{Html, function_component, html};
use yew_hooks::use_effect_once;

#[function_component]
pub fn Contents() -> Html {
    use_effect_once(move || {
        if let Ok(Some(node)) = document().query_selector("div#theme_sycamore") {
            sycamore::render_to(theme_view_sycamore::ThemeView, &node);
        }

        || ()
    });

    html! { <div id="theme_sycamore" /> }
}

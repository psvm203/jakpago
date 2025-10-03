use crate::views::{theme_view, upgrade_context_view};
use sycamore::prelude::*;

#[component]
pub fn App() -> View {
    view! {
        theme_view::ThemeView()
        upgrade_context_view::UpgradeContextView()
    }
}

pub use crate::models::theme::{THEMES, Theme};
use crate::utils::sycamore::Callback;
use gloo_storage::{LocalStorage, Storage};
use sycamore::prelude::*;
use web_sys::Event;

mod constants {
    pub const THEME_STORAGE_KEY: &str = "theme";
    pub const THEME_DEFAULT_VALUE: &str = "default";
}

#[derive(Clone)]
pub struct ThemeViewModel {
    pub current_theme: Signal<String>,
}

impl ThemeViewModel {
    pub fn new() -> Self {
        let stored_theme = LocalStorage::get(constants::THEME_STORAGE_KEY)
            .unwrap_or_else(|_| constants::THEME_DEFAULT_VALUE.to_owned());

        Self {
            current_theme: create_signal(stored_theme),
        }
    }

    pub fn theme_change_callback(&self, theme_value: &'static str) -> Callback {
        let current_theme = self.current_theme;

        Callback::from(move |_event: Event| {
            current_theme.set(theme_value.to_owned());
            LocalStorage::set(constants::THEME_STORAGE_KEY, theme_value.to_owned()).unwrap();
        })
    }
}

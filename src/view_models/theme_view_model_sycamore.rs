pub use crate::models::theme_sycamore::Theme;
use crate::models::theme_sycamore::ThemeCollection;
use gloo_storage::{LocalStorage, Storage};
use sycamore::prelude::{Signal, create_signal};

mod constants {
    pub const THEME_STORAGE_KEY: &str = "theme";
    pub const THEME_DEFAULT_VALUE: &str = "default";
}

#[derive(Clone)]
pub struct ThemeViewModel {
    pub theme_collection: ThemeCollection,
    current_theme: ThemeState,
}

impl ThemeViewModel {
    pub fn new() -> Self {
        Self {
            theme_collection: ThemeCollection::new(),
            current_theme: ThemeState::new(),
        }
    }

    pub fn get_theme(&self) -> String {
        self.current_theme.get_theme_untracked()
    }

    pub fn theme_change_callback(&self, theme_value: &'static str) {
        self.current_theme.set_theme(theme_value.to_owned());
        self.current_theme.store_in_storage();
    }
}

#[derive(Clone)]
struct ThemeState {
    theme: Signal<String>,
}

impl ThemeState {
    fn new() -> Self {
        let stored_theme = LocalStorage::get(constants::THEME_STORAGE_KEY)
            .unwrap_or_else(|_| constants::THEME_DEFAULT_VALUE.to_owned());

        Self {
            theme: create_signal(stored_theme),
        }
    }

    fn get_theme_untracked(&self) -> String {
        self.theme.get_clone_untracked()
    }

    fn set_theme(&self, theme: String) {
        self.theme.set(theme);
    }

    fn store_in_storage(&self) {
        LocalStorage::set(constants::THEME_STORAGE_KEY, self.get_theme_untracked()).unwrap();
    }
}

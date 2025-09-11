use crate::models::theme::ThemeCollection;
use yew::{Callback, Event, hook};
use yew_hooks::{UseLocalStorageHandle, use_effect_once, use_local_storage};

mod constants {
    pub const THEME_STORAGE_KEY: &str = "theme";
    pub const THEME_DEFAULT_VALUE: &str = "default";
}

#[derive(Clone, PartialEq)]
pub struct ThemeViewModel {
    current_theme: UseLocalStorageHandle<String>,
    theme_collection: ThemeCollection,
}

impl ThemeViewModel {
    pub fn current_theme(&self) -> &str {
        self.current_theme.as_deref().unwrap_or(constants::THEME_DEFAULT_VALUE)
    }

    fn set_current_theme(&self, theme_value: &'static str) {
        self.current_theme.set(theme_value.to_owned());
    }

    pub fn new(current_theme: UseLocalStorageHandle<String>) -> Self {
        Self {
            current_theme,
            theme_collection: ThemeCollection::new(),
        }
    }

    #[hook]
    pub fn use_theme() -> UseLocalStorageHandle<String> {
        let theme = use_local_storage(constants::THEME_STORAGE_KEY.to_owned());

        {
            let theme = theme.clone();
            use_effect_once(move || {
                if theme.is_none() {
                    theme.set(constants::THEME_DEFAULT_VALUE.to_owned());
                }
                || {}
            });
        }

        theme
    }

    pub fn create_theme_change_callback(&self, theme_value: &'static str) -> Callback<Event> {
        let view_model = self.clone();
        Callback::from(move |_: Event| {
            view_model.set_current_theme(theme_value);
        })
    }

    pub fn themes_data(&self) -> Vec<ThemeData> {
        self.theme_collection
            .themes()
            .iter()
            .map(|theme| ThemeData {
                value: theme.value(),
                name: theme.name(),
            })
            .collect()
    }
}

pub struct ThemeData {
    value: &'static str,
    name: &'static str,
}

impl ThemeData {
    pub const fn value(&self) -> &'static str {
        self.value
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }
}

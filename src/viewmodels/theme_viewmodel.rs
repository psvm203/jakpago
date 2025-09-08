use crate::models::theme::Theme;
use yew::hook;
use yew_hooks::{UseLocalStorageHandle, use_effect_once, use_local_storage};

mod constants {
    pub const THEME_DATA: &str = include_str!("../assets/data/theme.yaml");
    pub const THEME_STORAGE_KEY: &str = "theme";
    pub const THEME_DEFAULT_VALUE: &str = "default";
}

#[derive(Clone, PartialEq)]
pub struct ThemeViewModel {
    current_theme: UseLocalStorageHandle<String>,
    themes: Vec<Theme>,
}

impl ThemeViewModel {
    pub fn current_theme(&self) -> &str {
        self.current_theme.as_deref().unwrap_or(constants::THEME_DEFAULT_VALUE)
    }

    pub fn set_current_theme(&self, theme_value: &'static str) {
        self.current_theme.set(theme_value.to_owned());
    }

    pub fn new(current_theme: UseLocalStorageHandle<String>) -> Self {
        Self {
            current_theme,
            themes: Self::load_themes(constants::THEME_DATA),
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

    fn load_themes(yaml_data: &'static str) -> Vec<Theme> {
        serde_yaml::from_str(yaml_data).unwrap()
    }

    pub fn themes_data(&self) -> Vec<ThemeData> {
        self.themes
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

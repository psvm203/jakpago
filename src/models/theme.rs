use serde::Deserialize;

mod constants {
    pub const THEME_DATA: &str = include_str!("../assets/data/theme.yaml");
}

#[derive(Clone, Eq, PartialEq)]
pub struct ThemeCollection {
    themes: Vec<Theme>,
}

impl ThemeCollection {
    pub fn new() -> Self {
        Self {
            themes: Self::load_themes(constants::THEME_DATA),
        }
    }

    fn load_themes(yaml_data: &'static str) -> Vec<Theme> {
        serde_yaml::from_str(yaml_data).unwrap()
    }

    pub fn themes(&self) -> &[Theme] {
        &self.themes
    }
}

#[derive(Clone, Deserialize, Eq, PartialEq)]
pub struct Theme {
    value: &'static str,
    name: &'static str,
}

impl Theme {
    pub const fn value(&self) -> &'static str {
        self.value
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }
}

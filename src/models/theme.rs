use serde::Deserialize;

mod constants {
    pub const THEME_DATA: &str = include_str!("../assets/data/theme.yaml");
}

#[derive(Clone, Eq, PartialEq)]
pub struct ThemeCollection {
    pub themes: Vec<Theme>,
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
}

#[derive(Clone, Deserialize, Eq, PartialEq)]
pub struct Theme {
    pub value: &'static str,
    pub name: &'static str,
}

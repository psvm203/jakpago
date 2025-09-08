use serde::Deserialize;

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

#[derive(Clone, Eq, PartialEq)]
pub struct ThemeCollection {
    pub themes: Vec<Theme>,
}

impl ThemeCollection {
    pub fn new() -> Self {
        Self {
            themes: vec![
                Theme {
                    value: "default",
                    name: "자동",
                },
                Theme {
                    value: "light",
                    name: "라이트",
                },
                Theme {
                    value: "dark",
                    name: "다크",
                },
                Theme {
                    value: "caramellatte",
                    name: "카라멜라떼",
                },
                Theme {
                    value: "valentine",
                    name: "발렌타인",
                },
                Theme {
                    value: "aqua",
                    name: "아쿠아",
                },
                Theme {
                    value: "synthwave",
                    name: "신스웨이브",
                },
            ],
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Theme {
    pub value: &'static str,
    pub name: &'static str,
}

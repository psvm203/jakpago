#[derive(Clone)]
pub struct ThemeCollection {
    pub themes: Vec<Theme>,
}

impl ThemeCollection {
    pub fn new() -> Self {
        Self {
            themes: vec![
                Theme {
                    value: "default",
                    label: "자동",
                },
                Theme {
                    value: "light",
                    label: "라이트",
                },
                Theme {
                    value: "dark",
                    label: "다크",
                },
                Theme {
                    value: "caramellatte",
                    label: "카라멜라떼",
                },
                Theme {
                    value: "valentine",
                    label: "발렌타인",
                },
                Theme {
                    value: "aqua",
                    label: "아쿠아",
                },
                Theme {
                    value: "synthwave",
                    label: "신스웨이브",
                },
            ],
        }
    }
}

#[derive(Clone)]
pub struct Theme {
    pub value: &'static str,
    pub label: &'static str,
}

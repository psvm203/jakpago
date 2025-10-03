pub struct Theme {
    pub value: &'static str,
    pub label: &'static str,
}

pub const THEMES: [Theme; 7] = [DEFAULT, LIGHT, DARK, CARAMELLATTE, VALENTINE, AQUA, SYNTHWAVE];

const DEFAULT: Theme = Theme {
    value: "default",
    label: "자동",
};

const LIGHT: Theme = Theme {
    value: "light",
    label: "라이트",
};

const DARK: Theme = Theme {
    value: "dark",
    label: "다크",
};

const CARAMELLATTE: Theme = Theme {
    value: "caramellatte",
    label: "카라멜라떼",
};

const VALENTINE: Theme = Theme {
    value: "valentine",
    label: "발렌타인",
};

const AQUA: Theme = Theme {
    value: "aqua",
    label: "아쿠아",
};

const SYNTHWAVE: Theme = Theme {
    value: "synthwave",
    label: "신스웨이브",
};

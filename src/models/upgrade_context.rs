use serde::{Deserialize, Serialize};

pub mod spec_collection {
    pub struct Spec {
        pub label: &'static str,
        pub placeholder: &'static str,
        pub min: u32,
        pub max: u32,
    }

    pub const HANDICRAFT: Spec = Spec {
        label: "손재주",
        placeholder: "0 ~ 100",
        min: 0,
        max: 100,
    };

    pub const ENHANCE_MASTERY: Spec = Spec {
        label: "강화의 달인",
        placeholder: "0 ~ 4",
        min: 0,
        max: 4,
    };

    pub const UPGRADE_SALVATION: Spec = Spec {
        label: "실패를 두려워 않는",
        placeholder: "0 ~ 4",
        min: 0,
        max: 4,
    };

    pub const EQUIPMENT_LEVEL: Spec = Spec {
        label: "장비 레벨",
        placeholder: "0 ~ 250",
        min: 0,
        max: 250,
    };

    pub const UPGRADEABLE_COUNT: Spec = Spec {
        label: "주문서 강화 가능 횟수",
        placeholder: "1 ~ 12",
        min: 1,
        max: 12,
    };

    pub const TRACE_REQUIRED: Spec = Spec {
        label: "주문의 흔적 필요 갯수 (썬데이 미적용 기준)",
        placeholder: "0",
        min: 0,
        max: 10000,
    };

    pub const TRACE_PRICE: Spec = Spec {
        label: "주문의 흔적 시세",
        placeholder: "0",
        min: 0,
        max: 10000,
    };
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct UpgradeContext {
    pub handicraft: Option<u32>,
    pub enhance_mastery: Option<u32>,
    pub upgrade_salvation: Option<u32>,
    pub equipment_level: Option<u32>,
    pub upgradeable_count: Option<u32>,
    pub trace_required: Option<u32>,
    pub trace_price: Option<u32>,
}

pub fn handicraft_tooltip(handicraft_level: u32) -> String {
    format!("성공 확률 {}%p 증가", f64::from(handicraft_level / 5 * 5) / 10.0)
}

pub fn enhance_mastery_tooltip(enhance_mastery_level: u32) -> String {
    format!("성공 확률 {enhance_mastery_level}%p 증가")
}

pub fn upgrade_salvation_tooltip(upgrade_salvation_level: u32) -> String {
    format!("실패 시 {upgrade_salvation_level}% 확률로 횟수 차감 방지")
}

pub fn trace_price_tooltip(trace_price: u32) -> String {
    format!("{trace_price} 메소")
}

use crate::utils::sycamore::{Callback, ViewVecExt};
use crate::view_models::upgrade_context_view_model::{
    Spec, UpgradeContextViewModel, spec_collection,
};
use sycamore::prelude::*;

mod constants {
    pub const POTENTIAL_LEGEND: &str = "확률 정보";
    pub const EQUIPMENT_LEGEND: &str = "장비 정보";
    pub const PRICE_LEGEND: &str = "시세 정보";
}

#[component]
pub fn UpgradeContextView() -> View {
    let view_model = UpgradeContextViewModel::new();
    provide_context(view_model);

    view! { Fieldsets() }
}

#[component]
fn Fieldsets() -> View {
    view! {
        div(class="grid grid-cols-6 gap-48 p-16") {
            (fieldset(constants::POTENTIAL_LEGEND, probability_fields()))
            (fieldset(constants::EQUIPMENT_LEGEND, equipment_fields()))
            (fieldset(constants::PRICE_LEGEND, price_fields()))
        }
    }
}

fn fieldset(legend: &'static str, fields: Vec<View>) -> View {
    view! {
        fieldset(class="fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4") {
            legend(class="fieldset-legend") { (legend) }
            (fields)
        }
    }
}

fn probability_fields() -> Vec<View> {
    let view_model = use_context::<UpgradeContextViewModel>();

    let handicraft = view_model.get_field(|context| context.handicraft);
    let enhance_mastery = view_model.get_field(|context| context.enhance_mastery);
    let upgrade_salvation = view_model.get_field(|context| context.upgrade_salvation);

    let handicraft_callback = view_model.handicraft_change_callback();
    let enhance_mastery_callback = view_model.enhance_mastery_change_callback();
    let upgrade_salvation_callback = view_model.upgrade_salvation_change_callback();

    let handicraft_tooltip = view_model.handicraft_tooltip();
    let enhance_mastery_tooltip = view_model.enhance_mastery_tooltip();
    let upgrade_salvation_tooltip = view_model.upgrade_salvation_tooltip();

    [
        CharacterSearch(),
        view! {
            (field(&spec_collection::HANDICRAFT, handicraft.clone(), handicraft_callback.clone()))
            (handicraft_tooltip)
        },
        view! {
            (field(&spec_collection::ENHANCE_MASTERY, enhance_mastery.clone(), enhance_mastery_callback.clone()))
            (enhance_mastery_tooltip)
        },
        view! {
            (field(&spec_collection::UPGRADE_SALVATION, upgrade_salvation.clone(), upgrade_salvation_callback.clone()))
            (upgrade_salvation_tooltip)
        },
    ]
    .into_iter()
    .collect::<Vec<View>>()
    .join(|| view! { div(class="divider") })
}

fn equipment_fields() -> Vec<View> {
    let view_model = use_context::<UpgradeContextViewModel>();

    let equipment_level = view_model.get_field(|context| context.equipment_level);
    let upgradeable_count = view_model.get_field(|context| context.upgradeable_count);
    let trace_required = view_model.get_field(|context| context.trace_required);

    let equipment_level_callback = view_model.equipment_level_change_callback();
    let upgradeable_count_callback = view_model.upgradeable_count_change_callback();
    let trace_required_callback = view_model.trace_required_change_callback();

    [
        field(&spec_collection::EQUIPMENT_LEVEL, equipment_level, equipment_level_callback),
        field(&spec_collection::UPGRADEABLE_COUNT, upgradeable_count, upgradeable_count_callback),
        field(&spec_collection::TRACE_REQUIRED, trace_required, trace_required_callback),
    ]
    .into_iter()
    .collect::<Vec<View>>()
    .join(|| view! { div(class="divider") })
}

fn price_fields() -> Vec<View> {
    let view_model = use_context::<UpgradeContextViewModel>();

    let trace_price = view_model.get_field(|context| context.trace_price);

    let trace_price_callback = view_model.trace_price_change_callback();

    let trace_price_tooltip = view_model.trace_price_tooltip();

    [view! {
        (field(&spec_collection::TRACE_PRICE, trace_price.clone(), trace_price_callback.clone()))
        (trace_price_tooltip)
    }]
    .into_iter()
    .collect::<Vec<View>>()
    .join(|| view! { div(class="divider") })
}

#[component]
fn CharacterSearch() -> View {
    let view_model = use_context::<UpgradeContextViewModel>();
    let onchange = view_model.character_search_callback();

    view! {
        label(class="input") {
            svg(class="h-[1em] opacity-50", xmlns="http://www.w3.org/2000/svg", viewBox="0 0 24 24") {
                g(stroke-linejoin="round",
                stroke-linecap="round",
                stroke-width="2.5",
                fill="none",
                stroke="currentColor") {
                    circle(cx="11", cy="11", r="8") {}
                    path(d="m21 21-4.3-4.3") {}
                }
            }
            input(r#type="search", class="grow", placeholder="캐릭터 닉네임 검색", on:change=onchange) {}
            kbd(class="kbd kbd-sm") { "↲" }
        }
    }
}

fn field(spec: &Spec, value: Option<String>, callback: Callback) -> View {
    let label = spec.label;
    let placeholder = spec.placeholder;
    let min = spec.min.to_string();
    let max = spec.max.to_string();

    view! {
        label(class="label", r#for=label) { (label) }
        input(
            r#type="number",
            id=label,
            class="input validator",
            required=true,
            placeholder=placeholder,
            value=value,
            min=min,
            max=max,
            on:change=callback
        ) {}
    }
}

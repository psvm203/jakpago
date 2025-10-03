pub use crate::models::upgrade_context::{
    UpgradeContext,
    spec_collection::{self, Spec},
};
use crate::{
    models::upgrade_context,
    utils::sycamore::{Callback, EventParser},
};
use sycamore::prelude::*;
use web_sys::Event;

#[derive(Clone)]
pub struct UpgradeContextViewModel {
    pub signal: Signal<UpgradeContext>,
}

impl UpgradeContextViewModel {
    pub fn new() -> Self {
        Self {
            signal: create_signal(UpgradeContext::default()),
        }
    }

    pub fn handicraft_change_callback(&self) -> Callback {
        let signal = self.signal.clone();

        Callback::from(move |event: Event| {
            if let Some(value) = event.parse()
                && spec_collection::HANDICRAFT.allows(value)
            {
                let mut upgrade_context = signal.get_clone_untracked();
                upgrade_context.handicraft = Some(value);
                signal.set(upgrade_context);
            }
        })
    }

    pub fn enhance_mastery_change_callback(&self) -> Callback {
        let signal = self.signal.clone();

        Callback::from(move |event: Event| {
            if let Some(value) = event.parse()
                && spec_collection::ENHANCE_MASTERY.allows(value)
            {
                let mut upgrade_context = signal.get_clone_untracked();
                upgrade_context.enhance_mastery = Some(value);
                signal.set(upgrade_context);
            }
        })
    }

    pub fn upgrade_salvation_change_callback(&self) -> Callback {
        let signal = self.signal.clone();

        Callback::from(move |event: Event| {
            if let Some(value) = event.parse()
                && spec_collection::UPGRADE_SALVATION.allows(value)
            {
                let mut upgrade_context = signal.get_clone_untracked();
                upgrade_context.upgrade_salvation = Some(value);
                signal.set(upgrade_context);
            }
        })
    }

    pub fn equipment_level_change_callback(&self) -> Callback {
        let signal = self.signal.clone();

        Callback::from(move |event: Event| {
            if let Some(value) = event.parse()
                && spec_collection::EQUIPMENT_LEVEL.allows(value)
            {
                let mut upgrade_context = signal.get_clone_untracked();
                upgrade_context.equipment_level = Some(value);
                signal.set(upgrade_context);
            }
        })
    }

    pub fn upgradeable_count_change_callback(&self) -> Callback {
        let signal = self.signal.clone();

        Callback::from(move |event: Event| {
            if let Some(value) = event.parse()
                && spec_collection::UPGRADEABLE_COUNT.allows(value)
            {
                let mut upgrade_context = signal.get_clone_untracked();
                upgrade_context.upgradeable_count = Some(value);
                signal.set(upgrade_context);
            }
        })
    }

    pub fn trace_required_change_callback(&self) -> Callback {
        let signal = self.signal.clone();

        Callback::from(move |event: Event| {
            if let Some(value) = event.parse()
                && spec_collection::TRACE_REQUIRED.allows(value)
            {
                let mut upgrade_context = signal.get_clone_untracked();
                upgrade_context.trace_required = Some(value);
                signal.set(upgrade_context);
            }
        })
    }

    pub fn trace_price_change_callback(&self) -> Callback {
        let signal = self.signal.clone();

        Callback::from(move |event: Event| {
            if let Some(value) = event.parse()
                && spec_collection::TRACE_PRICE.allows(value)
            {
                let mut upgrade_context = signal.get_clone_untracked();
                upgrade_context.trace_price = Some(value);
                signal.set(upgrade_context);
            }
        })
    }

    pub fn handicraft_tooltip(&self) -> String {
        let handicraft_level = self.signal.get_clone_untracked().handicraft.unwrap_or_default();
        upgrade_context::handicraft_tooltip(handicraft_level)
    }

    pub fn enhance_mastery_tooltip(&self) -> String {
        let enhance_mastery_level =
            self.signal.get_clone_untracked().enhance_mastery.unwrap_or_default();
        upgrade_context::enhance_mastery_tooltip(enhance_mastery_level)
    }

    pub fn upgrade_salvation_tooltip(&self) -> String {
        let upgrade_salvation_level =
            self.signal.get_clone_untracked().upgrade_salvation.unwrap_or_default();
        upgrade_context::upgrade_salvation_tooltip(upgrade_salvation_level)
    }

    pub fn trace_price_tooltip(&self) -> String {
        let trace_price_level = self.signal.get_clone_untracked().trace_price.unwrap_or_default();
        upgrade_context::trace_price_tooltip(trace_price_level)
    }
}

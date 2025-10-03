pub use crate::models::upgrade_context::{
    UpgradeContext,
    spec_collection::{self, Spec},
};
use crate::{
    models::upgrade_context,
    utils::sycamore::{Callback, EventParser},
};
use gloo_storage::{LocalStorage, Storage};
use sycamore::prelude::*;
use web_sys::Event;

mod constants {
    pub const UPGRADE_CONTEXT_STORAGE_KEY: &str = "upgrade_context";
}

#[derive(Clone)]
pub struct UpgradeContextViewModel {
    pub current_upgrade_context: Signal<UpgradeContext>,
}

impl UpgradeContextViewModel {
    pub fn new() -> Self {
        let stored_upgrade_context: UpgradeContext =
            LocalStorage::get(constants::UPGRADE_CONTEXT_STORAGE_KEY).unwrap_or_default();

        Self {
            current_upgrade_context: create_signal(stored_upgrade_context),
        }
    }

    pub fn get_field<F>(&self, field_getter: F) -> Option<String>
    where
        F: Fn(&UpgradeContext) -> Option<u32>,
    {
        let upgrade_context = self.current_upgrade_context.get_clone();
        field_getter(&upgrade_context).map(|x| x.to_string())
    }

    fn create_callback<F>(&self, spec: &Spec, field_setter: F) -> Callback
    where
        F: Fn(&mut UpgradeContext, Option<u32>) + 'static,
    {
        let current_upgrade_context = self.current_upgrade_context;
        let min = spec.min;
        let max = spec.max;

        Callback::from(move |event: Event| {
            if let Some(value) = event.parse()
                && (min..=max).contains(&value)
            {
                let mut upgrade_context = current_upgrade_context.get_clone_untracked();
                field_setter(&mut upgrade_context, Some(value));
                current_upgrade_context.set(upgrade_context.clone());
                LocalStorage::set(constants::UPGRADE_CONTEXT_STORAGE_KEY, upgrade_context).unwrap();
            }
        })
    }

    fn create_tooltip<F>(&self, field_getter: F, tooltip_fn: fn(u32) -> String) -> String
    where
        F: Fn(&UpgradeContext) -> Option<u32>,
    {
        let upgrade_context = self.current_upgrade_context.get_clone_untracked();
        let value = field_getter(&upgrade_context).unwrap_or_default();

        tooltip_fn(value)
    }

    pub fn handicraft_change_callback(&self) -> Callback {
        self.create_callback(&spec_collection::HANDICRAFT, |context, value| {
            context.handicraft = value;
        })
    }

    pub fn enhance_mastery_change_callback(&self) -> Callback {
        self.create_callback(&spec_collection::ENHANCE_MASTERY, |context, value| {
            context.enhance_mastery = value;
        })
    }

    pub fn upgrade_salvation_change_callback(&self) -> Callback {
        self.create_callback(&spec_collection::UPGRADE_SALVATION, |context, value| {
            context.upgrade_salvation = value;
        })
    }

    pub fn equipment_level_change_callback(&self) -> Callback {
        self.create_callback(&spec_collection::EQUIPMENT_LEVEL, |context, value| {
            context.equipment_level = value;
        })
    }

    pub fn upgradeable_count_change_callback(&self) -> Callback {
        self.create_callback(&spec_collection::UPGRADEABLE_COUNT, |context, value| {
            context.upgradeable_count = value;
        })
    }

    pub fn trace_required_change_callback(&self) -> Callback {
        self.create_callback(&spec_collection::TRACE_REQUIRED, |context, value| {
            context.trace_required = value;
        })
    }

    pub fn trace_price_change_callback(&self) -> Callback {
        self.create_callback(&spec_collection::TRACE_PRICE, |context, value| {
            context.trace_price = value;
        })
    }

    pub fn handicraft_tooltip(&self) -> String {
        self.create_tooltip(|context| context.handicraft, upgrade_context::handicraft_tooltip)
    }

    pub fn enhance_mastery_tooltip(&self) -> String {
        self.create_tooltip(
            |context| context.enhance_mastery,
            upgrade_context::enhance_mastery_tooltip,
        )
    }

    pub fn upgrade_salvation_tooltip(&self) -> String {
        self.create_tooltip(
            |context| context.upgrade_salvation,
            upgrade_context::upgrade_salvation_tooltip,
        )
    }

    pub fn trace_price_tooltip(&self) -> String {
        self.create_tooltip(|context| context.trace_price, upgrade_context::trace_price_tooltip)
    }
}

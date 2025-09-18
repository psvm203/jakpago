use crate::models::upgrade_context::*;
use yew::prelude::*;
use yew_hooks::prelude::*;

mod constants {
    pub const UPGRADE_CONTEXT_STORAGE_KEY: &str = "upgrade_context";
}

#[derive(Clone, PartialEq)]
pub struct UpgradeContextViewModel {
    context: UpgradeContext,
    hooks: UpgradeContextHooks,
}

impl UpgradeContextViewModel {
    pub fn new(hooks: UpgradeContextHooks) -> Self {
        Self {
            context: UpgradeContext::new(),
            hooks,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct UpgradeContextHooks {
    state: UseStateHandle<UpgradeContextValues>,
    storage: UseLocalStorageHandle<UpgradeContextValues>,
}

impl UpgradeContextHooks {
    #[hook]
    pub fn use_upgrade_context() -> UpgradeContextHooks {
        let state = use_state(UpgradeContextValues::default);
        let storage = use_local_storage(constants::UPGRADE_CONTEXT_STORAGE_KEY.to_owned());

        {
            let state = state.clone();
            let storage = storage.clone();

            use_effect_once(move || {
                if let Some(stored) = storage.as_ref().cloned() {
                    state.set(stored);
                }
                || {}
            });
        }

        UpgradeContextHooks {
            state,
            storage,
        }
    }
}

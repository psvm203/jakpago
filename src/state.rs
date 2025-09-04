use crate::{api, input};

use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};
use yew::{Callback, Html, KeyboardEvent, html};
use yew_hooks::{UseLocalStorageHandle, UseMapHandle};

mod constants {
    pub const KEY_ENTER: &str = "Enter";

    pub mod texts {
        pub const CHARACTER_SEARCH_PLACEHOLDER: &str = "캐릭터 검색";
    }

    pub mod skills {
        pub const ENHANCE_MASTERY: &str = "강화의 달인";
        pub const UPGRADE_SALVATION: &str = "실패를 두려워 않는";
    }
}

#[allow(clippy::wildcard_imports)]
use constants::*;

type FieldMap = HashMap<input::FieldId, u32>;

#[derive(Clone)]
pub struct State {
    map: UseMapHandle<input::FieldId, u32>,
    storage: UseLocalStorageHandle<FieldMap>,
}

impl State {
    pub const fn new(
        map: UseMapHandle<input::FieldId, u32>,
        storage: UseLocalStorageHandle<FieldMap>,
    ) -> Self {
        Self {
            map,
            storage,
        }
    }

    pub fn insert(&self, key: input::FieldId, value: u32) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: input::FieldId) -> Option<u32> {
        self.map.current().get(&key).copied()
    }

    pub fn get_or_default(&self, key: input::FieldId) -> u32 {
        self.map.current().get(&key).copied().unwrap_or_default()
    }

    pub fn filtered(&self, fields: &input::FieldRegistry) -> FieldMap {
        self.map
            .current()
            .iter()
            .filter(|&(&id, &value)| {
                fields.get(id).get_min() <= value && value <= fields.get(id).get_max()
            })
            .map(|(&id, &value)| (id, value))
            .collect()
    }

    pub fn save_to_storage(&self, value: FieldMap) {
        self.storage.set(value);
    }

    async fn fetch_character_data(state: Self, character_name: String) {
        let handicraft_level =
            api::get_handicraft_level_by_character_name(character_name.clone()).await;

        if let Ok(handicraft_level) = handicraft_level {
            state.insert(input::FieldId::Handicraft, handicraft_level);
        }

        let enhance_mastery_level = api::get_guild_skill_level_by_character_name(
            character_name.clone(),
            skills::ENHANCE_MASTERY,
        )
        .await;

        if let Ok(enhance_mastery_level) = enhance_mastery_level {
            state.insert(input::FieldId::EnhancementMastery, enhance_mastery_level);
        }

        let upgrade_salvation_level =
            api::get_guild_skill_level_by_character_name(character_name, skills::UPGRADE_SALVATION)
                .await;

        if let Ok(upgrade_salvation_level) = upgrade_salvation_level {
            state.insert(input::FieldId::UpgradeSalvation, upgrade_salvation_level);
        }
    }

    fn search_character(&self) -> Callback<KeyboardEvent> {
        let state = self.clone();

        Callback::from(move |event: KeyboardEvent| {
            if event.key() == KEY_ENTER {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                if let Some(character_name) = input {
                    let state = state.clone();
                    let name = character_name.value();

                    spawn_local(async move {
                        Self::fetch_character_data(state, name).await;
                    });
                }
            }
        })
    }

    pub fn character_search_item(&self) -> Html {
        let onkeydown = self.search_character();

        html! {
            <label class={"input"}>
                <svg
                    class={"h-[1em] opacity-50"}
                    xmlns={"http://www.w3.org/2000/svg"}
                    viewBox={"0 0 24 24"}
                >
                    <g
                        stroke-linejoin={"round"}
                        stroke-linecap={"round"}
                        stroke-width={"2.5"}
                        fill={"none"}
                        stroke={"currentColor"}
                    >
                        <circle cx={"11"} cy={"11"} r={"8"} />
                        <path d={"m21 21-4.3-4.3"} />
                    </g>
                </svg>
                <input
                    type={"search"}
                    class={"grow"}
                    placeholder={texts::CHARACTER_SEARCH_PLACEHOLDER}
                    {onkeydown}
                />
                <kbd class="kbd kbd-sm">
                    { "↵" }
                </kbd>
            </label>
        }
    }

    pub fn tooltip_item(&self, field_id: input::FieldId) -> Html {
        let value = self.get(field_id).unwrap_or(0);
        let tooltip = field_id.get_tooltip(value);

        html! {
            <div>
                { tooltip }
            </div>
        }
    }
}

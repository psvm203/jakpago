pub fn handicraft_effect(handicraft_level: u32) -> f64 {
    f64::from(handicraft_level / 5 * 5) / 10.0
}

pub const fn enhance_mastery_effect(enhance_mastery_level: u32) -> u32 {
    enhance_mastery_level
}

pub const fn upgrade_salvation_effect(upgrade_salvation_level: u32) -> u32 {
    upgrade_salvation_level
}

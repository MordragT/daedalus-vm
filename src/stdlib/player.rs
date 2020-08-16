#[derive(Default)]
pub struct Mission {
    // Name des Auftraggebers
    name: String,
    description: String,
    // Max. Dauer in Tageszeiten
    duration: i32,
    important: i32,

    offer_conditions: u32,
    offer: u32,
    success_conditions: u32,
    success: u32,
    failure_conditions: u32,
    failure: u32,
    obsolete_conditions: u32,
    obsolete: u32,
    running: u32,
}
impl Mission {
    pub fn new() -> Mission {
        Default::default()
    }
}

#[derive(Default)]
pub struct Spell {
    time_per_mana: f32,
    damage_per_level: i32,
    damage_type: i32,
    spell_type: i32,
    can_turn_during_invest: i32,
    can_change_target_during_invest: i32,
    is_multi_effect: i32,
    target_collect_algo: i32,
    target_collect_type: i32,
    target_collect_range: i32,
    target_collect_azi: i32,
    target_collect_elev: i32,
}

impl Spell {
    pub fn new() -> Spell {
        Default::default()
    }
}

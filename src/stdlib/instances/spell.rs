use super::Instance;
#[derive(Default)]
pub struct Spell {
    instance_symbol: usize,
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

impl Instance for Spell {
    fn get_instance_symbol(&self) -> usize {
        self.instance_symbol
    }
    fn set_instance_symbol(&mut self, instance_symbol: usize) {
        self.instance_symbol = instance_symbol;
    }
}

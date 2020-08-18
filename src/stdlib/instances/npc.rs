use enumflags2::BitFlags;

use super::*;

#[derive(Default)]
struct Attribute {
    hit_points: i32,
    hit_points_max: i32,
    mana: i32,
    mana_max: i32,
    strength: i32,
    dexterity: i32,
    regernate_hp: i32,
    regerenate_mana: i32,
}

#[derive(BitFlags, Copy, Clone)]
#[repr(u16)]
pub enum Flag {
    //Nil = 0,
    Friends = 1 << 0,
    Immortal = 1 << 1,
    Ghost = 1 << 2,
    Protected = 1 << 10,
}
#[derive(Default)]
pub struct Npc {
    instance_symbol: usize,
    id: i32,
    name: [String; 5],
    slot: String,
    effect: String,
    kind: i32,
    flags: Option<Flag>,
    attributes: Attribute,
    hit_chance: [i32; MAX_HITCHANCE],
    protection: [i32; PROT_INDEX_MAX],
    damage: [i32; DAM_INDEX_MAX],
    damage_type: i32,
    guild: i32,
    level: i32,
    mission: [u32; MAX_MISSIONS],
    fight_tactic: i32,
    weapon: i32,

    voice: i32,
    voice_pitch: i32,
    body_mass: i32,
    daily_routine: u32,
    start_ai_state: u32,

    spawn_point: String,
    spawn_delay: i32,

    senses: i32,
    senses_range: i32,

    ai_var: Vec<i32>,
    waypoint: String,

    exp: i32,
    exp_next: i32,
    lp: i32,

    // If this is set to TRUE, the Npc can't be interrupted in any action (e.g. BS_FLAG_INTERRUPTABLE for anis is being ignored)
    body_state_interruptable_override: i32,
    // if "noFocus" is set to TRUE, the focus name and health bar will not be drawn of this nsc (hi, stefan!)
    no_focus: i32,
}

impl Npc {
    pub fn new() -> Npc {
        Default::default()
    }
    pub fn set_waypoint(&mut self, waypoint: &str) {
        self.waypoint = waypoint.to_owned();
    }
}

impl Instance for Npc {
    fn get_instance_symbol(&self) -> usize {
        self.instance_symbol
    }
    fn set_instance_symbol(&mut self, instance_symbol: usize) {
        self.instance_symbol = instance_symbol;
    }
}

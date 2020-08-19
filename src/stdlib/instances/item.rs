use super::*;
use enumflags2::BitFlags;

#[derive(BitFlags, Copy, Clone)]
#[repr(u32)]
pub enum Categories {
    Nil = 1 << 0,
    /// Equipable
    Nf = 1 << 1,
    /// Equipable
    Ff = 1 << 2,
    Mun = 1 << 3,
    /// Equipable
    Armor = 1 << 4,
    Food = 1 << 5,
    Docs = 1 << 6,
    Potion = 1 << 7,
    Light = 1 << 8,
    /// Equipable
    Rune = 1 << 9,
    /// Equipable
    Magic = 1 << 31,
    //Equipable = Nf | Ff | Armor | Rune | Magic,
}

#[derive(BitFlags, Copy, Clone)]
#[repr(u32)]
pub enum Flags {
    Dagger = 1 << 13,
    Sword = 1 << 14,
    Axe = 1 << 15,
    TwoHandSword = 1 << 16,
    TwoHandAxe = 1 << 17,
    Bow = 1 << 19,
    CrossBow = 1 << 20,
    Amulet = 1 << 22,
    Ring = 1 << 11,
    Belt = 1 << 24,
    Mission = 1 << 12,
}

impl Default for Flags {
    fn default() -> Self {
        BitFlags::<Flags>::emtpy()
    }
}

#[derive(Default)]
pub struct Item {
    instance_symbol: usize,
    id: i32,
    name: String,
    name_id: String,
    hp: i32,
    hp_max: i32,
    main_flag: Flags,
    flags: Flags,
    weight: i32,
    value: i32,
    damage_type: i32,
    damage_total: i32,
    damage: [i32; DAM_INDEX_MAX],
    wear: i32,
    protection: [i32; PROT_INDEX_MAX],
    nutrition: i32,
    cond_atr: [i32; COND_ATR_MAX],
    cond_value: [i32; COND_ATR_MAX],

    magic: u32,
    on_equip: u32,
    on_unequip: u32,
    on_state: [u32; 4],

    owner: u32,
    owner_guild: i32,
    //	Zur Schau getragene Gilde durch Verkleidung
    disguise_guild: i32,

    visual: String,
    visual_change: String,
    effect: String,
    visual_skin: i32,

    sceme_name: String,
    material: i32,
    munition: i32,
    spell: i32,
    range: i32,
    mag_circle: i32,

    description: String,
    text: [String; ITM_TEXT_MAX],
    count: [i32; ITM_TEXT_MAX],

    inv_zbias: i32,
    inv_rot: (i32, i32, i32),
    inv_animate: i32,

    pub amount: u32,
}

impl Item {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Instance for Item {
    fn get_instance_symbol(&self) -> usize {
        self.instance_symbol
    }
    fn set_instance_symbol(&mut self, instance_symbol: usize) {
        self.instance_symbol = instance_symbol;
    }
}

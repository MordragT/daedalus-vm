use super::Instance;
#[derive(Default)]
pub struct Focus {
    instance_symbol: usize,
    // NSCs
    npc_long_range: f32,
    npc_range: (f32, f32),
    npc_azi: f32,         // Azimuth
    npc_elev: (f32, f32), // Elevation down, up
    npc_prio: i32,

    // Item
    item_range: (f32, f32),
    item_azi: f32,
    item_elev: (f32, f32),
    item_prio: i32,

    // Mobs
    mob_range: (f32, f32),
    mob_azi: f32,
    mob_elev: (f32, f32),
    mob_prio: i32,
}
impl Focus {
    pub fn new() -> Focus {
        Default::default()
    }
}

impl Instance for Focus {
    fn get_instance_symbol(&self) -> usize {
        self.instance_symbol
    }
    fn set_instance_symbol(&mut self, instance_symbol: usize) {
        self.instance_symbol = instance_symbol;
    }
}

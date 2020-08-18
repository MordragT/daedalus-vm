pub mod menu;
pub mod npc;
pub mod particle;
pub mod player;
pub mod shared;
pub mod sound;

pub enum GameType {
    Gothic1,
    Gothic2,
}

pub enum TransitionType {
    Nil,
    Groove,
    Fill,
    Break,
    Intro,
    End,
    EndAndIntro,
}
pub enum SubTransitionType {
    Immediate,
    Beat,
    Measure,
}

#[derive(Default)]
pub struct Focus {
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

#[derive(Default)]
pub struct SubChoice {
    text: String,
    func_sym: u32,
}

impl SubChoice {
    pub fn new() -> SubChoice {
        Default::default()
    }
}

#[derive(Default)]
pub struct Info {
    npc: Option<NonZeroI32>,
    nr: Option<NonZeroI32>,
    important: Option<NonZeroI32>,
    condition: Option<NonZeroU32>,
    information: Option<NonZeroU32>,
    description: String,
    trade: Option<NonZeroI32>,
    permanent: Option<NonZeroI32>,
    sub_choices: Vec<SubChoice>,
}

impl Info {
    pub fn new() -> Info {
        Default::default()
    }
    pub fn add_choice(&mut self, choice: SubChoice) {
        self.sub_choices.push(choice);
    }
    pub fn remove_choice(&mut self, index: usize) {
        self.sub_choices.remove(index);
    }
}

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

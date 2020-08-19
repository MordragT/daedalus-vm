pub mod instances;
pub mod prelude;

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

#[derive(Eq, PartialEq, Hash)]
pub enum InstanceClass {
    Npc,
    Mission,
    Info,
    Item,
    ItemReact,
    Focus,
    Menu,
    MenuItem,
    Sfx,
    Pfx,
    MusicTheme,
}

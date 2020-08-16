use enumflags2::BitFlags;

const MAX_USERSTRINGS: i8 = 10;
const MAX_ITEMS: usize = 150;
const MAX_EVENTS: i8 = 10;
const MAX_SEL_ACTIONS: i8 = 5;
const MAX_USERVARS: i8 = 4;
const SEL_EVENT_UNDEF: i8 = 0;

#[repr(u8)]
pub enum Event {
    Execute,
    Changed,
    Leave,
    Timer,
    Close,
    Init,
    Prev,
    Next,
}

#[repr(u8)]
pub enum Action {
    Undef,
    Back,
    StartMenu,
    StartItem,
    Close,
    ConCommands,
    PlaySound,
    ExecCommands,
}
#[derive(BitFlags, Copy, Clone)]
#[repr(u8)]
enum MenuFlags {
    Overtop = 1,
    Exclusive = 2,
    Noani = 4,
    DontScaleDim = 8,
    DontScalePos = 16,
    AlignCenter = 32,
    ShowInfo = 64,
}

#[derive(Default)]
pub struct Menu {
    back_pic: String,
    back_world: String,
    pos: (i32, i32),
    dim: (i32, i32),
    alpha: i32,
    music_theme: String,
    event_timer_millisec: i32,
    //items: [String; MAX_ITEMS],
    items: Vec<String>,
    flags: i32,
    default_out_game: i32,
    default_in_game: i32,
}

// impl Default for Menu {
//     fn default() -> Self {
//         Menu {
//             back_pic: String::new(),
//             back_world: String::new(),
//             pos: (0, 0),
//             dim: (0, 0),
//             alpha: 0,
//             music_theme: String::new(),
//             event_timer_millisec: 0,
//             items: [String::new(); MAX_ITEMS],
//             flags: 0,
//             default_out_game: 0,
//             default_in_game: 0,
//         }
//     }
// }

impl Menu {
    pub fn new() -> Menu {
        Default::default()
    }
}
#[derive(BitFlags, Copy, Clone)]
#[repr(u16)]
pub enum ItemFlags {
    Chromakeyed = 1,
    Transparent = 2,
    Selectable = 4,
    Moveable = 8,
    TxtCenter = 16,
    Disabled = 32,
    Fade = 64,
    EffectsNext = 128,
    OnlyOutGame = 256,
    OnlyInGame = 512,
    PerfOption = 1024,
    Multiline = 2048,
    NeedsApply = 4096,
    NeedsRestart = 8192,
    ExtendedMenu = 16384,
}

#[repr(u8)]
pub enum ItemKind {
    Undef,
    Text,
    Slider,
    Input,
    Cursor,
    ChoiceBox,
    Button,
    ListBox,
}
#[derive(Default)]
pub struct Item {
    font_name: String,
    text: Vec<String>,
    back_pic: String,
    alphaMode: String,
    alpha: i32,
    kind: i32,
    on_sel_action: Vec<i32>,
    on_sel_action_s: Vec<String>,
    on_chg_set_option: String,
    on_chg_set_option_section: String,

    on_event_action: Vec<i32>,
    pos: (i32, i32),
    dim: (i32, i32),
    size_start_scale: f32,
    flags: i32,
    open_delay_time: f32,
    open_duration: f32,
    user_float: Vec<f32>,
    user_string: Vec<String>,
    frame_size: (i32, i32),
    hide_if_option_section_set: String,
    hide_if_option_set: String,
    hide_on_value: i32,
}

impl Item {
    pub fn new() -> Item {
        Default::default()
    }
}

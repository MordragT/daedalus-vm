use super::Instance;
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
    instance_symbol: usize,
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

impl Menu {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Instance for Menu {
    fn get_instance_symbol(&self) -> usize {
        self.instance_symbol
    }
    fn set_instance_symbol(&mut self, instance_symbol: usize) {
        self.instance_symbol = instance_symbol;
    }
}

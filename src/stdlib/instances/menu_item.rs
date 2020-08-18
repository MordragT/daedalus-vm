use super::Instance;
use enumflags2::BitFlags;
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
pub struct MenuItem {
    instance_symbol: usize,
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

impl MenuItem {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Instance for MenuItem {
    fn get_instance_symbol(&self) -> usize {
        self.instance_symbol
    }
    fn set_instance_symbol(&mut self, instance_symbol: usize) {
        self.instance_symbol = instance_symbol;
    }
}

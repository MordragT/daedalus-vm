use std::num::NonZeroI32;

#[derive(Default)]
pub struct Sfx {
    file: String,
    pitch_off: Option<NonZeroI32>,
    pitch_var: Option<NonZeroI32>,
    vol: i32,
    loop_: Option<NonZeroI32>,
    loop_start_offset: Option<NonZeroI32>,
    loop_end_offseT: Option<NonZeroI32>,
    reverb_level: f32,
    pfx_name: String,
}
impl Sfx {
    pub fn new() -> Sfx {
        Default::default()
    }
}

#[derive(Default)]
pub struct MusicTheme {
    file: String,
    vol: f32,
    loop_: i32,
    reverb_mix: f32,
    reverb_time: f32,
    trans_type: i32,
    trans_sub_type: i32,
}

impl MusicTheme {
    pub fn new() -> MusicTheme {
        Default::default()
    }
}

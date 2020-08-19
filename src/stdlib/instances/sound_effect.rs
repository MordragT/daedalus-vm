use super::Instance;
use std::num::NonZeroI32;
#[derive(Default)]
pub struct SoundEffect {
    instance_symbol: usize,
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
impl SoundEffect {
    pub fn new() -> Self {
        Default::default()
    }
}
impl Instance for SoundEffect {
    fn get_instance_symbol(&self) -> usize {
        self.instance_symbol
    }
    fn set_instance_symbol(&mut self, instance_symbol: usize) {
        self.instance_symbol = instance_symbol;
    }
}

use super::Instance;
#[derive(Default)]
pub struct MusicTheme {
    instance_symbol: usize,
    file: String,
    vol: f32,
    loop_: i32,
    reverb_mix: f32,
    reverb_time: f32,
    trans_type: i32,
    trans_sub_type: i32,
}

impl MusicTheme {
    pub fn new() -> Self {
        Default::default()
    }
}
impl Instance for MusicTheme {
    fn get_instance_symbol(&self) -> usize {
        self.instance_symbol
    }
    fn set_instance_symbol(&mut self, instance_symbol: usize) {
        self.instance_symbol = instance_symbol;
    }
}

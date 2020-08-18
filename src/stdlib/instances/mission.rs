use super::Instance;
#[derive(Default)]
pub struct Mission {
    instance_symbol: usize,
    // Name des Auftraggebers
    name: String,
    description: String,
    // Max. Dauer in Tageszeiten
    duration: i32,
    important: i32,

    offer_conditions: u32,
    offer: u32,
    success_conditions: u32,
    success: u32,
    failure_conditions: u32,
    failure: u32,
    obsolete_conditions: u32,
    obsolete: u32,
    running: u32,
}
impl Mission {
    pub fn new() -> Mission {
        Default::default()
    }
}
impl Instance for Mission {
    fn get_instance_symbol(&self) -> usize {
        self.instance_symbol
    }
    fn set_instance_symbol(&mut self, instance_symbol: usize) {
        self.instance_symbol = instance_symbol;
    }
}

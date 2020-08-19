use super::Instance;
#[derive(Default)]
pub struct ItemReact {
    instance_symbol: usize,
    npc: i32,
    trade_item: i32,
    trade_amount: i32,
    requested_cat: i32,
    requested_item: i32,
    requested_amount: i32,
    reaction: u32,
}

impl ItemReact {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Instance for ItemReact {
    fn get_instance_symbol(&self) -> usize {
        self.instance_symbol
    }
    fn set_instance_symbol(&mut self, instance_symbol: usize) {
        self.instance_symbol = instance_symbol;
    }
}

use super::Instance;
use std::num::{NonZeroI32, NonZeroU32};
#[derive(Default)]
pub struct SubChoice {
    text: String,
    func_sym: u32,
}

impl SubChoice {
    pub fn new() -> SubChoice {
        Default::default()
    }
}

#[derive(Default)]
pub struct Info {
    instance_symbol: usize,
    npc: Option<NonZeroI32>,
    nr: Option<NonZeroI32>,
    important: Option<NonZeroI32>,
    condition: Option<NonZeroU32>,
    information: Option<NonZeroU32>,
    description: String,
    trade: Option<NonZeroI32>,
    permanent: Option<NonZeroI32>,
    sub_choices: Vec<SubChoice>,
}

impl Info {
    pub fn new() -> Info {
        Default::default()
    }
    pub fn add_choice(&mut self, choice: SubChoice) {
        self.sub_choices.push(choice);
    }
    pub fn remove_choice(&mut self, index: usize) {
        self.sub_choices.remove(index);
    }
}

impl Instance for Info {
    fn get_instance_symbol(&self) -> usize {
        self.instance_symbol
    }
    fn set_instance_symbol(&mut self, instance_symbol: usize) {
        self.instance_symbol = instance_symbol;
    }
}

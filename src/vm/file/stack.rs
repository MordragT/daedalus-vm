use super::Operator;
use std::num::{NonZeroI32, NonZeroU8};

pub struct StackOpCode {
    operator: Operator,
    address: Option<NonZeroI32>,
    symbol: Option<NonZeroI32>,
    value: Option<NonZeroI32>,
    index: Option<NonZeroU8>,
    operator_size: usize,
}
impl StackOpCode {
    pub fn new(operator: Operator, operator_size: usize) -> StackOpCode {
        StackOpCode {
            operator,
            address: None,
            symbol: None,
            value: None,
            index: None,
            operator_size,
        }
    }
    pub fn with_address(&mut self, address: i32) -> &mut Self {
        self.address = NonZeroI32::new(address);
        self
    }
    pub fn with_symbol(&mut self, symbol: i32) -> &mut Self {
        self.symbol = NonZeroI32::new(symbol);
        self
    }
    pub fn with_value(&mut self, value: i32) -> &mut Self {
        self.value = NonZeroI32::new(value);
        self
    }
    pub fn with_index(&mut self, index: u8) -> &mut Self {
        self.index = NonZeroU8::new(index);
        self
    }
    pub fn get_operator(&self) -> Operator {
        self.operator
    }
}
#[derive(Copy, Clone, Default)]
pub struct Stack {
    offset: usize,
    size: usize,
}

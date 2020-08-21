use super::VirtualMachine;
pub enum CallStackFrame {
    Address(usize),
    SymbolIndex(usize),
}
impl CallStackFrame {
    pub fn insert_in_vm<'a>(self, virtual_machine: &'a mut VirtualMachine) -> &'a mut Self {
        virtual_machine.call_stack.push(self);
        virtual_machine.call_stack.last_mut().unwrap()
    }
    pub fn get(&self) -> usize {
        match self {
            CallStackFrame::Address(val) => *val,
            CallStackFrame::SymbolIndex(val) => *val,
        }
    }
}
// struct CallStackFrame {
//     address_type: AddressType,
// }
// impl CallStackFrame {
//     pub fn new(virtual_machine: &VirtualMachine, address_type: AddressType) -> CallStackFrame {
//         let call_stack = CallStackFrame { address_type };
//         virtual_machine.call_stack.insert(address_type.get(), address_type);
//         call_stack
//     }
// }

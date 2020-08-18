use file::symbol::{Data, Kind, Symbol, SymbolBuilder};
use file::{File, Operator, StackOpCode};
use crate::game_state::GameState;
use std::collections::HashMap;
use zen_memory::Handle;

mod file;

const NUM_FAKE_STRING_SYMBOLS: u8 = 5;
enum AddressType {
    Address(usize),
    SymbolIndex(usize),
}
struct CallStackFrame {
    address_type: AddressType,
}
impl CallStackFrame {
    pub fn new(address_type: AddressType) -> CallStackFrame {
        CallStackFrame { address_type }
    }
}
struct StackValue(u32);
impl StackValue {
    pub fn get_operator(&self) -> Operator {
        self.0 as Operator
    }
    pub fn get(&self) -> u32 {
        self.0
    }
}

impl From<Operator> for StackValue {
    fn from(operator: Operator) -> Self {
        Self(operator as u32)
    }
}
struct VirtualMachineState {
    current_instance_handle: Handle,
    current_instance_class: InstanceClass,
    program_counter: usize,
    stack: Vec<StackValue>,
    call_stack: Vec<usize>,
    symbol: Symbol,
}
#[derive(Default)]
pub struct VirtualMachine {
    file: File,
    program_counter: usize,
    stack: Vec<StackValue>,
    call_stack: Vec<AddressType>,
    externals_by_index: HashMap<usize, &dyn Fn(&VirtualMachine)>,
    current_instance: usize,
    current_instance_handle: Handle,
    current_instance_class: InstanceClass,
    registered_instances: HashMap<InstanceClass, Vec<usize>>,
    game_state: GameState,
    state_stack: Vec<VirtualMachineState>,
    fake_string_symbols: Vec<usize>,
}

impl VirtualMachine {
    pub fn new(file: String) -> VirtualMachine {
        let file = File::open(file).unwrap();
        let vm = VirtualMachine {
            file,
            ..Default::default()
        };
        for index in 0..NUM_FAKE_STRING_SYMBOLS {
            let builder = SymbolBuilder::new("")
                .with_properties(Default::default())
                .set_kind(Kind::CharString);
            let index = file.sym_table.push(builder.build());
            vm.fake_string_symbols.push(index);
        }
        vm.current_instance_handle.invalidate();
    }

    pub fn get_current_instruction(&self) -> StackOpCode {}
    pub fn prepare_run_func(&self) {}
    pub fn run_func_by_sym_index(&self, sym_index: usize) -> i32 {}
    pub fn set_program_counter(&self, target: u32) {}
    //pub fn register_external_func(&self, sym_name: &str, func: &dyn Fn(&VirtualMachine)) {}

    pub fn push<T: Into<u32>>(&mut self, value: T) {
        self.stack.push(value);
        self.stack.push(StackValue::from(Operator::PushInt));
    }
    pub fn push_var(&mut self, index: usize, array_index: u32) {
        self.stack.push(array_index);
        self.stack.push(index as u32);
        self.stack.push(StackValue::from(Operator::PushVar));
    }
    pub fn push_var_by_name(&mut self, sym_name: &str) {
        let index = self.file.sym_table.get_symbol_index_by_name(sym_name).unwrap();
        self.push_var(index, 0);
    }
    pub fn push_state(&self) {}


    pub fn set_return<T>(&self, v: T) {}
    pub fn pop_state(&self) {}
    /// Returns value
    pub fn pop<T: From<u32>>(&mut self) -> T {
        if self.stack.is_empty() {
            return T::from(0);
        }
        let token = self.stack.pop().unwrap().get_operator();
        let value = self.stack.pop().unwrap().get();
        match token {
            Operator::PushInt => T::from(value),
            Operator::PushVar => {
                let index = self.stack.pop().unwrap().get();
                T::from(self.file.sym_table.get_symbol_by_index(value).unwrap().nth(index))
            }
            _ => T::from(0)
        }
    }
    /// Returns (value, array_index)
    pub fn pop_var(&mut self) -> (u32, u32) {
        let mut index = 0;
        if self.stack.is_empty() {
            return (0, index);
        }
        let token = self.stack.pop().unwrap().get_operator();
        let value = self.stack.pop().unwrap().get();
        match token {
            Operator::PushInt => {
                self.stack.pop();
                (value, index)
            }
            Operator::PushVar => {
                self.stack.pop();
                index = self.stack.pop().unwrap().get();
                self.stack.pop();
                (value, index)
            }
            _ => (0xffffffff, 0)
        }
    }

    pub fn set_instance(
        &self,
        inst_symbol: &str,
        handle: Handle,
        instance_class: InstanceClass,
    ) {
    }
    pub fn set_current_instance(&self, sym_index: usize) {}
    pub fn initialise_instance(
        &self,
        handle: Handle,
        sym_index: usize,
        instance_class: InstanceClass,
    ) {
    }
    pub fn get_registered_instances_of(&self, instance_class: InstanceClass) -> Vec<usize> {}
    pub fn get_current_instance_data(&self) -> Box<Data> {}
    pub fn get_current_instance_class(&self) -> InstanceClass {}
    pub fn get_current_instance_handle(&self) -> Handle {}

    pub fn get_file(&self) -> File {}
    pub fn get_game_state(&self) -> GameState {}

    pub fn is_stack_empty(&self) -> bool {}
    pub fn get_call_stack(&self) -> Box<Vec<String>> {}
    pub fn clear_call_stack(&self) {}
    pub fn do_stack(&self) -> bool {
        let old_program_counter = self.program_counter;
        let operator = self.get_current_instruction();

        match operator.get_operator() {
            Operator::Add => self.push::<i32>(self.pop::<i32>() + self.pop::<i32>()),
            Operator::Subract => self.push::<i32>(self.pop::<i32>() - self.pop::<i32>()),
            Operator::Multiply => self.push::<i32>(self.pop::<i32>() * self.pop::<i32>()),
            Operator::Divide => self.push::<i32>(self.pop::<i32>() / self.pop::<i32>()),
            Operator::Mod => self.push::<i32>(self.pop::<i32>() % self.pop::<i32>()),
            Operator::BinOr => self.push::<i32>(self.pop::<i32>() | self.pop::<i32>()),
            Operator::BinAnd => self.push::<i32>(self.pop::<i32>() & self.pop::<i32>()),
            Operator::Less => self.push::<i32>(self.pop::<i32>() < self.pop::<i32>() as i32),
            Operator::Greater => self.push::<i32>(self.pop::<i32>() > self.pop::<i32>() as i32),
            Operator::AssignFunc | Operator::Assign => {
                let (prev_addr, next_addr) = (self.pop_var().0, self.pop::<i32>());
                self.file
                    .sym_table
                    .get_symbol_by_index(prev_addr)
                    .unwrap()
                    .set_address(next_addr);
            }
            Operator::LogOr => {
                let val = (self.pop::<i32>(), self.pop::<i32>());
                self.push::<i32>(val.0 || val.1 as i32);
            }
            Operator::LogAnd => {
                let val = (self.pop::<i32>(), self.pop::<i32>());
                self.push::<i32>(val.0 && val.1 as i32);
            }
            Operator::ShiftLeft => self.push::<i32>(self.pop::<i32>() << self.pop::<i32>()),
            Operator::ShiftRight => self.push::<i32>(self.pop::<i32>() << self.pop::<i32>()),
            Operator::LessOrEqual => self.push::<i32>(self.pop::<i32>() <= self.pop::<i32>()),
            Operator::Equal => self.push::<i32>(self.pop::<i32>() == self.pop::<i32>()),
            Operator::NotEqual => self.push::<i32>(self.pop::<i32>() != self.pop::<i32>()),
            Operator::GreaterOrEqual => self.push::<i32>(self.pop::<i32>() >= self.pop::<i32>()),
            Operator::AssignAdd => {
                let 
            }
        }
    }

    //pub fn set_on_symbol_value_changed_callback(&self, func: &dyn Fn(u32, Operator)) {}
    pub fn set_on_external_called_callback(&self, func: Fn(u32)) {}
}

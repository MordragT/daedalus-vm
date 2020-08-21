use crate::game_state::{GameExternals, GameState};
use crate::stdlib::InstanceClass;
use call_stack_frame::CallStackFrame;
use file::file::File;
use file::stack::StackOpCode;
use file::symbol::{Data, Symbol, SymbolBuilder};
use file::{Flag, Kind, Operator};
use std::collections::{HashMap, VecDeque};
use std::convert::TryInto;
use zen_memory::Handle;

mod call_stack_frame;
mod external_funcs;
mod file;

const NUM_FAKE_STRING_SYMBOLS: u8 = 5;
struct StackValue(u32);
impl StackValue {
    pub fn get_operator(&self) -> Result<Operator, ()> {
        let num = self.0 as u8;
        num.try_into()
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
impl From<u32> for StackValue {
    fn from(value: u32) -> Self {
        Self(value)
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
pub struct VirtualMachine<'a> {
    file: File,
    program_counter: usize,
    stack: Vec<StackValue>,
    call_stack: Vec<CallStackFrame>,
    externals_by_index: HashMap<usize, &'a dyn Fn(&VirtualMachine)>,
    current_instance: usize,
    current_instance_handle: Handle,
    current_instance_class: Option<InstanceClass>,
    registered_instances: HashMap<InstanceClass, Vec<usize>>,
    game_state: GameState<'a>,
    state_stack: Vec<VirtualMachineState>,
    fake_string_symbols: VecDeque<usize>,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(file: String) -> VirtualMachine<'a> {
        let file = File::open(file).unwrap();
        let virtual_machine = VirtualMachine {
            file,
            program_counter: 0,
            stack: vec![],
            call_stack: vec![],
            externals_by_index: HashMap::new(),
            current_instance: 0,
            current_instance_handle: Handle::new(),
            current_instance_class: None,
            registered_instances: HashMap::new(),
            game_state: GameState::new(GameExternals::new()),
            state_stack: vec![],
            fake_string_symbols: VecDeque::new(),
        };
        // Register functions
        virtual_machine
            .register_external_func("insert_item", |virtual_machine: &VirtualMachine| {});

        for index in 0..NUM_FAKE_STRING_SYMBOLS {
            let builder = SymbolBuilder::new("")
                .with_properties(Default::default())
                .set_kind(Kind::CharString);
            let index = file.sym_table.push(builder.build());
            virtual_machine.fake_string_symbols.push(index);
        }
        virtual_machine.current_instance_handle.invalidate();
    }

    pub fn get_current_instruction(&mut self) -> StackOpCode {
        let operator = self.file.get_stack_op_code(self.program_counter);
        self.program_counter += operator.get_operator_size();
        operator
    }
    //pub fn prepare_run_func(&self) {}
    pub fn run_func_by_sym_index(
        &mut self,
        sym_index: usize,
        clear_data_stack: bool,
    ) -> Option<u32> {
        if clear_data_stack {
            self.stack = vec![];
        }
        CallStackFrame::SymbolIndex(sym_index).insert_in_vm(self);
        let func_sym = self.file.sym_table.get_symbol_by_index(sym_index).unwrap();
        let address = match func_sym.get_address() {
            Some(val) => val.get(),
            None => return None,
        };
        self.set_program_counter(address);
        while self.do_stack() {}
        let result = match func_sym.properties.has_flag(Flag::Return) && !self.stack.is_empty() {
            true => self.pop_int().unwrap(),
            false => 0,
        };
        self.pop_state();
        Some(result)
    }
    pub fn set_program_counter(&mut self, target: u32) {
        self.program_counter = target as usize;
    }
    pub fn register_external_func(&mut self, sym_name: &str, func: &'a dyn Fn(&VirtualMachine)) {
        match self.file.sym_table.get_symbol_index_by_name(sym_name) {
            Some(index) => {
                self.externals_by_index.insert(index, func);
            }
            None => (),
        }
    }

    pub fn push_int(&mut self, value: u32) {
        self.stack.push(StackValue::from(value));
        self.stack.push(StackValue::from(Operator::PushInt));
    }
    pub fn push_string(&mut self, string: String) {
        let sym_index = self.fake_string_symbols.front().unwrap();
        let symbol = self.file.sym_table.get_symbol_by_index(*sym_index).unwrap();
        self.fake_string_symbols
            .push_back(self.fake_string_symbols.pop_front().unwrap());
        let data_string = symbol.get_mut_string().unwrap();
        data_string.clear();
        data_string.push_str(string.as_str());
        self.push_var(*sym_index, 0);
    }
    pub fn push_var(&mut self, index: usize, array_index: u32) {
        self.stack.push(StackValue::from(array_index));
        self.stack.push(StackValue::from(index as u32));
        self.stack.push(StackValue::from(Operator::PushVar));
    }
    pub fn push_var_by_name(&mut self, sym_name: &str) {
        let index = self
            .file
            .sym_table
            .get_symbol_index_by_name(sym_name)
            .unwrap();
        self.push_var(index, 0);
    }
    pub fn push_state(&self) {}

    pub fn set_return<T>(&self, v: T) {}
    pub fn pop_state(&self) {}
    /// Returns value
    pub fn pop_int(&mut self) -> Option<u32> {
        if self.stack.is_empty() {
            return None;
        }
        let token = self.stack.pop().unwrap().get_operator().unwrap();
        let value = self.stack.pop().unwrap().get();
        match token {
            Operator::PushInt => Some(value),
            Operator::PushVar => {
                let index = self.stack.pop().unwrap().get();

                match self
                    .file
                    .sym_table
                    .get_symbol_by_index(value as usize)
                    .unwrap()
                    .get_data()
                    .unwrap()
                {
                    Data::IntSequence(vec) => match vec.get(index as usize) {
                        Some(val) => Some(*val),
                        None => None,
                    },
                    _ => None,
                }
            }
            _ => None,
        }
    }
    pub fn pop_float(&mut self) -> Option<f32> {
        if self.stack.is_empty() {
            return None;
        }
        let token = self.stack.pop().unwrap().get_operator().unwrap();
        let value = self.stack.pop().unwrap().get();
        match token {
            Operator::PushInt => Some(value as f32),
            Operator::PushVar => {
                let index = self.stack.pop().unwrap().get();

                match self
                    .file
                    .sym_table
                    .get_symbol_by_index(value as usize)
                    .unwrap()
                    .get_data()
                    .unwrap()
                {
                    Data::FloatSequence(vec) => match vec.get(index as usize) {
                        Some(val) => Some(*val),
                        None => None,
                    },
                    _ => None,
                }
            }
            _ => None,
        }
    }
    pub fn pop_string(&mut self) -> Option<String> {
        let (value, index) = self.pop_var();
        match self
            .file
            .sym_table
            .get_symbol_by_index(index as usize)
            .unwrap()
            .get_data()
            .unwrap()
        {
            Data::String(string) => Some(*string),
            _ => None,
        }
    }
    /// Returns (value, array_index)
    pub fn pop_var(&mut self) -> (u32, u32) {
        let mut index = 0;
        if self.stack.is_empty() {
            return (0, index);
        }
        let token = self.stack.pop().unwrap().get_operator().unwrap();
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
            _ => (0xffffffff, 0),
        }
    }

    pub fn set_instance(&self, inst_symbol: &str, handle: Handle, instance_class: InstanceClass) {}
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

    pub fn get_file(&self) -> File {
        self.file
    }
    pub fn get_game_state(&self) -> GameState {
        self.game_state
    }

    pub fn is_stack_empty(&self) -> bool {}
    pub fn get_call_stack(&self) -> Vec<String> {
        self.call_stack
    }
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
            Operator::AssignAdd => {}
            _ => (),
        }
    }

    //pub fn set_on_symbol_value_changed_callback(&self, func: &dyn Fn(u32, Operator)) {}
    pub fn set_on_external_called_callback(&self, func: Fn(u32)) {}
}

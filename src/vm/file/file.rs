use super::stack::{Stack, StackOpCode};
use super::sym_table::SymTable;
use super::symbol::{Data, Properties, SymbolBuilder};
use super::{Flag, Kind, Operator};
use std::mem;
use zen_parser::ZenParser;
pub struct File {
    parser: ZenParser,
    pub sym_table: SymTable,
    sort_table: Vec<u32>,
    // offset, size
    stack: Stack,
}

impl File {
    pub fn open(file: String) -> Result<File, String> {
        let parser = ZenParser::new(file);
        let version = parser.read_binary::<u8>().unwrap();
        println!("Version: {}", version);
        println!("Reading Sym Table...");
        let count = parser.read_binary::<u32>().unwrap();
        let mut sym_table = SymTable::with_capacity(count as usize);
        let sort_table = parser.read_binary_as_vec::<u32>(count as usize).unwrap();
        for index in 0..count {
            let name = match parser.read_binary::<u32>() {
                Ok(_) => {
                    let mut inner = String::new();
                    while let Ok(ch) = parser.read_binary::<char>() {
                        if ch == 0x0a as char {
                            break;
                        }
                        if ch != 0xff as char {
                            // FIXME: if Bedinung eigentlich nicht notwendig
                            inner.push(ch);
                        }
                    }
                    inner
                }
                Err(_) => "".to_owned(),
            };
            let mut symbol_builder = SymbolBuilder::new(name.as_str());

            let properties = Properties::new(
                parser.read_binary::<i32>().unwrap(),
                parser.read_binary::<u32>().unwrap(),
                // (Value, Reserved)
                parser.read_binary::<u32>().unwrap(),
                parser.read_binary::<u32>().unwrap(),
                parser.read_binary::<u32>().unwrap(),
                parser.read_binary::<u32>().unwrap(),
                parser.read_binary::<u32>().unwrap(),
            );

            if properties.is_not_flag(Flag::ClassVar) {
                match properties.get_kind() {
                    Kind::Float => {
                        symbol_builder.with_data(
                            parser
                                .read_binary_as_vec::<Data>(
                                    mem::size_of::<f32>() * properties.get_count() as usize,
                                )
                                .unwrap(),
                        );
                    }
                    Kind::Int => {
                        symbol_builder.with_data(
                            parser
                                .read_binary_as_vec::<Data>(
                                    mem::size_of::<u32>() * properties.get_count() as usize,
                                )
                                .unwrap(),
                        );
                    }
                    Kind::CharString => {
                        let mut inner = vec![];
                        for _ in 0..properties.get_count() {
                            while let Ok(ch) = parser.read_binary::<char>() {
                                if ch == 0x0a as char {
                                    break;
                                }
                                if ch != 0xff as char {
                                    inner.push(Data::Char(ch));
                                }
                            }
                            // TODO Replace \\n with \n
                        }
                        symbol_builder.with_data(inner);
                    }
                    Kind::Class => {
                        symbol_builder.with_class_offset(parser.read_binary::<i32>().unwrap());
                    }
                    Kind::Instance | Kind::Func | Kind::Prototype => {
                        symbol_builder.with_address(parser.read_binary::<u32>().unwrap());
                    }
                    _ => (),
                };
            }
            symbol_builder
                .with_properties(properties)
                .with_parent(parser.read_binary::<u32>().unwrap());

            sym_table.insert_symbol(index as usize, symbol_builder.build().unwrap());
        }
        let size = parser.read_binary::<i32>().unwrap() as usize;
        let offset = parser.get_seek();

        let file = File {
            parser,
            sym_table,
            sort_table,
            stack: Stack { offset, size },
        };

        println!("Reading Stack...");

        while file.parser.get_seek() < file.parser.get_file_size() {
            file.get_stack_op_code(file.parser.get_seek());
        }

        Ok(file)
    }
    pub fn get_stack(&self) -> Stack {
        self.stack
    }
    pub fn get_stack_op_code(&self, proc_counter: usize) -> StackOpCode {
        self.parser.set_seek(proc_counter);
        let operator = self.parser.read_binary::<Operator>().unwrap();
        //let operator: Operator = unsafe { mem::transmute(operator_num) };
        let stack_op_code = match operator {
            Operator::Call => {
                let mut op_code =
                    StackOpCode::new(operator, mem::size_of::<u8>() + mem::size_of::<i32>());
                op_code.with_address(self.parser.read_binary::<i32>().unwrap());
                op_code
            }
            Operator::CallExternal => {
                let mut op_code =
                    StackOpCode::new(operator, mem::size_of::<u8>() + mem::size_of::<i32>());
                op_code.with_symbol(self.parser.read_binary::<i32>().unwrap());
                op_code
            }
            Operator::PushInt => {
                let mut op_code =
                    StackOpCode::new(operator, mem::size_of::<u8>() + mem::size_of::<i32>());
                op_code.with_value(self.parser.read_binary::<i32>().unwrap());
                op_code
            }
            Operator::PushVar => {
                let mut op_code =
                    StackOpCode::new(operator, mem::size_of::<u8>() + mem::size_of::<i32>());
                op_code.with_symbol(self.parser.read_binary::<i32>().unwrap());
                op_code
            }
            Operator::PushInstance => {
                let mut op_code =
                    StackOpCode::new(operator, mem::size_of::<u8>() + mem::size_of::<i32>());
                op_code.with_symbol(self.parser.read_binary::<i32>().unwrap());
                op_code
            }
            Operator::Jump => {
                let mut op_code = StackOpCode::new(operator, mem::size_of::<i32>());
                op_code.with_address(self.parser.read_binary::<i32>().unwrap());
                op_code
            }
            Operator::JumpIf => {
                let mut op_code =
                    StackOpCode::new(operator, mem::size_of::<u8>() + mem::size_of::<i32>());
                op_code.with_address(self.parser.read_binary::<i32>().unwrap());
                op_code
            }
            Operator::SetInstance => {
                let mut op_code =
                    StackOpCode::new(operator, mem::size_of::<u8>() + mem::size_of::<i32>());
                op_code.with_symbol(self.parser.read_binary::<i32>().unwrap());
                op_code
            }
            Operator::PushArrayVar => {
                let mut op_code =
                    StackOpCode::new(operator, mem::size_of::<u8>() + mem::size_of::<i32>());
                op_code
                    .with_symbol(self.parser.read_binary::<i32>().unwrap())
                    .with_index(self.parser.read_binary::<u8>().unwrap());
                op_code
            }
            _ => {
                let op_code = StackOpCode::new(operator, mem::size_of::<u8>());
                op_code
            }
        };
        stack_op_code
    }
    // pub fn add_symbol(&mut self) -> usize {
    //     let builder = SymbolBuilder::new("").with_properties(Default::default());
    //     let symbol = builder.build().unwrap();
    //     self.sym_table.push(symbol)
    // }
}

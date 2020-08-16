use std::mem;
use symbol::{Data, Flag, Kind, Operator, Properties, SymTable, SymbolBuilder};
use zen_parser::ZenParser;

mod symbol;

pub struct StackOpCode {
    operator: Operator,
    address: i32,
    symbol: i32,
    value: i32,
    index: u8,
    operator_size: usize,
}
#[derive(Copy, Clone)]
pub struct Stack {
    offset: usize,
    size: usize,
}

pub struct File {
    parser: ZenParser,
    pub sym_table: SymTable,
    // offset, size
    stack: Stack,
}

impl File {
    pub fn new() -> Result<File, String> {
        let parser = ZenParser::new();
        let version = parser.read_binary::<u8>().unwrap();
        let count = parser.read_binary::<u32>().unwrap();
        let mut sym_table = SymTable::with_capacity(count as usize);
        let sort_table = parser.read_binary_as_vec::<u32>(count as usize);
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
        Ok(File {
            parser,
            sym_table,
            stack: Stack {
                offset: 0x0,
                size: 0x0,
            },
        })
    }
    pub fn get_stack(&self) -> Stack {
        self.stack
    }
    pub fn get_stack_op_code(&self, proc_counter: usize) -> StackOpCode {
        self.parser.set_seek(proc_counter);
        let operator = self.parser.read_binary::<Operator>().unwrap();
        //let operator: Operator = unsafe { mem::transmute(operator_num) };
        let stack_op_code = match operator {
            Operator::Call => StackOpCode {
                operator,
                address: self.parser.read_binary::<i32>().unwrap(),
                symbol: 0,
                value: 0,
                index: 0,
                operator_size: mem::size_of::<u8>() + mem::size_of::<i32>(),
            },
            Operator::CallExternal => StackOpCode {
                operator,
                address: 0,
                symbol: self.parser.read_binary::<i32>().unwrap(),
                value: 0,
                index: 0,
                operator_size: mem::size_of::<u8>() + mem::size_of::<i32>(),
            },
            Operator::PushInt => StackOpCode {
                operator,
                address: 0,
                symbol: 0,
                value: self.parser.read_binary::<i32>().unwrap(),
                index: 0,
                operator_size: mem::size_of::<u8>() + mem::size_of::<i32>(),
            },
            Operator::PushVar => StackOpCode {
                operator,
                address: 0,
                symbol: self.parser.read_binary::<i32>().unwrap(),
                value: 0,
                index: 0,
                operator_size: mem::size_of::<u8>() + mem::size_of::<i32>(),
            },
            Operator::PushInstance => StackOpCode {
                operator,
                address: 0,
                symbol: self.parser.read_binary::<i32>().unwrap(),
                value: 0,
                index: 0,
                operator_size: mem::size_of::<u8>() + mem::size_of::<i32>(),
            },
            Operator::Jump => StackOpCode {
                operator,
                address: self.parser.read_binary::<i32>().unwrap(),
                symbol: 0,
                value: 0,
                index: 0,
                operator_size: mem::size_of::<i32>(),
            },
            Operator::JumpIf => StackOpCode {
                operator,
                address: self.parser.read_binary::<i32>().unwrap(),
                symbol: 0,
                value: 0,
                index: 0,
                operator_size: mem::size_of::<u8>() + mem::size_of::<i32>(),
            },
            Operator::SetInstance => StackOpCode {
                operator,
                address: 0,
                symbol: self.parser.read_binary::<i32>().unwrap(),
                value: 0,
                index: 0,
                operator_size: mem::size_of::<u8>() + mem::size_of::<i32>(),
            },
            Operator::PushArrayVar => StackOpCode {
                operator,
                address: 0,
                symbol: self.parser.read_binary::<i32>().unwrap(),
                value: 0,
                index: self.parser.read_binary::<u8>().unwrap(),
                operator_size: 2 * mem::size_of::<u8>() + mem::size_of::<i32>(),
            },
            _ => StackOpCode {
                operator,
                address: 0,
                symbol: 0,
                value: 0,
                index: 0,
                operator_size: mem::size_of::<u8>(),
            },
        };
        stack_op_code
    }
}

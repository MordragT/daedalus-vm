use std::mem;
use std::num::{NonZeroI32, NonZeroU8};
use symbol::{Data, Flag, Kind, Properties, SymTable, SymbolBuilder};
use zen_parser::ZenParser;

pub mod symbol;

#[repr(u8)]
pub enum Operator {
    Add = 0,             // a + b
    Subract = 1,         // a - b
    Multiply = 2,        // a * b
    Divide = 3,          // a / b
    Mod = 4,             // a % b
    BinOr = 5,           // a | b
    BinAnd = 6,          // a & b
    Less = 7,            // a < b
    Greater = 8,         // a > b
    Assign = 9,          // a = b
    LogOr = 11,          // a || b
    LogAnd = 12,         // a && b
    ShiftLeft = 13,      // a << b
    ShiftRight = 14,     // a >> b
    LessOrEqual = 15,    // a <= b
    Equal = 16,          // a == b
    NotEqual = 17,       // a != b
    GreaterOrEqual = 18, // a >= b
    AssignAdd = 19,      // a += b (a = a + b)
    AssignSubtract = 20, // a -= b (a = a - b)
    AssignMultiply = 21, // a *= b (a = a * b)
    AssignDivide = 22,   // a /= b (a = a / b)
    Plus = 30,           // +a
    Minus = 31,          // -a
    Not = 32,            // !a
    Negate = 33,         // ~a
    //	LeftBracket     = 40,    // '('
    //	RightBracket    = 41,    // ')'
    //	Semicolon       = 42,    // ';'
    //	Comma           = 43,    // ','
    //	CurlyBracket    = 44,    // '{', '}'
    //	None            = 45,
    //	Float           = 51,
    //	Var             = 52,
    //	Operator        = 53,
    Ret = 60,
    Call = 61,
    CallExternal = 62,
    //	PopInt          = 63,
    PushInt = 64,
    PushVar = 65,
    //	PushString      = 66,
    PushInstance = 67,
    //	PushIndex       = 68,
    //	PopVar          = 69,
    AssignString = 70,
    AssignStringRef = 71,
    AssignFunc = 72,
    AssignFloat = 73,
    AssignInstance = 74,
    Jump = 75,
    JumpIf = 76,
    SetInstance = 80,
    //	Skip            = 90,
    //	Label           = 91,
    //	Func            = 92,
    //	FuncEnd         = 93,
    //	Class           = 94,
    //	ClassEnd        = 95,
    //	Instance        = 96,
    //	InstanceEnd     = 97,
    //	String          = 98,
    //	Array           = 180,  // Var + 128
    PushArrayVar = 245, // PushVar + Array
}
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

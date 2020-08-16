use bitfield::{bitfield, BitRange};
use std::collections::HashMap;
use std::mem;
use std::num::{NonZeroI32, NonZeroU32};

enum InstanceClass {
    Npc,
    Mission,
    Info,
    Item,
    ItemReact,
    Focus,
    Menu,
    MenuItem,
    Sfx,
    Pfx,
    MusicTheme,
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Flag {
    Const = 0b00001,
    Return = 0b00010,
    ClassVar = 0b00100,
    External = 0b01000,
    Merged = 0b10000,
}
#[repr(u8)]
#[derive(Debug)]
pub enum Kind {
    Void,
    Float,
    Int,
    CharString,
    Class,
    Func,
    Prototype,
    Instance,
}
pub enum Data {
    Float(f32),
    Int(i32),
    Char(char),
}
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

bitfield! {
    struct Element(u32);
    u32, get_count, set_count: 12, 0;
    Kind, get_kind, set_kind: 16, 12;
    u8, get_flags, set_flags: 22, 16;
    u32, get_space, set_space: 23, 22;
    u32, get_reserved, set_reserved: 32, 23;
}
impl BitRange<Kind> for Element {
    fn bit_range(&self, msb: usize, lsb: usize) -> Kind {
        let width = msb - lsb + 1;
        let mask = (1 << width) - 1;
        let num = ((self.0 >> lsb) & mask) as u8;
        let kind: Kind = unsafe { mem::transmute(num) };
        kind
    }
    fn set_bit_range(&mut self, msb: usize, lsb: usize, value: Kind) {
        self.0 = (value as u32) << lsb;
    }
}

// impl BitRange<Flag> for Element {
//     fn bit_range(&self, msb: usize, lsb: usize) -> Flag {
//         let width = msb - lsb + 1;
//         let mask = (1 << width) - 1;
//         let num = ((self.0 >> lsb) & mask) as u8;
//         let flag: Flag = unsafe { mem::transmute(num) };
//         flag
//     }
//     fn set_bit_range(&mut self, msb: usize, lsb: usize, value: Flag) {
//         self.0 = (value as u32) << lsb;
//     }
// }

bitfield! {
    struct Structure(u32);
    u32, get_value, set_value: 19, 0;
    u32, get_reserved, set_reserved: 32, 19;
}
bitfield! {
    struct CharStructure(u32);
    u32, get_value, set_value: 24, 0;
    u32, get_reserved, set_reserved: 32, 24;
}
pub struct Properties {
    off_cls_ret: i32,
    element: Element,
    // (Value, Reserved)
    file_index: Structure,
    line_start: Structure,
    line_count: Structure,
    char_start: CharStructure,
    char_count: CharStructure,
}
impl Properties {
    pub fn new(
        off_cls_ret: i32,
        element: u32,
        file_index: u32,
        line_start: u32,
        line_count: u32,
        char_start: u32,
        char_count: u32,
    ) -> Properties {
        Properties {
            off_cls_ret,
            element: Element(element),
            file_index: Structure(file_index),
            line_start: Structure(line_start),
            line_count: Structure(line_count),
            char_start: CharStructure(char_start),
            char_count: CharStructure(char_count),
        }
    }
    pub fn has_flag(&self, flag: Flag) -> bool {
        self.element.get_flags() & flag as u8 == flag as u8
    }
    pub fn is_not_flag(&self, flag: Flag) -> bool {
        self.element.get_flags() & flag as u8 == 0
    }
    pub fn get_flags(&self) -> u8 {
        self.element.get_flags()
    }
    pub fn get_count(&self) -> u32 {
        self.element.get_count()
    }
    pub fn get_kind(&self) -> Kind {
        self.element.get_kind()
    }
}
pub struct SymbolBuilder {
    name: String,
    properties: Option<Properties>,
    // Valid for Classes that write directly to the engine
    // Offset to be able to access class member via name
    class_member_offset: Option<NonZeroI32>,
    // Valid for Classes that write directly to the engine
    // Store array size of the Class member var
    class_member_array_size: Option<NonZeroI32>,
    parent: Option<NonZeroU32>,
    address: Option<NonZeroU32>,
    data: Option<Vec<Data>>,
}

impl SymbolBuilder {
    pub fn new(name: &str) -> SymbolBuilder {
        SymbolBuilder {
            name: String::from(name),
            properties: None,
            class_member_offset: None,
            class_member_array_size: None,
            parent: None,
            address: None,
            data: None,
        }
    }
    pub fn with_properties(&mut self, properties: Properties) -> &mut Self {
        self.properties = Some(properties);
        self
    }
    pub fn with_class_offset(&mut self, offset: i32) -> &mut Self {
        self.class_member_offset = NonZeroI32::new(offset);
        self
    }
    pub fn with_class_array_size(&mut self, array_size: i32) -> &mut Self {
        self.class_member_array_size = NonZeroI32::new(array_size);
        self
    }
    pub fn with_parent(&mut self, parent: u32) -> &mut Self {
        self.parent = NonZeroU32::new(parent);
        self
    }
    pub fn with_address(&mut self, address: u32) -> &mut Self {
        self.address = NonZeroU32::new(address);
        self
    }
    pub fn with_data(&mut self, data: Vec<Data>) -> &mut Self {
        self.data = Some(data);
        self
    }
    pub fn build<'a>(self) -> Result<Symbol, String> {
        if self.properties.is_none() {
            return Err("Cannot build Symbol, Properties are missing.".to_owned());
        }
        let properties = self.properties.unwrap();
        match properties.element.get_kind() {
            Kind::Float | Kind::Int | Kind::CharString if self.data.is_none() => {
                return Err(format!(
                    "Symbol is of kind {:?}, but does not specify its data.",
                    properties.element.get_kind()
                ))
            }
            _ => (),
        }
        Ok(Symbol {
            name: self.name,
            properties,
            class_member_offset: self.class_member_offset,
            class_member_array_size: self.class_member_array_size,
            parent: self.parent,
            address: self.address,
            data: self.data,
        })
    }
}

pub struct Symbol {
    name: String,
    pub properties: Properties,
    class_member_offset: Option<NonZeroI32>,
    // Valid for Classes that write directly to the engine
    // Store array size of the Class member var
    class_member_array_size: Option<NonZeroI32>,
    parent: Option<NonZeroU32>,
    address: Option<NonZeroU32>,
    data: Option<Vec<Data>>,
}

impl Symbol {
    pub fn get_name(&self) -> Option<&str> {
        //let name = self.name.clone();
        if self.name == "" {
            return None;
        } else {
            return Some(self.name.as_str());
        }
    }
    pub fn get_address(&self) -> Result<NonZeroU32, &str> {
        match self.address {
            Some(address) => return Ok(address),
            None => return Err("Address is not specified."),
        }
    }
    // pub fn get_data(&self) -> Result<Vec<Data>, &str> {
    //     match self.data {
    //         Some(data) => return Ok(data),
    //         None => return Err("Data not specified"),
    //     }
    // }
}
pub struct SymTable {
    sort_table: Vec<u32>,
    symbols: Vec<Symbol>,
    pub symbols_by_name: HashMap<String, usize>,
    pub functions_by_address: HashMap<usize, usize>,
}

impl SymTable {
    pub fn new() -> SymTable {
        SymTable {
            sort_table: vec![],
            symbols: vec![],
            symbols_by_name: HashMap::new(),
            functions_by_address: HashMap::new(),
        }
    }
    pub fn with_capacity(symbol_count: usize) -> SymTable {
        let sort_table = Vec::with_capacity(symbol_count);
        let symbols = Vec::with_capacity(symbol_count);
        let symbols_by_name = HashMap::with_capacity(symbol_count);
        SymTable {
            sort_table,
            symbols,
            symbols_by_name,
            functions_by_address: HashMap::new(),
        }
    }
    pub fn write_sort_table(&mut self, table: &[u32]) {
        self.sort_table = Vec::from(table);
    }
    pub fn has_symbol_name(&self, sym_name: &str) -> Result<(), String> {
        match self.symbols_by_name.get(sym_name) {
            Some(_) => return Ok(()),
            None => return Err(format!("Symbol {} not found", sym_name)),
        }
    }
    pub fn get_symbol_by_name(&self, sym_name: &str) -> Result<&Symbol, String> {
        match self.symbols_by_name.get(sym_name) {
            Some(index) => return Ok(self.symbols.get(*index).unwrap()),
            None => return Err(format!("Symbol {} not found", sym_name)),
        }
    }
    pub fn get_symbol_index_by_name(&self, sym_name: &str) -> Result<usize, String> {
        match self.symbols_by_name.get(sym_name) {
            Some(index) => return Ok(*index),
            None => return Err(format!("Symbol {} not found", sym_name)),
        }
    }
    pub fn get_symbol_by_index(&self, index: usize) -> Result<&Symbol, String> {
        match self.symbols.get(index) {
            Some(sym) => return Ok(sym),
            None => return Err(format!("Index {} out of bound", index)),
        }
    }
    pub fn get_function_index_by_address(&self, address: usize) -> Result<usize, String> {
        match self.functions_by_address.get(&address) {
            Some(index) => return Ok(*index),
            None => return Err(format!("Function at address {} not found", address)),
        }
    }
    pub fn insert_symbol(&mut self, index: usize, symbol: Symbol) -> usize {
        let name = symbol.get_name();
        if let Some(name) = name {
            self.symbols_by_name
                .insert(String::from(name), index as usize);
        }
        if (symbol.properties.get_kind() as u8 == Kind::Prototype as u8
            || symbol.properties.get_kind() as u8 == Kind::Func as u8)
            && !symbol.properties.has_flag(Flag::ClassVar)
            && symbol.properties.has_flag(Flag::Const)
        {
            self.functions_by_address
                .insert(symbol.get_address().unwrap().get() as usize, index as usize);
        }
        self.symbols.insert(index, symbol);
        self.symbols.len()
    }
    pub fn iterate_symbols_of_class(&self, class_name: &str, callback: &dyn Fn(usize, &Symbol)) {
        let base = self.get_symbol_index_by_name(class_name).unwrap();
        self.symbols.iter().enumerate().for_each(|(index, symbol)| {
            if symbol.properties.element.get_kind() as u8 != Kind::Instance as u8 {
                return;
            }
            let parent_address = match symbol.parent {
                Some(address) => address.get(),
                None => return,
            };
            let parent = self.get_symbol_by_index(parent_address as usize).unwrap();

            let parent_base = if parent.properties.element.get_kind() as u8 == Kind::Prototype as u8
            {
                match parent.parent {
                    Some(address) => address.get(),
                    None => parent_address,
                }
            } else {
                parent_address
            };
            if base == parent_base as usize {
                callback(index, symbol);
            }
        });
    }
}

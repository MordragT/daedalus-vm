use enumflags2::BitFlags;

pub mod file;
pub mod stack;
pub mod sym_table;
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

#[repr(u8)]
#[derive(BitFlags, Copy, Clone)]
pub enum Flag {
    Const = 0b00001,
    Return = 0b00010,
    ClassVar = 0b00100,
    External = 0b01000,
    Merged = 0b10000,
}
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Kind {
    Void = 0,
    Float = 1,
    Int = 2,
    CharString = 3,
    Class = 4,
    Func = 5,
    Prototype = 6,
    Instance = 7,
}

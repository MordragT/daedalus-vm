use super::{Flag, Kind};
use crate::stdlib::InstanceClass;
use bitfield::{bitfield, BitRange};
use std::mem;
use std::num::{NonZeroI32, NonZeroU32};
pub enum Data {
    Float(f32),
    Int(i32),
    CharString(String),
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
#[derive(Default)]
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
    instance_data_handle: Option<Handle>,
    instance_data_class: Option<InstanceClass>,
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
            instance_data_handle: None,
            instance_data_class: None,
            parent: None,
            address: None,
            data: None,
        }
    }
    pub fn set_kind(&mut self, kind: Kind) -> &mut Self {
        if self.data.is_none() {
            match kind {
                Kind::CharString => self.with_data(vec![Data::CharString("")]),
                Kind::Float => self.with_data(vec![Data::Float(0.0)]),
                Kind::Int => self.with_data(vec![Data::Int(0)]),
                _ => (),
            }
        }
        self.element.set_kind(kind);
        self
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
    pub fn with_instance_data(&mut self, handle: Handle, kind: InstanceClass) -> &mut Self {
        self.instance_data_handle = handle;
        self.instance_data_class = kind;
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
        let instance_data_handle = match self.instance_data_handle {
            Some(handle) => handle,
            None => Handle::new(),
        };
        Ok(Symbol {
            name: self.name,
            properties,
            class_member_offset: self.class_member_offset,
            class_member_array_size: self.class_member_array_size,
            instance_data_handle,
            instance_data_class: self.instance_data_class,
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
    instance_data_handle: Handle,
    instance_data_class: Option<InstanceClass>,
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
    pub fn set_address(&mut self, address: u32) {
        self.address = NonZeroU32::new(address);
    }
    pub fn nth(&self, index: usize) -> Result<Data, &str> {
        match self.data {
            Some(data) => return Ok(data.nth(index)),
            None => return Err("Data not specified"),
        }
    }
    pub fn get(&self) -> Result<Box<Vec<Data>>, &str> {
        match self.data {
            Some(data) => Ok(Box::new(data)),
            None => return Err("Data not specified"),
        }
    }
    pub fn set_class_member(&mut self, offset: u32, array_size: u32) {
        self.class_member_offset = NonZeroU32::new(offset);
        self.class_member_array_size = NonZeroU32::new(array_size);
    }
}

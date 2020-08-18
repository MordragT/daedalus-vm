use super::symbol::Symbol;
use super::{Flag, Kind};
use crate::stdlib::instances::Instance;
use std::collections::HashMap;
use std::mem;

#[derive(Default)]
pub struct SymTable {
    sort_table: Vec<u32>,
    symbols: Vec<Symbol>,
    pub symbols_by_name: HashMap<String, usize>,
    pub functions_by_address: HashMap<usize, usize>,
}

impl SymTable {
    pub fn new() -> Self {
        Default::default()
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
    pub fn has_symbol_name(&self, sym_name: &str) -> bool {
        match self.symbols_by_name.get(sym_name) {
            Some(_) => true,
            None => false,
        }
    }
    pub fn get_symbol_by_name(&self, sym_name: &str) -> Result<&Symbol, String> {
        match self.symbols_by_name.get(sym_name) {
            Some(index) => Ok(self.symbols.get(*index).unwrap()),
            None => Err(format!("Symbol {} not found", sym_name)),
        }
    }
    pub fn get_mut_symbol_by_name(&self, sym_name: &str) -> Result<&mut Symbol, String> {
        match self.symbols_by_name.get(sym_name) {
            Some(index) => Ok(self.symbols.get_mut(*index).unwrap()),
            None => Err(format!("Symbol {} not found", sym_name)),
        }
    }
    pub fn get_symbol_index_by_name(&self, sym_name: &str) -> Result<usize, String> {
        match self.symbols_by_name.get(sym_name) {
            Some(index) => Ok(*index),
            None => Err(format!("Symbol {} not found", sym_name)),
        }
    }
    pub fn get_symbol_by_index(&self, index: usize) -> Result<&Symbol, String> {
        match self.symbols.get(index) {
            Some(sym) => Ok(sym),
            None => Err(format!("Index {} out of bound", index)),
        }
    }
    pub fn get_function_index_by_address(&self, address: usize) -> Result<usize, String> {
        match self.functions_by_address.get(&address) {
            Some(index) => Ok(*index),
            None => Err(format!("Function at address {} not found", address)),
        }
    }
    fn insert_symbol_in_hash_maps(&mut self, index: usize, symbol: &Symbol) {
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
    }
    pub fn insert(&mut self, index: usize, symbol: Symbol) -> usize {
        self.insert_symbol_in_hash_maps(index, symbol);
        self.symbols.insert(index, symbol);
        self.symbols.len()
    }
    pub fn push(&mut self, symbol: Symbol) -> usize {
        let index = self.symbols.len();
        self.insert_symbol_in_hash_maps(index, symbol);
        self.symbols.push(symbol);
        index + 1
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
    pub fn register<I: Instance, M>(&self, sym_name: &str, instance: I, member: M) {
        match self.get_mut_symbol_by_name(sym_name) {
            Ok(symbol) => {
                let offset = &member - &instance;
                symbol.set_class_member(offset, mem::size_of_val(member));
            }
            Err(_) => (),
        }
    }
}

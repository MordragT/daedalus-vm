use crate::stdlib::prelude::*;
use crate::vm::VirtualMachine;
use object_allocator::ObjectAllocator;
use std::collections::HashMap;
use zen_memory::Handle;

mod object_allocator;

const MAX_NUM_MISC: usize = 1024;

const MAX_NUM_NPCS: usize = 12000;
const MAX_NUM_ITEMS: usize = 12000;
const MAX_NUM_MISSIONS: usize = 512;
const MAX_NUM_FOCUS: usize = MAX_NUM_MISC;
const MAX_NUM_ITEMREACT: usize = MAX_NUM_MISC;
const MAX_NUM_INFO: usize = 16000;
const MAX_NUM_MENU: usize = MAX_NUM_MISC;
const MAX_NUM_MENUITEM: usize = MAX_NUM_MISC;
const MAX_NUM_SFX: usize = 4096; // G2 has 1700
const MAX_NUM_PFX: usize = 1024;
const MAX_NUM_MUSICTHEME: usize = 512;

type Inventory = Vec<Handle>;

pub struct GameState<'a> {
    npcs: ObjectAllocator<Npc>,
    items: ObjectAllocator<Item>,
    item_reacts: ObjectAllocator<ItemReact>,
    missions: ObjectAllocator<Mission>,
    focuses: ObjectAllocator<Focus>,
    infos: ObjectAllocator<Info>,
    menu: ObjectAllocator<Menu>,
    menu_items: ObjectAllocator<MenuItem>,
    sound_effects: ObjectAllocator<SoundEffect>,
    particle_effects: ObjectAllocator<ParticleEffect>,
    music_themes: ObjectAllocator<MusicTheme>,
    virtual_machine: &'a VirtualMachine,
    npc_inventories: HashMap<Handle, Inventory>,
}

impl<'a> GameState<'a> {
    pub fn new(virtual_machine: &VirtualMachine) -> Self {
        Self {
            npcs: ObjectAllocator::<Npc>::new(MAX_NUM_NPCS),
            items: ObjectAllocator::<Item>::new(MAX_NUM_ITEMS),
            item_reacts: ObjectAllocator::<ItemReact>::new(MAX_NUM_ITEMREACT),
            missions: ObjectAllocator::<Mission>::new(MAX_NUM_MISSIONS),
            focuses: ObjectAllocator::<Focus>::new(MAX_NUM_FOCUS),
            infos: ObjectAllocator::<Info>::new(MAX_NUM_INFO),
            menu: ObjectAllocator::<Menu>::new(MAX_NUM_MENU),
            menu_items: ObjectAllocator::<MenuItem>::new(MAX_NUM_MENUITEM),
            sound_effects: ObjectAllocator::<SoundEffect>::new(MAX_NUM_SFX),
            particle_effects: ObjectAllocator::<ParticleEffect>::new(MAX_NUM_PFX),
            music_themes: ObjectAllocator::<MusicTheme>::new(MAX_NUM_MUSICTHEME),
            virtual_machine,
            npc_inventories: HashMap::new(),
        }
    }
    pub fn insert_npc(&self, instance: usize, waypoint: &str) -> Handle {
        let handle = self.npcs.create().unwrap();
        let npc = self.npcs.get_mut(handle).unwrap();
        npc.set_waypoint(waypoint);
        npc.set_instance_symbol(instance);
        let symbol = self
            .virtual_machine
            .get_file()
            .sym_table
            .get_symbol_by_index(instance)
            .unwrap();
    }
    pub fn insert_item(&self, instance: usize) -> Handle {}
    pub fn insert_sound_effect(&self, instance: usize) -> Handle {}
    pub fn insert_music_theme(&self, instance: usize) -> Handle {}

    pub fn create_inv_item(&mut self, item_symbol: usize, npc: &Handle, amount: u32) -> Handle {
        if amount == 0 {
            amount = 1;
        }
        let items = self.npc_inventories.get(npc).unwrap();
        for handle in items {
            let item = self.items.get_mut(handle).unwrap();
            if item.get_instance_symbol() == item_symbol {
                item.amount += amount;
                return handle;
            }
        }
        let handle = self.items.create().unwrap();
        let item = self.items.get_mut(handle).unwrap();
        item.amount = amount;

        self.virtual_machine
            .initialise_instance(handle, item_symbol, InstanceClass::Item);
        self.add_item_to_inv(handle, npc);
        handle
    }
    pub fn add_item_to_inv(&mut self, item_handle: &Handle, npc: &Handle) -> Handle {
        let items = self.npc_inventories.get(npc).unwrap();
        for handle in items {
            let item = self.items.get_mut(handle).unwrap();
            if item.get_instance_symbol()
                == self.items.get(item_handle).unwrap().get_instance_symbol()
            {
                item.amount += 1;
                return handle;
            }
        }
    }
    pub fn remove_inv_item(&mut self, item_symbol: usize, npc: &Handle, amount: u32) -> bool {
        let items = self.npc_inventories.get_mut(npc).unwrap();
        let handle = None;
        for inner_handle in items {
            let item = self.items.get(inner_handle).unwrap();
            if item.get_instance_symbol() == item_symbol {
                handle = Some(inner_handle);
            }
        }
        let handle = match handle {
            Some(handle) => {
                let item = self.items.get_mut(handle).unwrap();
                match item.amount > amount {
                    true => {
                        item.amount -= amount;
                        return true;
                    }
                    false => handle,
                }
            }
            None => return false,
        };
        self.items.remove(handle);
        items.remove_item(handle);
        return true;
    }
    pub fn get_inv_of(&self, npc: &Handle) -> Result<&mut Inventory, &str> {
        match self.npc_inventories.get_mut(npc) {
            Some(val) => Ok(val),
            None => Err("Npc has no Inventory"),
        }
    }
}

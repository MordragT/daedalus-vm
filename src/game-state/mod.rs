use zen_memory::{StaticReferencedAllocator, Handle};
use super::stdlib::npc::Npc;
use super::stdlib::item::{Item, ItemReact};
use super::stdlib::player::Mission;
use super::stdlib::{Focus, Info};
use super::stdlib::menu::{Menu, Item as MenuItem};
use super::stdlib::sound::{SoundEffect, MusicTheme};
use super::stdlib::particle::ParticleEffect;

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

type Allocator<T> = StaticReferencedAllocator<T>;

/// RegisteredObjects and their max number of instances
pub enum ObjectAllocator {
    Npcs(Allocator<Npc>, MAX_NUM_NPCS),
    Items(Allocator<Item>, MAX_NUM_ITEMS),
    ItemReacts(Allocator<ItemReact>, MAX_NUM_ITEMREACT),
    Missions(Allocator<Mission>, MAX_NUM_MISSIONS),
    Focuses(Allocator<Focus>, MAX_NUM_FOCUS),
    Infos(Allocator<Info>, MAX_NUM_INFO),
    Menus(Allocator<Menu>, MAX_NUM_MENU),
    MenuItems(Allocator<MenuItem>, MAX_NUM_MENUITEM),
    SoundEffects(Allocator<SoundEffect>, MAX_NUM_SFX),
    ParticleEffects(Allocator<ParticleEffect>, MAX_NUM_PFX),
    MusicThemes(Allocator<MusicTheme>, MAX_NUM_MUSICTHEME),
}

impl ObjectAllocator {
    pub fn new() -> ObjectAllocator {
        
    }
    pub fn create_handle(&self) -> Handle {

    }
    pub fn get_npc(&self, handle: &Handle) -> Option<Npc> {
        match self {
            RegisteredObject::Npcs(alloc, _) => Some(alloc.get_element(handle)),
            _ => None,
        }
    }
    pub fn get_item(&self, handle: &Handle) -> Option<Item> {
        match self {
            RegisteredObject::Items(alloc, _) => Some(alloc.get_element(handle)),
            _ => None,
        }
    }
    pub fn get_item_react(&self, handle: &Handle) -> Option<ItemReact> {
        match self {
            RegisteredObject::ItemReacts(alloc, _) => Some(alloc.get_element(handle)),
            _ => None,
        }
    }
    pub fn get_mission(&self, handle: &Handle) -> Option<Mission> {
        match self {
            RegisteredObject::Missions(alloc, _) => Some(alloc.get_element(handle)),
            _ => None,
        }
    }
    pub fn get_focus(&self, handle: &Handle) -> Option<Focus> {
        match self {
            RegisteredObject::Focuses(alloc, _) => Some(alloc.get_element(handle)),
            _ => None,
        }
    }
    pub fn get_info(&self, handle: &Handle) -> Option<Info> {
        match self {
            RegisteredObject::Infos(alloc, _) => Some(alloc.get_element(handle)),
            _ => None,
        }
    }
    pub fn get_menu(&self, handle: &Handle) -> Option<Menu> {
        match self {
            RegisteredObject::Menus(alloc, _) => Some(alloc.get_element(handle)),
            _ => None,
        }
    }
    pub fn get_menu_item(&self, handle &Handle) -> Option<MenuItem> {
        match self {
            RegisteredObject::MenuItems(alloc, _) => Some(alloc.get_element(handle)),
            _ => None,
        }
    }
    pub fn get_sound_effect(&self, handle: &Handle) -> Option<SoundEffect> {
        match self {
            RegisteredObject::SoundEffects(alloc, _) => Some(alloc.get_element(handle)),
            _ => None,
        }
    }
    pub fn get_particle_effect(&self, handle: &Handle) -> Option<ParticleEffect> {
        match self {
            RegisteredObject::ParticleEffects(alloc, _) => Some(alloc.get_element(handle)),
            _ => None,
        }
    }
    pub fn get_music_theme(&self, handle: &Handle) -> Option<MusicTheme> {
        match self {
            RegisteredObject::MusicThemes(alloc, _) => Some(alloc.get_element(handle)),
            _ => None,
        }
    }
    // TODO: kompletes Allocator interface nun hier als wrapper implementieren
    // möglicherweise reicht aber auch get()
}

pub struct GameState {

}

impl GameState {
    // create func through registed objects implemented
    pub fn create_inv_item(&self, item_symbol: usize, npc: Box<Handle>) -> Handle {}
    pub fn add_item_to_inv(&self, item: &Handle, npc: Box<Handle>) -> Handle {}
    pub fn remove_inv_item(&self, item_symbol: usize, npc: Handle) -> Result<(), &str> {}

    pub fn remove_item(&self, item: Handle) {}
    pub fn remove_npc(&self, npc: Handle) {}
    pub fn remove_menu(&self, menu: Handle) {}
    pub fn remove_menu_item(&self, menu_item: Handle) {}

    pub fn insert_npc(&self, instance: usize, waypoint: String) -> Handle {}
    pub fn insert_item(&self, instance: usize) -> Handle {}
    pub fn insert_sound_effect(&self, instance: usize) -> Handle {}
    pub fn insert_music_theme(&self, instance: usize) -> Handle {}

}
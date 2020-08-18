pub mod focus;
pub mod info;
pub mod item;
pub mod item_react;
pub mod menu;
pub mod menu_item;
pub mod mission;
pub mod music_theme;
pub mod npc;
pub mod particle_effect;
pub mod sound_effect;
pub mod spell;

pub const MAX_CHAPTER: usize = 5;
pub const MAX_MISSIONS: usize = 5;
pub const MAX_HITCHANCE: usize = 5;
pub const ATR_INDEX_MAX: usize = 8;
pub const ITM_TEXT_MAX: usize = 6;

pub const DAM_INDEX_BARRIER: usize = 0; //				 nur der Vollstandigkeit und Transparenz wegen hier definiert ( _NICHT_ verwenden )
pub const DAM_INDEX_BLUNT: usize = DAM_INDEX_BARRIER + 1;
pub const DAM_INDEX_EDGE: usize = DAM_INDEX_BLUNT + 1;
pub const DAM_INDEX_FIRE: usize = DAM_INDEX_EDGE + 1;
pub const DAM_INDEX_FLY: usize = DAM_INDEX_FIRE + 1;
pub const DAM_INDEX_MAGIC: usize = DAM_INDEX_FLY + 1;
pub const DAM_INDEX_POINT: usize = DAM_INDEX_MAGIC + 1;
pub const DAM_INDEX_FALL: usize = DAM_INDEX_POINT + 1; //				 nur der Vollstandigkeit und Transparenz wegen hier definiert ( _NICHT_ verwenden )
pub const DAM_INDEX_MAX: usize = DAM_INDEX_FALL + 1;

pub const PROT_BARRIER: usize = DAM_INDEX_BARRIER;
pub const PROT_BLUNT: usize = DAM_INDEX_BLUNT;
pub const PROT_EDGE: usize = DAM_INDEX_EDGE;
pub const PROT_FIRE: usize = DAM_INDEX_FIRE;
pub const PROT_FLY: usize = DAM_INDEX_FLY;
pub const PROT_MAGIC: usize = DAM_INDEX_MAGIC;
pub const PROT_POINT: usize = DAM_INDEX_POINT;
pub const PROT_FALL: usize = DAM_INDEX_FALL;
pub const PROT_INDEX_MAX: usize = DAM_INDEX_MAX;

pub const COND_ATR_MAX: usize = 3;

pub trait Instance {
    fn get_instance_symbol(&self) -> usize;
    fn set_instance_symbol(&mut self, instance_symbol: usize);
}

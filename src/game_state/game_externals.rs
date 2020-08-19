use zen_memory::Handle;
#[derive(Default)]
pub struct GameExternals<'a> {
    pub insert_npc: Option<&'a dyn Fn(Handle, &str)>,
    pub post_insert_npc: Option<&'a dyn Fn(Handle)>,
    pub remove_npc: Option<&'a dyn Fn(Handle)>,
    pub insert_item: Option<&'a dyn Fn(Handle)>,
    pub create_inv_item: Option<&'a dyn Fn(Handle, Handle)>,
    pub get_day: Option<&'a dyn Fn() -> i32>,
    pub log_create_topic: Option<&'a dyn Fn(&str)>,
    pub log_set_topic_status: Option<&'a dyn Fn(&str)>,
    pub log_add_entry: Option<&'a dyn Fn(&str, &str)>,
}

impl<'a> GameExternals<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

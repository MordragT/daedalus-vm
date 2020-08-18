#[derive(Default)]
pub struct ParticleEffect {
    // 1) Emitter: zeitliches  Austoss-Verhalten, particles-per-second
    pps_value: f32,
    pps_scale_keys: String,
    pps_is_looping: i32,
    pps_is_smooth: i32,
    pps_fps: f32,
    pps_create_em: String,
    pps_create_em_delay: f32,

    // 2) Emitter: raeumliches Austoss-Verhalten
    shp_type: String, //	"point, line, box, circle, sphere, mesh"
    shp_for: String,  //	"object,world"
    shp_offset_vec: String,
    shp_distrib_type: String, //	"RAND, UNIFORM, WALK"
    shp_distrib_walk_speed: f32,
    shp_is_volume: i32,
    shp_dim: String, //	"", "30", "10 20 30", "30", "30", "" //	line: nur 1 Dimension !=0 // shape Dimensions
    shp_mesh: String, //	"cross.3ds"
    shp_mesh_render: i32,
    shp_scale_keys: String, //	"[1.0] [0.8 0.9 0.2] [1.0]"
    shp_scale_is_looping: i32,
    shp_scale_is_smooth: i32,
    shp_scale_fps: f32,

    // 3) Partikel: Start Richtung/Speed:
    dir_mode: String, //	"DIR, TARGET, MESH_POLY"
    dir_for: String,  //	"OBJECT, WORLD"
    dir_mode_target_for: String,
    dir_mode_target_pos: String, //	"30 23 67"
    dir_angle_head: f32,
    dir_angle_head_var: f32,
    dir_angle_elev: f32,
    dir_angle_elev_var: f32,
    vel_avg: f32,
    vel_var: f32,

    // 4) Partikel: Lebensdauer
    lsp_part_avg: f32,
    lsp_part_var: f32,

    // 5) Partikel: Flugverhalten (gravity, nicht-linear?, mesh-selfRot?,..)
    // grav: a) nur Y, b) XYZ, c) auf Ziel zu steuern
    //  flyMode_S;								//	"LINEAR, LIN_SINUS,.."
    // flyMeshSelfRotSpeedMin, flyMeshSelfRotSpeedMax
    fly_gravity: String,
    fly_coll_det: i32,

    // 6) Partikel: Visualisierung
    vis_name: String,          //	"NAME_V0_A0.TGA/.3DS"	(Variation, Animation)
    vis_orientation: String,   //	"NONE, VELO"
    vis_tex_is_quad_poly: i32, //	0=triMesh, 1=quadMesh
    vis_tex_ani_fps: f32,
    vis_tex_ani_is_looping: i32, //	0=oneShot, 1=looping
    // color		(nur Tex, lifeSpan-Sync)
    vis_tex_color_start: String,
    vis_tex_color_end: String,
    // size-ani		(nur Tex, lifeSpan-Sync)
    vis_size_start: String,
    vis_size_end_scale: f32,
    // alpha		(lifeSpan-Sync)
    vis_alpha_func: String,
    vis_alpha_start: f32,
    vis_alpha_end: f32,

    // 7) misc effects

    // trail
    trl_fade_speed: f32,
    trl_texture: String,
    trl_width: f32,

    // marks
    mrk_fade_speed: f32,
    mrk_texture: String,
    mrk_size: f32,

    // flocking
    flock_mode: String,
    flock_strength: f32,

    // local frame of reference override
    // calculates the position of the particles each frame relative to the emitters pos/rot
    // can be expensive
    // WARNING: in comb with flyCollDet_B this can be a performance-hog deluxe
    use_emitters_for: i32,

    // optional you can set a valid timeperiod in which this pfx should be rendered (e.g. "8 22": should be rendererd from 8 to 22 o clock")
    time_start_end: String,

    // with the next setting you can define weather this pfx is an ambient pfx, thus can be disabled in the gothic.ini with the value [ENGINE]/noAmbientPFX
    b_is_ambient_pfx: i32,
}

impl ParticleEffect {
    pub fn new() -> ParticleEffect {
        Default::default()
    }
}

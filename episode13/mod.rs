use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

// Allows for variable to be tracked for every player seperately
static mut paluPosX: [f32; 8] = [0.0; 8];
static mut paluPosY: [f32; 8] = [0.0; 8];
static mut paluPosZ: [f32; 8] = [0.0; 8];

unsafe extern "C" fn palutena_game_appealhi(agent: &mut L2CAgentBase) {
    // Get player number
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(agent) {
        KineticModule::set_consider_ground_friction(agent.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("appeal_hi_l"), false, -1.0);
        // Sets position
        PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: paluPosX[entry_id], y: paluPosY[entry_id], z: paluPosZ[entry_id] });
    }
}

unsafe extern "C" fn palutena_game_appeallw(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(agent) {
        paluPosX[entry_id] = PostureModule::pos_x(agent.module_accessor);
        paluPosY[entry_id] = PostureModule::pos_y(agent.module_accessor);
        paluPosZ[entry_id] = PostureModule::pos_z(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("palutena")
        .game_acmd("game_appealhil", palutena_game_appealhi, Default)
        .game_acmd("game_appealhir", palutena_game_appealhi, Default)
        .game_acmd("game_appeallwl", palutena_game_appeallw, Default)
        .game_acmd("game_appeallwr", palutena_game_appeallw, Default)
        .install();
}
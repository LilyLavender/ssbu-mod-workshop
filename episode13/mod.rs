use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
		hash40
    },
    smash_script::*,
    smashline::*,
};

static mut paluPosX: [f32; 8] = [0.0; 8];
static mut paluPosY: [f32; 8] = [0.0; 8];
static mut paluPosZ: [f32; 8] = [0.0; 8];

#[acmd_script( agent = "palutena", scripts = [ "game_appealhil", "game_appealhir" ], category = ACMD_GAME, low_priority )]
unsafe fn palutena_appealhi(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if macros::is_excute(fighter) {
        KineticModule::set_consider_ground_friction(fighter.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("appeal_hi_l"), false, -1.0);
		PostureModule::set_pos(fighter.module_accessor, &Vector3f{ x: paluPosX[entry_id], y: paluPosY[entry_id], z: paluPosZ[entry_id] });
	}
}

#[acmd_script( agent = "palutena", scripts = [ "game_appeallwl", "game_appeallwr" ], category = ACMD_GAME, low_priority )]
unsafe fn palutena_appeallw(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if macros::is_excute(fighter) {
		paluPosX[entry_id] = PostureModule::pos_x(fighter.module_accessor);
		paluPosY[entry_id] = PostureModule::pos_y(fighter.module_accessor);
		paluPosZ[entry_id] = PostureModule::pos_z(fighter.module_accessor);
	}
}

pub fn install() {
    smashline::install_acmd_scripts!(
        palutena_appealhi,
		palutena_appeallw
    );
}

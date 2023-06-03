use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
		hash40
    },
    smash_script::*,
    smashline::*
};

// Use these for effects
//pub static mut TIME_SLOW_EFFECT_VECTOR: smash::phx::Vector3f = smash::phx::Vector3f {x:-3.0,y:3.0,z:0.0};
//pub const TIME_SLOW_EFFECT_HASH: u64 = smash::hash40("sys_sp_flash");

#[fighter_frame( agent = FIGHTER_KIND_BAYONETTA )]
fn bayonetta_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        // get bomas
        let boma1 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(0));
        let boma2 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
		
		// get positions
		let b1x = PostureModule::pos_x(boma1);
		let b1y = PostureModule::pos_y(boma1);
		let b2x = PostureModule::pos_x(boma2);
		let b2y = PostureModule::pos_y(boma2);
		
		// distance formula
		let dSquared: f32 = (b1x - b2x) * (b1x - b2x) + (b1y - b2y) * (b1y - b2y);
		let d = dSquared.sqrt();
		
		// if players are close, slow opponent
		if d < 50.0 {
			macros::SLOW_OPPONENT(fighter, 3.0, 1.0); // Strength, frames
			//EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new_raw(TIME_SLOW_EFFECT_HASH), smash::phx::Hash40::new("head"), &TIME_SLOW_EFFECT_VECTOR, &TIME_SLOW_EFFECT_VECTOR, 1.0, &TIME_SLOW_EFFECT_VECTOR, &TIME_SLOW_EFFECT_VECTOR, false, 0, 0, 0);
		}
		
	}
}

pub fn install() {
	smashline::install_agent_frames!(
		bayonetta_frame
    );
}

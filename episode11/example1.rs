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

#[fighter_frame( agent = FIGHTER_KIND_KAMUI )]
fn kamui_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		// No special fall
	    	if(MotionModule::motion_kind(fighter.module_accessor) == hash40("fall_special")) {
			fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
			WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
		}
	}
}

pub fn install() {
    smashline::install_agent_frames!(
		kamui_frame,
    );
}

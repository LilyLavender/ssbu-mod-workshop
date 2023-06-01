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

#[fighter_frame( agent = FIGHTER_KIND_DEDEDE )]
fn dedede_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		
		let currentSize = PostureModule::scale(fighter.module_accessor);
		
		if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
			PostureModule::set_scale(fighter.module_accessor, currentSize*1.04, false);
		} else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
			PostureModule::set_scale(fighter.module_accessor, currentSize/1.04, false);
		} 
		
	}
}

pub fn install() {
	smashline::install_agent_frames!(
		dedede_frame
    );
}

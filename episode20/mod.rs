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

unsafe extern "C" fn dedede_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let currentSize = PostureModule::scale(fighter.module_accessor);
        
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
            PostureModule::set_scale(fighter.module_accessor, currentSize * 1.04, false);
        } else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            PostureModule::set_scale(fighter.module_accessor, currentSize / 1.04, false);
        } 
        
    }
}

pub fn install() {
    Agent::new("dedede")
        .on_line(Main, dedede_frame)
        .install();
}

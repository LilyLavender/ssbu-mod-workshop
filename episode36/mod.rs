use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CAgent, L2CValue},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

unsafe extern "C" fn fox_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = fighter.module_accessor;
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD && 
        ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
        }
    }
}

pub fn install() {
    Agent::new("fox")
        .on_line(Main, fox_frame)
        .install();
}

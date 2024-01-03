use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::*
};

#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        // Return player 2 to 0% if above 80%
        let boma2 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
        if DamageModule::damage(boma2, 0) > 80.0 {
            DamageModule::add_damage(boma2, -DamageModule::damage(boma2, 0), 0);
        }
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame
    );
}
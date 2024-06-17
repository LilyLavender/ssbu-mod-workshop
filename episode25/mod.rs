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

#[skyline::hook(replace = StatusModule::situation_kind)]
unsafe fn situation_kind_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> i32 {
    // On left side of stage
    if PostureModule::pos_x(module_accessor) < 0.0 {
        return *SITUATION_KIND_GROUND;
    } 
    // Run original code of situation_kind funct
    original!()(module_accessor)
}

pub fn install() {
    skyline::install_hooks!(
        situation_kind_replace
    );
}
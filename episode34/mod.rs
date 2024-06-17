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

unsafe extern "C" fn younglink_game_appeal(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_DEKU), 0, 0, false, false);
    }
}

pub fn install() {
    Agent::new("younglink")
        .game_acmd("game_appealhil", younglink_game_appeal, Default)
        .game_acmd("game_appealhir", younglink_game_appeal, Default)
        .game_acmd("game_appealsl", younglink_game_appeal, Default)
        .game_acmd("game_appealsr", younglink_game_appeal, Default)
        .game_acmd("game_appeallwl", younglink_game_appeal, Default)
        .game_acmd("game_appeallwr", younglink_game_appeal, Default)
        .install();
}
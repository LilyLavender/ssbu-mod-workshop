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

#[acmd_script( agent = "younglink", scripts = [ "game_appealhil", "game_appealhir", "game_appealsl", "game_appealsr", "game_appeallwl", "game_appeallwr" ], category = ACMD_GAME, low_priority )]
unsafe fn younglink_game_appeal(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_DEKU), 0, 0, false, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        younglink_game_appeal
    );
}
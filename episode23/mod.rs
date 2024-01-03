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

#[acmd_script( agent = "link", scripts = [ "game_appealhil", "game_appealhir" ], category = ACMD_GAME, low_priority )]
unsafe fn link_game_appealhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) { // Dpad up
            VisibilityModule::set_whole(agent.module_accessor, true);
        } else if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) { // Dpad down
            VisibilityModule::set_whole(agent.module_accessor, false);
        } else if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) { // Dpad right
            ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("link_ken"), false);
            ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("link_tate"), false);
            ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("link_kenb"), false);
            ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("link_tateb"), false);
        } else if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) { // Dpad left
            ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("link_ken"), true);
            ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("link_tate"), true);
            ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("link_kenb"), true);
            ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("link_tateb"), true);
        }
    }
}

static mut linkPosX: [f32; 8] = [0.0; 8];
static mut linkPosY: [f32; 8] = [0.0; 8];
static mut linkPosZ: [f32; 8] = [0.0; 8];

#[acmd_script( agent = "link", script = "game_appealsr", category = ACMD_GAME, low_priority )]
unsafe fn link_game_appealsr(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_hit_curse"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: linkPosX[entry_id], y: linkPosY[entry_id], z: linkPosZ[entry_id] });
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_curse"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 3.0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 2);
        VisibilityModule::set_whole(agent.module_accessor, true);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "link", script = "game_appealsl", category = ACMD_GAME, low_priority )]
unsafe fn link_game_appealsl(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(agent) {
        linkPosX[entry_id] = PostureModule::pos_x(agent.module_accessor);
        linkPosY[entry_id] = PostureModule::pos_y(agent.module_accessor);
        linkPosZ[entry_id] = PostureModule::pos_z(agent.module_accessor);
    }
}


pub fn install() {
    smashline::install_acmd_scripts!(
        link_game_appealhi,
        link_game_appealsr,
        link_game_appealsl
    );
}
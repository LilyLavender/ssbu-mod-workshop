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

unsafe extern "C" fn samusd_game_attack12(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        // Does damage and knockback of jab1 to make it connect into jab3 easier
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 3.0, 361, 10, 0, 25, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 3.0, 361, 10, 0, 25, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    // Added flag to tell the game that it can transition into jab3 if the player chooses
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

unsafe extern "C" fn samusd_game_attack13(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.7, 361, 60, 0, 80, 2.8, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ELBOW);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.7, 361, 60, 0, 80, 3.4, 0.0, 8.5, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ELBOW);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 6.7, 361, 60, 0, 80, 4.2, 0.0, 9.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ELBOW);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn samusd_effect_attack13(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 4, 5.5, -2, -37, -19, 13, 1.15, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.5);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 4, 6, -4, -37, -19, 13, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.5);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 15, 8, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true, 0.6);
    }
}

pub fn install() {
    Agent::new("samusd")
        .game_acmd("game_attack12", samusd_game_attack12, Default)
        .game_acmd("game_attack13", samusd_game_attack13, Default)
        .effect_acmd("effect_attack13", samusd_effect_attack13, Default)
        .install();
}
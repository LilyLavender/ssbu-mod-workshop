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

unsafe extern "C" fn cloud_game_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        // Random number 40 to 139
        let deg = (smash::app::sv_math::rand(hash40("agent"), 100) + 40) as u64;
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 8.0, deg, 100, 0, 46, 5.6, 0.0, 12.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 8.0, deg, 100, 0, 46, 5.0, 0.0, 8.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 8.0, deg, 100, 0, 46, 6.0, -1.0, 3.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 8.0, deg, 100, 0, 46, 4.5, 0.0, 21.0, 0.0, Some(0.0), Some(16.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 3, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn cloud_game_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 60, 30, 25, 30, 4.5, 0.0, 9.0, 10.0, Some(0.0), Some(9.0), Some(6.5), 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_SMASH_01, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 170, 100, 10, 0, 4.5, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(6.5), 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_SMASH_01, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 0, 100, 10, 0, 4.5, 0.0, 9.0, 10.0, Some(0.0), Some(9.0), Some(6.5), 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_SMASH_01, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 180, 100, 10, 0, 4.5, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(6.5), 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_SMASH_01, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 15, 100, 80, 0, 2.5, 0.0, 9.0, 5.0, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_SMASH_02, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 15, 100, 50, 0, 5.2, 0.0, 9.0, 10.0, Some(0.0), Some(9.0), Some(6.5), 1.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_SMASH_02, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 170, 100, 10, 0, 5.2, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(6.5), 1.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_SMASH_02, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        // Random number 0-7
        let rand = smash::app::sv_math::rand(hash40("agent"), 8) as u64;
        if rand != 7 {
            // Vanilla hitboxes
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 361, 115, 0, 42, 5.5, 0.0, 9.0, 10.0, Some(0.0), Some(9.0), Some(6.5), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_SMASH_03, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 361, 115, 0, 42, 5.5, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(6.5), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_SMASH_03, *ATTACK_REGION_SWORD);
        } else {
            // Crit hitboxes
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 26.0, 361, 115, 0, 42, 5.5, 0.0, 9.0, 10.0, Some(0.0), Some(9.0), Some(6.5), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_SMASH_03, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 115, 0, 42, 5.5, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(6.5), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_SMASH_03, *ATTACK_REGION_SWORD);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("cloud")
        .game_acmd("game_attackhi3", cloud_game_attackhi3, Default)
        .game_acmd("game_attacks4", cloud_game_attacks4, Default)
        .install();
}
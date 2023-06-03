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

#[acmd_script( agent = "wario", script = "game_attackhi3", category = ACMD_GAME, low_priority )]
unsafe fn wario_attackhi3(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 1 {
		
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 5.5, 96, 130, 0, 28, 3.5, -0.5, -0.8, 0.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 5.5, 96, 130, 0, 28, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 5.5, 96, 130, 0, 28, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		
	} else {
		
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 92, 130, 0, 28, 5.0, 0.0, 18.0, 5.0, Some(0.0), Some(18.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 95, 130, 0, 28, 5.0, 0.0, 6.0, 1.5, Some(0.0), Some(6.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 90, 120, 0, 20, 4.0, 0.0, 14.5, 3.5, Some(0.0), Some(14.5), Some(-3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
			AttackModule::clear(fighter.module_accessor, 0, false);
		}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		
	}
}

#[acmd_script( agent = "wario", script = "effect_attackhi3", category = ACMD_EFFECT, low_priority )]
unsafe fn wario_attackhi3_fx(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 1 {
		
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mario_atkhi3_arc"), Hash40::new("mario_atkhi3_arc"), Hash40::new("top"), 2, 12, 1, 0, 4, 103, 1, true, *EF_FLIP_YZ);
		}
		frame(fighter.lua_state_agent, 24.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
		}
		
	} else {
		
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, -2, -5.5, -90, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
			macros::EFFECT(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, -2, 5.5, -90, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 19, -5, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 360, true);
			macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 19, 5, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 360, true);
		}
		
	}
}

pub fn install() {
	smashline::install_acmd_scripts!(
        wario_attackhi3,
		wario_attackhi3_fx
    );
}

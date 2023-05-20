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

#[acmd_script( agent = "kirby", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn kirby_attacklw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*do not edit*/
		0, /*id*/
		0, /*part*/
		Hash40::new("toer"), /*bone*/
		6.0, /*damage*/
		361, /*angle*/
		43, 0, 30, /*kbg, fkb, bkb*/
		3.7, /*size*/
		4.3, 0.0, 0.0, /*position*/
		None, None, None, /*position 2*/
		1.0, /*hitlag*/
		1.0, /*sdi*/
		*ATTACK_SETOFF_KIND_ON, /*clang rebound*/
		*ATTACK_LR_CHECK_POS, /*facing restriction*/
		false, /*set weight*/
		0, /*shield damage*/
		0.35, /*trip chance*/
		0, /*rehit*/
		false, /*reflectable*/
		false, /*absorbable*/
		false, /*flinchless*/
		false, /*disable hitlag*/
		true, /*direct hitbox*/
		*COLLISION_SITUATION_MASK_GA, /*ground or air*/
		*COLLISION_CATEGORY_MASK_ALL, /*hitbits*/
		*COLLISION_PART_MASK_ALL, /*collision part*/
		false, /*friendly fire*/
		Hash40::new("collision_attr_normal"), /*effect*/
		*ATTACK_SOUND_LEVEL_S, /*sfx level*/
		*COLLISION_SOUND_ATTR_KICK, /*sfx type*/
		*ATTACK_REGION_KICK); /*type*/
		
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        kirby_attacklw3,
    );
}
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
        macros::ATTACK(fighter, // do not edit
		0, // id - 0
		0, // part - 1
		Hash40::new("toer"), // bone - 2
		6.0, // damage - 3
		361, // angle - 4
		43, 0, 30, // kbg, fkb, bkb - 5, 6, 7
		3.7, // size - 8 
		4.3, 0.0, 0.0, // position - 9, 10, 11
		None, None, None, // position 2 - 12, 13, 14
		1.0, // hitlag - 15
		1.0, // sdi - 16
		*ATTACK_SETOFF_KIND_ON, // clang rebound - 17
		*ATTACK_LR_CHECK_POS, // facing restriction - 18
		false, // set weight - 19
		0, // shield damage - 20
		0.35, // trip chance - 21
		0, // rehit - 22
		false, // reflectable - 23
		false, // absorbable - 24
		false, // flinchless - 25
		false, // disable hitlag - 26
		true, // direct hitbox - 27
		*COLLISION_SITUATION_MASK_GA, // ground or air - 28
		*COLLISION_CATEGORY_MASK_ALL, // hitbits - 29
		*COLLISION_PART_MASK_ALL, // collision part - 30
		false, // friendly fire - 31
		Hash40::new("collision_attr_normal"), // effect - 32
		*ATTACK_SOUND_LEVEL_S, // sfx level - 33
		*COLLISION_SOUND_ATTR_KICK, // sfx type - 34
		*ATTACK_REGION_KICK); // type - 35
		
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

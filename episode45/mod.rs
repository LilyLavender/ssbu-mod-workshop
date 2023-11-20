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
use smash::lib::L2CValue;

#[acmd_script( agent = "luigi_fireball", script = "game_regular", category = ACMD_GAME, low_priority )]
unsafe fn luigi_fireball_game_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 361, 40, 0, 80, 5.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

#[acmd_script( agent = "luigi_fireball", script = "effect_regular", category = ACMD_EFFECT, low_priority )]
unsafe fn luigi_fireball_effect_regular(agent: &mut L2CAgentBase) {
	if macros::is_excute(agent) {
		if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
			macros::EFFECT_FOLLOW(agent, Hash40::new("luigi_fb_bullet_l"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.6, true);
		} else {
			macros::EFFECT_FOLLOW(agent, Hash40::new("luigi_fb_bullet_r"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.6, true);
		}
		macros::LAST_EFFECT_SET_COLOR(agent, 1.5, 0.0, 0.0);
		for _ in 0..10 {
			macros::EFFECT_FOLLOW(agent, Hash40::new("sys_aura_dark"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 5.0, true);
			macros::LAST_EFFECT_SET_COLOR(agent, 0.5, 0.0, 0.0);
		}
	}
}

#[status_script(agent = "luigi_fireball", status = WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe extern "C" fn luigi_fireball_start_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
	StatusModule::init_settings(
		weapon.module_accessor, 
		smash::app::SituationKind(*SITUATION_KIND_AIR), 
		*WEAPON_KINETIC_TYPE_NORMAL, 
		GROUND_CORRECT_KIND_NONE.into(), 
		smash::app::GroundCliffCheckKind(0), 
		false, 
		0, 
		0, 
		0, 
		0
	);
	return 0.into();
}

#[status_script(agent = "luigi_fireball", status = WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn luigi_fireball_start_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
	MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
	weapon.fastshift(L2CValue::Ptr(luigi_fireball_start_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_fireball_start_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
	// Declare owner boma
	let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	// Declare facing
	let facing = PostureModule::lr(weapon.module_accessor);
	// Declare x and y speeds
	let energy_type = KineticModule::get_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;
	let mut speed_x: f32 = lua_bind::KineticEnergy::get_speed_x(energy_type);
	let mut speed_y: f32 = lua_bind::KineticEnergy::get_speed_y(energy_type);
	// Declare acceleration and max speed
	let accel_x: f32 = if facing == 1.0 { 0.04 } else { -0.04 };
	let accel_y: f32 = -0.1;
	let speed_max_x: f32 = if facing == 1.0 { 1.0 } else { -1.0 };
	let speed_max_y: f32 = 2.0;
	// Declare status_frame
	let status_frame = weapon.global_table[0xe].get_f32();
	// Get control stick y pos
	let stick_y = ControlModule::get_stick_y(owner_boma);
	
	// Add x speed until max speed is reached
	if speed_max_x > speed_x.abs() {
		speed_x += accel_x;
	}
	
	// Add y speed until max speed is reached
	if status_frame == 1.0 {
		speed_y = 2.0 + (stick_y + 1.0) / 2.0;
	}
	if speed_max_y > speed_x.abs() {
		speed_y += accel_y;
	}
	
	// Set speed
	weapon.agent.clear_lua_stack();
	weapon.agent.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
	weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_x));
	weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_y));
	sv_kinetic_energy::set_speed(weapon.lua_state_agent);
	return 0.into();
}

pub fn install() {
    install_status_scripts!(
		luigi_fireball_start_pre,
		luigi_fireball_start_main,
    );
	smashline::install_acmd_scripts!(
		luigi_fireball_game_regular,
		luigi_fireball_effect_regular,
    );
}

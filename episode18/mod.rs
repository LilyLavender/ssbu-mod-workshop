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

#[acmd_script( agent = "captain", script = "game_specialairsstart", category = ACMD_GAME, low_priority )]
unsafe fn captain_specialairsstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
		// Get left stick y position
		let yCtrl = ControlModule::get_stick_y(fighter.module_accessor);
		// Add speed based on left stick pos
		KineticModule::add_speed(fighter.module_accessor, &Vector3f{ x: 0.0, y: yCtrl * 3.0, z: 0.0 });
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BACK);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.1, 0.0, 12.0, 11.0, Some(0.0), Some(3.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_GRAVITY_ONOFF);
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[fighter_frame( agent = FIGHTER_KIND_CAPTAIN )]
fn captain_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

		if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) || 
		   ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) || 
		   ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) || 
		   ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
			
			if StatusModule::situation_kind(fighter.module_accessor) != SITUATION_KIND_GROUND {
				
				KineticModule::clear_speed_all(fighter.module_accessor);
				
			}
			
		}
		
	}
}

pub fn install() {
    smashline::install_acmd_scripts!(
        captain_specialairsstart,
    );
	smashline::install_agent_frames!(
		captain_frame,
    );
}

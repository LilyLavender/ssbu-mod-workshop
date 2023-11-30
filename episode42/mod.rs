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

pub const SITUATION_KIND:	i32 = 0x16;

#[status_script(agent = "szerosuit", status = FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_FLIP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn szerosuit_speciallwflip_main(fighter: &mut L2CFighterCommon) -> L2CValue {
	MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_flip"), 0.0, 1.0, false, 0.0, false, false);
	fighter.set_situation(SITUATION_KIND_AIR.into());
	GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
	WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
	WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
	WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
	WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
	WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
	WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
	WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_TREAD_JUMP);
	WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_WALL_JUMP);
	fighter.sub_shift_status_main(L2CValue::Ptr(szerosuit_speciallwflip_main_loop as *const () as _))
}

unsafe extern "C" fn szerosuit_speciallwflip_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
	if !CancelModule::is_enable_cancel(fighter.module_accessor) {
		if !MotionModule::is_end(fighter.module_accessor) {
			if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
				fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
			}
            // Code added to vanilla status
			if GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
				fighter.change_status(FIGHTER_STATUS_KIND_WALL_JUMP.into(), false.into());
			} 
		} else {
			fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
		}
	} else {
		if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
			if fighter.sub_air_check_fall_common().get_bool() {
				return 0.into();
			}
			if !MotionModule::is_end(fighter.module_accessor) {
				if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
					fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
				}
			} else {
				fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
			}
		}
	}
	return 0.into();
}

pub fn install() {
	install_status_scripts!(
        szerosuit_speciallwflip_main,
    );
}

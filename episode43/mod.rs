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

pub const SUB_STATUS:			i32 = 0x15;
pub const SITUATION_KIND:		i32 = 0x16;
pub const PREV_SITUATION_KIND:	i32 = 0x17;

#[status_script(agent = "metaknight", status = FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_SPIN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn metaknight_specialnspin_main(fighter: &mut L2CFighterCommon) -> L2CValue {	
	// In vanilla, this is set to the button_unable_frame param, which is 60
	WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_BUTTON_UNABLE_COUNTER);
	// Motion-Related
	MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_spin"), 0.0, 1.0, false, 0.0, false, false);
	let start_rot_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("start_rot_speed"));
	MotionModule::set_rate(fighter.module_accessor, start_rot_speed);
	
	// Effect-Related
	if !StopModule::is_stop(fighter.module_accessor) {
		FUN_710001ca30(false, fighter, false);
	}
	FUN_710001d850(fighter);
	
	// Speed-Related
	fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(metaknight_specialnspin_substatus as *const () as _));
	
	// Transition-Related
	fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_specialnspin_main_loop as *const () as _))
}

// Sub Status
unsafe extern "C" fn metaknight_specialnspin_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
	let add_speed_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("add_speed_stick")); // 0.8
	let start_stick_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("start_stick_speed")); // 1.5
	let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
	if stick_x.abs() >= add_speed_stick { // 0.8
		let facing = PostureModule::lr(fighter.module_accessor);
		//let final_x_speed = start_stick_speed * facing;
		let final_x_speed = start_stick_speed * stick_x * facing;
		KineticModule::add_speed(fighter.module_accessor, &Vector3f{ x: final_x_speed, y: 0.0, z: 0.0 });
	}
	return 0.into();
}

// Main Loop
unsafe extern "C" fn metaknight_specialnspin_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
	if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
		transitionIntoAerial(fighter);
	}
	
	let end_rot_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("end_rot_speed"));
	if MotionModule::rate(fighter.module_accessor) <= end_rot_speed {
		fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_END.into(), false.into());
	}
	
	if !StatusModule::is_changing(fighter.module_accessor) {
		if fighter.global_table[PREV_SITUATION_KIND] != *SITUATION_KIND_GROUND {
			if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
				FUN_710001d850(fighter);
				return 0.into();
			}
		} else {
			if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_AIR {
				FUN_710001d850(fighter);
				return 0.into();
			}
		}
	}
	
	return 0.into();
}

unsafe extern "C" fn FUN_710001d850(fighter: &mut L2CFighterCommon) {
	if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
		GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
		KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_AIR_N);
	} else {
		GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
		KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_N);
		WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_FLOAT_GROUND_EFFECT_COUNTER);
	}
	return;
}

unsafe extern "C" fn FUN_710001ca30(param_1: bool, fighter: &mut L2CFighterCommon, param_3: bool) {
	WorkModule::inc_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_BUTTON_HOP_COUNT);
	WorkModule::dec_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE);
	let motion_kind = MotionModule::motion_kind(fighter.module_accessor);

	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE) <= 0 {
		WorkModule::set_int(fighter.module_accessor, 5, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE);
		
		// Todo: optimize this bit
		let ss_se_counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE_COUNTER);
		if ss_se_counter != 0 {
			if ss_se_counter != 1 {
				if ss_se_counter != 2 {
					if ss_se_counter != 3 {
						if ss_se_counter != 4 {
							if ss_se_counter != 5 {
								if ss_se_counter != 6 {
									if ss_se_counter != 7 {
										if ss_se_counter != 8 {
											if ss_se_counter != 9 {
												SoundModule::play_se(fighter.module_accessor, Hash40::new("se_metaknight_swish05"), true, false, false, false, smash::app::enSEType(0));
											} else {
												SoundModule::play_se(fighter.module_accessor, Hash40::new("se_metaknight_swish05"), true, false, false, false, smash::app::enSEType(0));
											}
										} else {
											SoundModule::play_se(fighter.module_accessor, Hash40::new("se_metaknight_swish06"), true, false, false, false, smash::app::enSEType(0));
										}
									} else {
										SoundModule::play_se(fighter.module_accessor, Hash40::new("se_metaknight_swish06"), true, false, false, false, smash::app::enSEType(0));
									}
								} else {
									if motion_kind != hash40("special_n_spin_c3") {
										SoundModule::play_se(fighter.module_accessor, Hash40::new("se_metaknight_swish07"), true, false, false, false, smash::app::enSEType(0));
									} else { 
										SoundModule::play_se(fighter.module_accessor, smash::phx::Hash40 { hash: 0x1896dcd23e }, true, false, false, false, smash::app::enSEType(0));
									}
								}
							} else {
								if motion_kind != hash40("special_n_spin_c3") {
									SoundModule::play_se(fighter.module_accessor, Hash40::new("se_metaknight_swish09"), true, false, false, false, smash::app::enSEType(0));
								} else {
									SoundModule::play_se(fighter.module_accessor, smash::phx::Hash40 { hash: 0x187603a50d }, true, false, false, false, smash::app::enSEType(0));
								}
							}
						} else {
							SoundModule::play_se(fighter.module_accessor, Hash40::new("se_metaknight_swish10"), true, false, false, false, smash::app::enSEType(0));
						}
					} else {
						if motion_kind != hash40("special_n_spin_c3") {
							SoundModule::play_se(fighter.module_accessor, Hash40::new("se_metaknight_swish11"), true, false, false, false, smash::app::enSEType(0));
						} else {
							SoundModule::play_se(fighter.module_accessor, smash::phx::Hash40 { hash: 0x188ed7a452 }, true, false, false, false, smash::app::enSEType(0));
						}
					}
				} else {
					if motion_kind != hash40("special_n_spin_c3") {
						SoundModule::play_se(fighter.module_accessor, Hash40::new("se_metaknight_swish11"), true, false, false, false, smash::app::enSEType(0));
					} else {
						SoundModule::play_se(fighter.module_accessor, smash::phx::Hash40 { hash: 0x188ed7a452 }, true, false, false, false, smash::app::enSEType(0));
					}
				}
			} else {
				if motion_kind != hash40("special_n_spin_c3") {
					SoundModule::play_se(fighter.module_accessor, Hash40::new("se_metaknight_swish09"), true, false, false, false, smash::app::enSEType(0));
				} else {
					SoundModule::play_se(fighter.module_accessor, smash::phx::Hash40 { hash: 0x187603a50d }, true, false, false, false, smash::app::enSEType(0));
				}
			}
		} else {
			if motion_kind != hash40("special_n_spin_c3") {
				SoundModule::play_se(fighter.module_accessor, Hash40::new("se_metaknight_swish07"), true, false, false, false, smash::app::enSEType(0));
			} else {
				SoundModule::play_se(fighter.module_accessor, smash::phx::Hash40 { hash: 0x1896dcd23e }, true, false, false, false, smash::app::enSEType(0));
			}
		}
		WorkModule::inc_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE_COUNTER);
	}
	
	if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
	
		let aLStack128 = WorkModule::get_float(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_FLOAT_GROUND_EFFECT_COUNTER);
		let LStack160 = MotionModule::rate(fighter.module_accessor);
		
		let fVar8 = aLStack128 - LStack160;

		WorkModule::set_float(fighter.module_accessor, fVar8, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_FLOAT_GROUND_EFFECT_COUNTER);
		if fVar8 <= 0.0 {
			FighterSpecializer_Metaknight::set_special_n_ground_effect(fighter.module_accessor as *mut Fighter);
		}
	}
}

unsafe extern "C" fn transitionIntoAerial(fighter: &mut L2CFighterCommon) {
	if PostureModule::lr(fighter.module_accessor) > 0.0 {
		if ControlModule::get_stick_y(fighter.module_accessor) > 0.707 {
			ControlModule::set_attack_air_kind(fighter.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI);
		} else {
			if ControlModule::get_stick_y(fighter.module_accessor) < -0.707 {
			ControlModule::set_attack_air_kind(fighter.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW);
			} else {
				if ControlModule::get_stick_x(fighter.module_accessor) > 0.707 {
					ControlModule::set_attack_air_kind(fighter.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_F);
				} else {
					if ControlModule::get_stick_x(fighter.module_accessor) < -0.707 {
						ControlModule::set_attack_air_kind(fighter.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_B);
					} else {
						ControlModule::set_attack_air_kind(fighter.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_N);
					}
				}
			}
		}
		StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
	} else {
		if ControlModule::get_stick_y(fighter.module_accessor) > 0.707 {
			ControlModule::set_attack_air_kind(fighter.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI);
		} else {
			if ControlModule::get_stick_y(fighter.module_accessor) < -0.707 {
			ControlModule::set_attack_air_kind(fighter.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW);
			} else {
				if ControlModule::get_stick_x(fighter.module_accessor) < -0.707 {
					ControlModule::set_attack_air_kind(fighter.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_F);
				} else {
					if ControlModule::get_stick_x(fighter.module_accessor) > 0.707 {
						ControlModule::set_attack_air_kind(fighter.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_B);
					} else {
						ControlModule::set_attack_air_kind(fighter.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_N);
					}
				}
			}
		}
		StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
	}
}

pub fn install() {
	install_status_scripts!(
        metaknight_specialnspin_main,
    );
}

use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CAgent, L2CValue},
        hash40
    },
    smash_script::*,
    smashline::*
};

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
        FUN_710001ca30(fighter, false.into());
    }
    FUN_710001d850(fighter);
    
    // Speed-Related
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(FUN_710001ca30 as *const () as _));
    
    // Starting Horizontal Speed
    let add_speed_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("add_speed_stick")); // 0.8
    let start_stick_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("start_stick_speed")); // 1.5
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    if stick_x.abs() >= add_speed_stick { // 0.8
        let facing = PostureModule::lr(fighter.module_accessor);
        //let final_x_speed = start_stick_speed * facing;
        let final_x_speed = start_stick_speed * stick_x * facing;
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{ x: final_x_speed, y: 0.0, z: 0.0 });
    }

    // Added transition term to transition into Aerials
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);

    // Transition-Related
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_specialnspin_main_loop as *const () as _))
}

// Main Loop
unsafe extern "C" fn metaknight_specialnspin_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // The vanilla transition check into aerials
    if fighter.sub_transition_group_check_air_attack().get_bool() {
        return 0.into();
    }
    
    let end_rot_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("end_rot_speed"));
    if MotionModule::rate(fighter.module_accessor) <= end_rot_speed {
        fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_END.into(), false.into());
    }
    
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        FUN_710001d850(fighter);
        return 0.into();
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

// Substatus
unsafe extern "C" fn FUN_710001ca30(fighter: &mut L2CFighterCommon, param_3: L2CValue) -> L2CValue {
    if param_3.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_BUTTON_HOP_COUNT);
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE) <= 0 {
            WorkModule::set_int(fighter.module_accessor, 5, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE);
            
            let se_counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE_COUNTER);
            let se = match se_counter {
                0 => { if motion_kind == hash40("special_n_spin_c3") { 0x1896dcd23e } else { hash40("se_metaknight_swish07") } },
                1 => { if motion_kind == hash40("special_n_spin_c3") { 0x187603a50d } else { hash40("se_metaknight_swish09") } },
                2 => { if motion_kind == hash40("special_n_spin_c3") { 0x188ed7a452 } else { hash40("se_metaknight_swish11") } },
                3 => { if motion_kind == hash40("special_n_spin_c3") { 0x188ed7a452 } else { hash40("se_metaknight_swish11") } },
                4 => hash40("se_metaknight_swish10"),
                5 => { if motion_kind == hash40("special_n_spin_c3") { 0x187603a50d } else { hash40("se_metaknight_swish09") } },
                6 => { if motion_kind == hash40("special_n_spin_c3") { 0x1896dcd23e } else { hash40("se_metaknight_swish07") } },
                7 => hash40("se_metaknight_swish06"),
                8 => hash40("se_metaknight_swish06"),
                9 => hash40("se_metaknight_swish05"),
                _ => hash40("se_metaknight_swish05"),
            };
            SoundModule::play_se(fighter.module_accessor, Hash40::new_raw(se), true, false, false, false, enSEType(0));
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE_COUNTER);
        }
        
        if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
        
            let aLStack128 = WorkModule::get_float(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_FLOAT_GROUND_EFFECT_COUNTER);
            let LStack160 = MotionModule::rate(fighter.module_accessor);
            
            let fVar8 = aLStack128 - LStack160;
    
            WorkModule::set_float(fighter.module_accessor, fVar8, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_FLOAT_GROUND_EFFECT_COUNTER);
            if fVar8 <= 0.0 {
                let fighta = fighter.global_table[0x4].get_ptr() as *mut Fighter;
                FighterSpecializer_Metaknight::set_special_n_ground_effect(fighta);
            }
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        metaknight_specialnspin_main,
    );
}

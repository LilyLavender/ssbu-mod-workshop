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

#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = fighter.module_accessor;
        // Param declarations
        let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
        let wall_jump_type = WorkModule::get_param_int(boma, hash40("wall_jump_type"), 0) != 0;
        // Status-related declarations
        let status_kind = StatusModule::status_kind(boma);
        let status_frame = fighter.global_table[0xe].get_f32();
        
        // Characters with a run speed of over 2.2 get impact run
        if run_speed_max > 2.2 {
            impactRun(fighter, status_kind, status_frame);
        }
        
        // Characters that can wall jump start at 50%
        if wall_jump_type && (status_kind == *FIGHTER_STATUS_KIND_ENTRY || status_kind ==  *FIGHTER_STATUS_KIND_REBIRTH) && status_frame == 1.0 {
            DamageModule::add_damage(fighter.module_accessor, 50.0, 0);
        }
    }
}

unsafe extern "C" fn impactRun(fighter: &mut L2CFighterCommon, status_kind: i32, status_frame: f32) {
    let boma = fighter.module_accessor;
    // Param declaration
    let half_height = WorkModule::get_param_float(boma, hash40("height"), 0) / 2.0;
    
    if status_kind == *FIGHTER_STATUS_KIND_RUN {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 80, 0, 50, 15.0, 0.0, half_height, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        if status_frame == 1.0 {
            macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("sys_speedbooster"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, true, WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR));
        }
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame
    );
}
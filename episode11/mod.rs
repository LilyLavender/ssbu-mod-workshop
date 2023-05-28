use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
		hash40
    },
    smash_script::*,
    smashline::*,
};

#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
		let boma2 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
		if DamageModule::damage(boma2, 0) > 80.0 {
			DamageModule::add_damage(boma2, -DamageModule::damage(boma2, 0), 0);
		}
    }
}

#[fighter_frame( agent = FIGHTER_KIND_KAMUI )]
fn kamui_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		if(MotionModule::motion_kind(fighter.module_accessor) == hash40("fall_special")) {
			fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
			WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
		}
		macros::ATTACK(fighter, 0, 1, Hash40::new("haver"), 10.5, 45, 100, 0, 30, 5.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 0, 2, Hash40::new("haver"), 10.5, 45, 100, 0, 30, 4.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 0, 3, Hash40::new("top"), 10.5, 45, 100, 0, 30, 4.0, 0.0, 8.0, 9.0, Some(0.0), Some(8.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
		global_fighter_frame
    );
    smashline::install_agent_frames!(
		kamui_frame,
    );
}

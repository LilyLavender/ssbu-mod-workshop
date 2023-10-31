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
static mut currDamage: [f32; 8] = [0.0; 8];

#[fighter_frame( agent = FIGHTER_KIND_ROBOT )]
fn robot_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if currDamage[entry_id] != DamageModule::damage(boma, 0) {
			println!();
			println!();
			println!("<|°_°|> Player {}", entry_id - 1);
			println!("Damage changed from {} to {}", currDamage[entry_id], DamageModule::damage(boma, 0));
			println!("current position: {}, {}", PostureModule::pos_x(boma), PostureModule::pos_y(boma));
			println!("gyro charge value: {}", WorkModule::get_float(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_GYRO_CHARGE_VALUE));
			currDamage[entry_id] = DamageModule::damage(boma, 0)
		}
	}
}

pub fn install() {
    smashline::install_agent_frames!(
		robot_frame,
    );
}

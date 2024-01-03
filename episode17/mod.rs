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

#[fighter_frame( agent = FIGHTER_KIND_MEWTWO )]
fn mewtwo_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
		if DamageModule::damage(fighter.module_accessor, 0) == 0.0 {
			// reset all
			AttackModule::set_power_up(fighter.module_accessor, 1.0); 
			AttackModule::set_reaction_mul(fighter.module_accessor, 1.0); 
			DamageModule::set_damage_mul(fighter.module_accessor, 1.0); 
			DamageModule::set_reaction_mul(fighter.module_accessor, 1.0); 
			smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 1.0); 
		} else if DamageModule::damage(fighter.module_accessor, 0) == 1.0 {
			AttackModule::set_power_up(fighter.module_accessor, 0.5); // do x0.5 damage
		} else if DamageModule::damage(fighter.module_accessor, 0) == 2.0 {
			AttackModule::set_reaction_mul(fighter.module_accessor, 0.5); // do x0.5 knockback
		} else if DamageModule::damage(fighter.module_accessor, 0) == 3.0 {
			DamageModule::set_damage_mul(fighter.module_accessor, 0.5); // take x0.5 damage
		} else if DamageModule::damage(fighter.module_accessor, 0) == 4.0 {
			DamageModule::set_reaction_mul(fighter.module_accessor, 0.5); // take x0.5 knockback
		} else if DamageModule::damage(fighter.module_accessor, 0) == 5.0 {
			AttackModule::set_power_up(fighter.module_accessor, 2.0); // do x2 damage
		} else if DamageModule::damage(fighter.module_accessor, 0) == 6.0 {
			AttackModule::set_reaction_mul(fighter.module_accessor, 2.0); // do x2 knockback
		} else if DamageModule::damage(fighter.module_accessor, 0) == 7.0 {
			DamageModule::set_damage_mul(fighter.module_accessor, 2.0); // take x2 damage
		} else if DamageModule::damage(fighter.module_accessor, 0) == 8.0 {
			DamageModule::set_reaction_mul(fighter.module_accessor, 2.0); // take x2 knockback
		} else if DamageModule::damage(fighter.module_accessor, 0) == 9.0 {
			smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 0.5); // move x2 slower
		} else if DamageModule::damage(fighter.module_accessor, 0) == 10.0 {
			smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 2.0); // move x2 faster
		}
	}
}

pub fn install() {
    smashline::install_agent_frames!(
		mewtwo_frame
    );
}

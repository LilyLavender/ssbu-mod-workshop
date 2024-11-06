use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

// AttackModule - Dealt
// DamageModule - Taken
// power/damage - Damage
// reaction_mul - Knockback

unsafe extern "C" fn mewtwo_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        if DamageModule::damage(fighter.module_accessor, 0) == 0.0 {
            // Reset all multipliers
            AttackModule::set_power_up(fighter.module_accessor, 1.0); 
            AttackModule::set_reaction_mul(fighter.module_accessor, 1.0); 
            DamageModule::set_damage_mul(fighter.module_accessor, 1.0); 
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0); 
            smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 1.0); 
        } else if DamageModule::damage(fighter.module_accessor, 0) == 1.0 {
            AttackModule::set_power_up(fighter.module_accessor, 0.5); // Do x0.5 damage
        } else if DamageModule::damage(fighter.module_accessor, 0) == 2.0 {
            AttackModule::set_reaction_mul(fighter.module_accessor, 0.5); // Do x0.5 knockback
        } else if DamageModule::damage(fighter.module_accessor, 0) == 3.0 {
            DamageModule::set_damage_mul(fighter.module_accessor, 0.5); // Take x0.5 damage
        } else if DamageModule::damage(fighter.module_accessor, 0) == 4.0 {
            DamageModule::set_reaction_mul(fighter.module_accessor, 0.5); // Take x0.5 knockback
        } else if DamageModule::damage(fighter.module_accessor, 0) == 5.0 {
            AttackModule::set_power_up(fighter.module_accessor, 2.0); // Do x2 damage
        } else if DamageModule::damage(fighter.module_accessor, 0) == 6.0 {
            AttackModule::set_reaction_mul(fighter.module_accessor, 2.0); // Do x2 knockback
        } else if DamageModule::damage(fighter.module_accessor, 0) == 7.0 {
            DamageModule::set_damage_mul(fighter.module_accessor, 2.0); // Take x2 damage
        } else if DamageModule::damage(fighter.module_accessor, 0) == 8.0 {
            DamageModule::set_reaction_mul(fighter.module_accessor, 2.0); // Take x2 knockback
        } else if DamageModule::damage(fighter.module_accessor, 0) == 9.0 {
            smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 0.5); // Move x2 slower
        } else if DamageModule::damage(fighter.module_accessor, 0) == 10.0 {
            smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 2.0); // Move x2 faster
        }
    }
}

pub fn install() {
    Agent::new("mewtwo")
        .on_line(Main, mewtwo_frame)
        .install();
}
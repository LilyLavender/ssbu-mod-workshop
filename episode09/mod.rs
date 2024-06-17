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

unsafe extern "C" fn zelda_game_appealhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) { // A
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 14.5, 88, 5, 0, 102, 100.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        } else if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) { // B
            EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_zelda_final"), false, false, false);
        } else if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_JUMP) { // Jump
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_dead"), Hash40::new("top"), 0, 0, -1, 0, 0, 0, 2.0, true);
        } else if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) { // Dpad up
            DamageModule::add_damage(agent.module_accessor, 10.0, 0);
        } else if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) { // Dpad down
            DamageModule::heal(agent.module_accessor, -10.0, 0);
        } else if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) { // Dpad left
            DamageModule::add_damage(agent.module_accessor, 3.0, 0);
        } else if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) { // Dpad right
            DamageModule::heal(agent.module_accessor, -3.0, 0);
        }
        
        let xpos = ControlModule::get_stick_x(agent.module_accessor);
        let ypos = ControlModule::get_stick_y(agent.module_accessor);
        if xpos < 0.0 && ypos < 0.0 {
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 0.0, 88, 5, 0, 102, 10000.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("zelda")
        .game_acmd("game_appealhil", zelda_game_appealhi, Default)
        .game_acmd("game_appealhir", zelda_game_appealhi, Default)
        .install();
}
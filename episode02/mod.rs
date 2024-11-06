
// rockman_game_attackairf is the name of the function you're defining. It can be whatever you'd like, but charname_type_scriptname is a common and helpful rule to follow
unsafe extern "C" fn rockman_game_attackairf(agent: &mut L2CAgentBase) {

    // The frame that all of the next actions will be processed on, in this example, 3
    frame(agent.lua_state_agent, 3.0);
    // macros:is_execute guarantees that whatever's inside of it will only be executed once
    if macros::is_excute(agent) {
        // Turn a flag on
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    // Frame 9
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        // Summon hitboxes (more about hitboxes is covered in episode 3)
        macros::ATTACK(agent, 0, 0, Hash40::new("havel"), 8.5, 50, 118, 0, 48, 4.0, 0.0, 11.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("havel"), 8.0, 50, 118, 0, 48, 4.0, 0.0, 7.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("havel"), 8.0, 50, 118, 0, 48, 4.0, 0.0, 2.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }

    // Wait 3 frames from the last frame declaration
    // In this case, this code will run on frame 12
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        // Because these hitboxes have the same ID and part, they will override the previous ones
        macros::ATTACK(agent, 0, 0, Hash40::new("havel"), 5.0, 361, 100, 0, 12, 3.0, 0.0, 10.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("havel"), 5.0, 361, 100, 0, 12, 3.0, 0.0, 5.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("havel"), 5.0, 361, 100, 0, 12, 3.0, 0.0, 2.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }

    // Frame 18
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        // Remove all hitboxes
        AttackModule::clear_all(agent.module_accessor);
    }

    // Frame 39
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        // Turn flag off
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

pub fn install() {
    Agent::new("rockman")
        .game_acmd("game_attackairf", rockman_game_attackairf, Default)
        .install();
}

// pub fn install() = The install section. This is where you install scripts on agents

// Agent::new("rockman") = The fighter you're editing.
    // Some characters use different names, such as megaman, who uses "rockman" instead. 
    // A full list can be found at episode02/FighterCodeNames.md

// .game_acmd = The type of script. 
    // Either game_acmd, effect_acmd, sound_acmd, or expression_acmd

// "game_attackairf" = The name of the script you're editing
    // A list of common scripts can be found at episode02/CommonAttackNames.md
    // A full list of script names can be found here: 
    // https://docs.google.com/spreadsheets/d/1q_TpWoQkr9YWgQ7fc3JpHuU9zKfCLtl80Uodcyc0NPY/edit#gid=0

// rockman_game_attackairf = The function name defined previously

// Default = Priority
    // Can either be Low, Default, or High. Best to keep as Default

// Make sure to end each agent with .install();
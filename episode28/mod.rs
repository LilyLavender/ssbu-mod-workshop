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

#[skyline::hook(replace = sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    // Declare boma and fighter_kind from lua_state parameter
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = smash::app::utility::get_kind(boma);
    
    // Declare l2c_agent from lua_state parameter
    let mut l2c_agent = L2CAgent::new(lua_state);
    // Declare hitbox_params as a growable array from stack we're getting from lua_state
    let hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
    // Clear stack
    l2c_agent.clear_lua_stack();
    // Repeat 36 times
    for i in 0..36 {
        if fighter_kind == FIGHTER_KIND_BUDDY {
            
            // If index of for loop is a specific number, set it to a certain value
            if i == 3 {
                l2c_agent.push_lua_stack(&mut L2CValue::new_num(10.0));
            } else if i == 7 {
                l2c_agent.push_lua_stack(&mut L2CValue::new_int(160));
            } else if i == 32 {
                l2c_agent.push_lua_stack(&mut L2CValue::new_int(hash40("collision_attr_ice")));
            }
            // If index isn't one of the aforementioned values, use the values originally passed into the function
            else {
                l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
            }
            
        } else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
        }
    }
    original!()(lua_state);
}

pub fn install() {
    skyline::install_hooks!(
        attack_replace
    );
}
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
use smash::lib::L2CValue;
use smash::lib::L2CAgent;

#[skyline::hook(replace = sv_animcmd::AFTER_IMAGE4_ON_arg29)]
unsafe fn after_image4_on_arg29_replace(lua_state: u64) {
	let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
	let fighter_kind = smash::app::utility::get_kind(boma);
	if fighter_kind == FIGHTER_KIND_EDGE && WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
		
		let mut l2c_agent = L2CAgent::new(lua_state);
		let hitbox_params: Vec<L2CValue> = (0..29).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
		l2c_agent.clear_lua_stack();
		for i in 0..29 {
			if i == 0 {
				// Hash is tex_edge_sword1, this method is reqired because 0x0fd3b5c426 is too big to use outside of a statment such as this
				if hitbox_params[0].get_int() == L2CValue::new_int(0x0fd3b5c426).get_int() { 
					l2c_agent.push_lua_stack(&mut L2CValue::new_int(0x155d2ab4d2)); // hash for tex_edge_sword1_blood
				} else { // Because the only other time sephiroth uses ai4a29 is to call sword3, this is essentially just checking for that
					l2c_agent.push_lua_stack(&mut L2CValue::new_int(0x15cab5a5fb)); // hash for tex_edge_sword3_blood
				}
			} else {
				l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
			}
			
		} 
    }
    original!()(lua_state);
}

pub fn install() {
    skyline::install_hooks!(
        after_image4_on_arg29_replace
    );
}

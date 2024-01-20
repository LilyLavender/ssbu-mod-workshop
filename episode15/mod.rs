use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::*
};

static mut gaoMode: [i32; 8] = [0; 8];

#[fighter_frame( agent = FIGHTER_KIND_GAOGAEN )]
fn gaogaen_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_r") || 
        MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_l") {
            if MotionModule::frame(fighter.module_accessor) > 10.0 && MotionModule::frame(fighter.module_accessor) < 20.0 {
                
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) { // Dpad up
                    gaoMode[entry_id] = 0;
                } else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) { // Dpad down
                    gaoMode[entry_id] = 1;
                } else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) { // Dpad left
                    gaoMode[entry_id] = 2;
                } else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) { // Dpad right
                    gaoMode[entry_id] = 3;
                }
                
            }   
        }
        
        if gaoMode[entry_id] == 0 { // Clear effects
            macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0); 
        } else if gaoMode[entry_id] == 1 { // Invincibility (Moves connect, don't do damage)
            macros::WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE); 
        } else if gaoMode[entry_id] == 2 { // Intangibily (moves can't connect)
            macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU); 
        } else if gaoMode[entry_id] == 3 { // Super Armor
            // DAMAGE_NO_REACTION_MODE_ALWAYS means that the armor will never break. To do damage-based armor, set the 3rd argument to "DAMAGE_NO_REACTION_MODE_DAMAGE_POWER" and the 4th to the max damage of the armor.
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0); 
        }
        
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        gaogaen_frame,
    );
}

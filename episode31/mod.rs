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

#[fighter_frame( agent = FIGHTER_KIND_PEACH )]
fn peach_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = fighter.global_table[0xb].get_i32();
        let status_frame = fighter.global_table[0xe].get_i32();
        let stick_y = fighter.global_table[0x1b].get_f32();
        
        // If at least 32 frames into up b, and left joystick is pointed up, transition into float.
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && status_frame >= 32 && stick_y > 0.707 {
            fighter.change_status(FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START.into(), true.into());
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        peach_frame
    );
}

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
use skyline::hooks::{getRegionAddress, Region};

static mut INT_OFFSET : usize = 0x4ded80;
static mut FLOAT_OFFSET : usize = 0x4dedc0;

#[skyline::hook(offset=INT_OFFSET)]
pub unsafe fn int_param_accessor_hook(
boma: u64,
param_type: u64,
param_hash: u64) -> i32 {
    let ret = original!()(boma, param_type, param_hash);
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if param_hash == 0 {
        
        if fighter_kind == FIGHTER_KIND_SHULK {
            if color == 1 {
                if param_type == hash40("jump_squat_frame") {
                    return 20;
                } else if param_type == hash40("attach_wall_type") {
                    return 1;
                } 
            } 
        }
    
    }
    ret
}

#[skyline::hook(offset=FLOAT_OFFSET)]
pub unsafe fn float_param_accessor_hook(
boma: u64,
param_type: u64,
param_hash: u64) -> f32 {
    let ret = original!()(boma, param_type, param_hash);
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if param_hash == 0 {
        
        if fighter_kind == FIGHTER_KIND_SHULK {
            if color == 1 {
                if param_type == hash40("jump_y") {
                    return 100.0;
                } else if param_type == hash40("weight") {
                    return 10000.0;
                } else if param_type == hash40("shield_break_y") {
                    return 100.0;
                } else if param_type == hash40("landing_attack_air_frame_f") {
                    return 100.0;
                }
            }
        }
        
    }
    ret
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];

static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];

pub fn install() {
    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
    }
    skyline::install_hooks!(
        int_param_accessor_hook,
        float_param_accessor_hook
    );
}
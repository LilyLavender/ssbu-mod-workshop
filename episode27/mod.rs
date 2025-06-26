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
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;

static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x67a7b0;
const FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEARCH_HIT : i32 = 0x200000EB;

unsafe extern "C" fn ryu_game_attackairf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    }
    macros::FT_MOTION_RATE(agent, 1.34);
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        // Added search_hit on_flag
        WorkModule::on_flag(agent.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
    }
    frame(agent.lua_state_agent, 5.5);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 61, 56, 0, 82, 4.3, 0.0, 5.5, 10.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 70, 25, 0, 65, 3.8, 0.0, 5.5, 2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(agent, 0.6);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 61, 56, 0, 82, 4.0, 0.0, 5.5, 10.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 70, 25, 0, 65, 4.6, 0.0, 5.5, 2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        // Added search_hit ff_flag
        WorkModule::off_flag(agent.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    macros::FT_MOTION_RATE(agent, 1.15);
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_id: u32, defender_id: u32, damage: f32, attack_kind: i32, arg6: bool) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_id);
    let defender_boma = sv_battle_object::module_accessor(defender_id);
    let attacker_kind = sv_battle_object::kind(attacker_id);
    let defender_kind = sv_battle_object::kind(defender_id);
    // If search_hit flag is on
    if WorkModule::is_flag(attacker_boma, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEARCH_HIT) {
        // Add velocity
        KineticModule::add_speed(attacker_boma, &Vector3f{ x: -3.0, y: 3.0, z: 0.0 });
        // Disable flag
        WorkModule::off_flag(attacker_boma, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
        // If thing being hit is a fighter
        if utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            // Give fighter being hit sticky bomb
            ItemModule::have_item(defender_boma, smash::app::ItemKind(*ITEM_KIND_CHEWING), 0, 0, false, false);
        }
    }
    
    original!()(fighter_manager, attacker_id, defender_id, damage, attack_kind, arg6)
}
/*
    attacker_id:    object_id of attacker
    defender_id:    object_id of defender
    damage:         damage dealt by the hit
    attack_kind:    FIGHTER_LOG_ACTION_KIND_... constant, for the "kind" of move done
    arg6:           unknown
*/

unsafe extern "C" fn ryu_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        
        if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_air_f") {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
        }
        
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static OFFSET_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1, //.text:0000007100675A20                 SUB             SP, SP, #0xC0
    0xe8, 0x2b, 0x00, 0xfd, //.text:0000007100675A24                 STR             D8, [SP,#0xB0+var_60]
    0xfc, 0x6f, 0x06, 0xa9, //.text:0000007100675A28                 STP             X28, X27, [SP,#0xB0+var_50]
    0xfa, 0x67, 0x07, 0xa9, //.text:0000007100675A2C                 STP             X26, X25, [SP,#0xB0+var_40]
    0xf8, 0x5f, 0x08, 0xa9, //.text:0000007100675A30                 STP             X24, X23, [SP,#0xB0+var_30]
    0xf6, 0x57, 0x09, 0xa9, //.text:0000007100675A34                 STP             X22, X21, [SP,#0xB0+var_20]
    0xf4, 0x4f, 0x0a, 0xa9, //.text:0000007100675A38                 STP             X20, X19, [SP,#0xB0+var_10]
    0xfd, 0x7b, 0x0b, 0xa9, //.text:0000007100675A3C                 STP             X29, X30, [SP,#0xB0+var_s0]
    0xfd, 0xc3, 0x02, 0x91, //.text:0000007100675A40                 ADD             X29, SP, #0xB0
    0xfb, 0x03, 0x00, 0xaa  //.text:0000007100675A44                 MOV             X27, X0
];

pub fn install() {
    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, OFFSET_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }
    skyline::install_hook!(
        notify_log_event_collision_hit_replace
    );
    Agent::new("ryu")
        .game_acmd("game_attackairf", ryu_game_attackairf, Default)
        .on_line(Main, ryu_frame)
        .install();
}

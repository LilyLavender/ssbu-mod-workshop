use {
	smash::{
		lua2cpp::*,
		phx::*,
		app::{sv_animcmd::*, lua_bind::*, *},
		lib::lua_const::*,
		hash40
	},
	smash_script::*,
	smashline::*
};
use smash::lib::L2CValue;

pub const KIND:						i32 = 0x2;
pub const CMD_CAT4:					i32 = 0x23;
pub const CHECK_SPECIAL_COMMAND:	i32 = 0x3C;

const FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SPECIAL_S_COMMAND : i32 = 0x200000f0;

// Agent init - Runs once when a fighter starts existing
#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
	unsafe {
		let fighter_kind = fighter.global_table[KIND].get_i32();
		if fighter_kind == *FIGHTER_KIND_WOLF {
			// Turning this flag on allows a character to do command inputs
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
			// Assigning our own defined function to the correct spot in the global_table
			fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(wolf_check_special_command as *const () as _));
		}
	}
}

unsafe extern "C" fn wolf_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
	let mut ret = false;
	let cat4 = fighter.global_table[CMD_CAT4].get_i32();
	
	// Side B
	// If player did a qcf input
	// A full list of inputs can be found in the other file in the episode45 folder.
	if !ret && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
	// If fighter is allowed to transition into side b (ie, not in hitstun, not doing another attack)
	&& WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND) {
		WorkModule::on_flag(fighter.module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SPECIAL_S_COMMAND);
		fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), false.into());
		ret = true;
	}
	
	// Up B
	// If player did a [2]8A/B input
	if !ret && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI2_COMMAND != 0
	// If fighter is allowed to transition into up b
	&& WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND) {
		fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
		ret = true;
	}
	
	// Return bool
	ret.into()
}

// Add a section to a status script without having to translate it
#[status_script(agent = "wolf", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn wolf_special_s_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SPECIAL_S_COMMAND) {
		AttackModule::set_power_up(fighter.module_accessor, 1.5);
		WorkModule::off_flag(fighter.module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SPECIAL_S_COMMAND)
	}
	original!(fighter)
}

pub fn install() {
	install_agent_init_callback!(
		agent_init
	);
	install_status_scripts!(
		wolf_special_s_start_main,
	);
}
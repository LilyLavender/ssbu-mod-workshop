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

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
	unsafe {
		let fighter_kind = fighter.global_table[KIND].get_i32();
		if fighter_kind == *FIGHTER_KIND_WOLF {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
			fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(wolf_check_special_command as *const () as _));
		}
	}
}

unsafe extern "C" fn wolf_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
	let mut ret = false;
	let cat4 = fighter.global_table[CMD_CAT4].get_i32();
	
	// Side B
	if !ret && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
	&& WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND) {
		WorkModule::on_flag(fighter.module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SPECIAL_S_COMMAND);
		fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), false.into());
		ret = true;
	}
	
	// Up B
	if !ret && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI2_COMMAND != 0
	&& WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND) {
		fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
		ret = true;
	}
	
	// Return bool
	ret.into()
}

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

// List from black_calculus
/*
FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND = Quarter Circle Forward, Hadoken
FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND = Half Circle Forward, Shakunetsu
FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND = Quarter Circle Back, Tatsumaki
FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND = Z input, Shoryuken
FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6 = Double tap forward
FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_4N4 = Double tap backward
FIGHTER_PAD_CMD_CAT4_FLAG_ATTACK_COMMAND1 = Quarter circle down, Ken's Oosoto Mawashi Geri
FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND = Dobule Quarter Circle Forward, Buster Wolf
FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND = Half Circle Back -> Forward, Power Geyser
FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI2_COMMAND = Charge down -> Up, Rising Tackle
*/

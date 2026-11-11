#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	non_upper_case_globals,
	non_snake_case,
    clippy::borrow_interior_mutable_const
)]

use smash::lib::lua_const::*;

pub static mut FIGHTER_CHROM_GENERATE_ARTICLE_LIGHTBEAM: i32 = -1;

mod chrom;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    let lightbeam = smashline::clone_weapon("chrom", "lightbeam", *WEAPON_KIND_MARIO_FIREBALL, true);
    unsafe { FIGHTER_CHROM_GENERATE_ARTICLE_LIGHTBEAM = lightbeam.generate_id; }

    chrom::install();
}

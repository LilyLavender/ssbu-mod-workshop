# Smash Ultimate Modding Workshop
The Smash Ultimate Mod Workshop is the series where I go over everything from getting started to editing status scripts.

<a href="https://www.youtube.com/playlist?list=PLJ8C0Hk2ZKHvxjfFylRUIbVnDUvxLdejh">Here's</a> a link to the playlist

<br><h2>List of episodes and what's covered in them:</h2>
<b>1. Getting Started</b>
<br>&emsp;&ensp;•Setting up a workspace to start code modding
<br><b>2. Basic File Structure</b>
  <br>&emsp;&ensp;•How hitbox scripts are composed
<br><b>3. Changing Hitbox Data</b>
  <br>&emsp;&ensp;•macros::ATTACK and its parameters
<br><b>4. My Resources</b>
  <br>&emsp;&ensp;•<a href="https://github.com/WuBoytH/SSBU-Dumped-Scripts/tree/main/smashline">WuBoy's script dump</a>
  <br>&emsp;&ensp;•cpp.rs and lua_const.rs files
  <br>&emsp;&ensp;•<a href="https://discord.gg/ASJyTrZ">Smash Ultimate Modding Discord</a>
  <br>&emsp;&ensp;•<a href="https://ultimateframedata.com/">Ultimate Frame Data</a>
  <br>&emsp;&ensp;•<a href="https://www.ssbwiki.com/">SSBWiki</a>
  <br>&emsp;&ensp;•<a href="https://smashultimatetools.com/">Smash Ultimate Tools</a>
  <br>&emsp;&ensp;•<a href="https://raw.githubusercontent.com/ultimate-research/param-labels/master/ParamLabels.csv">Hash List</a>
  <br>&emsp;&ensp;•<a href="https://ultimate-research.github.io/skyline-rs-template/doc/smash/index.html">Rust Docs</a>
  <br>&emsp;&ensp;•<a href="https://docs.google.com/spreadsheets/d/1q_TpWoQkr9YWgQ7fc3JpHuU9zKfCLtl80Uodcyc0NPY/edit#gid=0">AnimCMD Spreadsheet</a>
  <br>&emsp;&ensp;•Github
<br><b>5. Effects & Backgrounds</b>
  <br>&emsp;&ensp;•macros::FOOT_EFFECT
  <br>&emsp;&ensp;•macros::EFFECT_FOLLOW
  <br>&emsp;&ensp;•macros::AFTER_IMAGE4_ON_arg29
  <br>&emsp;&ensp;•macros::AFTER_IMAGE_OFF
  <br>&emsp;&ensp;•macros::EFFECT_OFF_KIND
  <br>&emsp;&ensp;•EffectModule::req_screen
<br><b>6. Porting Moves</b>
  <br>&emsp;&ensp;•Putting one character's hitboxes on another
  <br>&emsp;&ensp;•Porting animations with arcexplorer
  <br>&emsp;&ensp;•Editing fighter_param.prc with smashultimatetools
  <br>&emsp;&ensp;•Editing motion_list.bin with Discord Forge
  <br>&emsp;&ensp;•Using the replace tool in an IDE
<br><b>7. Common Errors & Troubleshooting Tips</b>
  <br>&emsp;&ensp;•Function cannot be found
  <br>&emsp;&ensp;•Mismatched types
  <br>&emsp;&ensp;•Wrong amount of args
  <br>&emsp;&ensp;•const)&const
  <br>&emsp;&ensp;•operatorbool
  <br>&emsp;&ensp;•float var
  <br>&emsp;&ensp;•hash40 error
  <br>&emsp;&ensp;•Unclosed/mismatched delimter
  <br>&emsp;&ensp;•How to search in discord 
<br><b>8. Damage & Healing</b>
  <br>&emsp;&ensp;•DamageModule::add_damage
  <br>&emsp;&ensp;•DamageModule::damage
  <br>&emsp;&ensp;•DamageModule::heal
  <br>&emsp;&ensp;•If else statements
  <br>&emsp;&ensp;•Greater & less than statements
<br><b>9. Getting Player Inputs</b>
  <br>&emsp;&ensp;•ControlModule::check_button_on
  <br>&emsp;&ensp;•ControlModule::get_stick_x & ControlModule::get_stick_y
  <br>&emsp;&ensp;•Declaring variables
  <br>&emsp;&ensp;•Usage of cpp.rs
  <br>&emsp;&ensp;•Replacing multiple attack scripts at once
  <br>&emsp;&ensp;•Replacing scripts for taunts
<br><b>10. Transitioning Into Moves</b>
  <br>&emsp;&ensp;•StatusModule::change_status_request_from_script
  <br>&emsp;&ensp;•ControlModule::set_attack_air_kind
<br><b>11. Fighter Frames</b>
  <br>&emsp;&ensp;•Global fighter frames
  <br>&emsp;&ensp;•Fighter-specific fighter frames
  <br>&emsp;&ensp;•Running code on players other than player 1
  <br>&emsp;&ensp;•MotionModule::motion_kind
  <br>&emsp;&ensp;•fighter.change_status
  <br>&emsp;&ensp;•WorkModule::set_int
  <br>&emsp;&ensp;•Making a move always active
<br><b>12. All About Frames</b>
  <br>&emsp;&ensp;•Using ||
  <br>&emsp;&ensp;•MotionModule::frame
  <br>&emsp;&ensp;•FighterMotionModuleImpl::get_cancel_frame
  <br>&emsp;&ensp;•CancelModule::enable_cancel
  <br>&emsp;&ensp;•macros::FT_MOTION_RATE
<br><b>13. Teleportation & Movement</b>
  <br>&emsp;&ensp;•Global variables
  <br>&emsp;&ensp;•Mutable variables
  <br>&emsp;&ensp;•Saving values to variables
  <br>&emsp;&ensp;•Arrays
  <br>&emsp;&ensp;•entry_id
  <br>&emsp;&ensp;•Vectors
  <br>&emsp;&ensp;•PostureModule::set_pos
  <br>&emsp;&ensp;•PostureModule::pos_x, PostureModule::pos_y, & PostureModule::pos_z
  <br>&emsp;&ensp;•Teleportation (part 1)
<br><b>14. Crits & Randomness</b>
  <br>&emsp;&ensp;•smash::app::sv_math::rand
  <br>&emsp;&ensp;•Using <b>as</b> to change the type of a variable
  <br>&emsp;&ensp;•Using sv_math::rand to make crits
<br><b>15. Super Armor & Invincibility</b>
  <br>&emsp;&ensp;•Programming "modes" akin to monado arts
  <br>&emsp;&ensp;•macros::WHOLE_HIT
  <br>&emsp;&ensp;•damage!
  <br>&emsp;&ensp;•Invincibility, intangibility, super armor
<br><b>16. Reflectors</b>
  <br>&emsp;&ensp;•Delcaring reflectors
  <br>&emsp;&ensp;•Removing reflectors
<br><b>17. Multipliers</b>
  <br>&emsp;&ensp;•AttackModule::set_power_up
  <br>&emsp;&ensp;•AttackModule::set_reaction_mul
  <br>&emsp;&ensp;•DamageModule::set_damage_mul
  <br>&emsp;&ensp;•DamageModule::set_reaction_mul
  <br>&emsp;&ensp;•smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul
  <br>&emsp;&ensp;•kinetic_motion
<br><b>18. Velocity</b>
  <br>&emsp;&ensp;•KineticModule::add_speed
  <br>&emsp;&ensp;•KineticModule::clear_speed_all
  <br>&emsp;&ensp;•StatusModule::situation_kind
  <br>&emsp;&ensp;•Vectors (again)
<br><b>19. Direction</b>
  <br>&emsp;&ensp;•PostureModule::lr
<br><b>20. Changing Size</b>
  <br>&emsp;&ensp;•PostureModule::set_scale
  <br>&emsp;&ensp;•PostureModule::scale
<br><b>21. Slowing Time</b>
  <br>&emsp;&ensp;•Distance formula
  <br>&emsp;&ensp;•.sqrt()
  <br>&emsp;&ensp;•macros::SLOW_OPPONENT
<br><b>22. Spawning Items & Articles</b>
  <br>&emsp;&ensp;•ArticleModule::generate_article
  <br>&emsp;&ensp;•ArticleModule::remove_exist
  <br>&emsp;&ensp;•ItemModule::have_item
<br><b>23. Visibility</b>
  <br>&emsp;&ensp;•VisibilityModule::set_whole
  <br>&emsp;&ensp;•ModelModule::set_mesh_visibility
  <br>&emsp;&ensp;•Meshes
  <br>&emsp;&ensp;•Basic StudioSB Usage
  <br>&emsp;&ensp;•macros::EFFECT vs macros::EFFECT_FOLLOW
  <br>&emsp;&ensp;•macros::LAST_EFFECT_SET_RATE
  <br>&emsp;&ensp;•Teleportaion (part 2)
<br><b>24. Slotting</b>
  <br>&emsp;&ensp;•WorkModule::get_int
  <br>&emsp;&ensp;•Using get_int to get a player's slot
  <br>&emsp;&ensp;•Modulo
  <br>&emsp;&ensp;•Slotting movesets
<br><b>25. Hooking Functions</b>
  <br>&emsp;&ensp;•Hooking functions other that hitboxes
  <br>&emsp;&ensp;•Hooking StatusModule::situation_kind
<br><b>26. Parameter Edits</b>
  <br>&emsp;&ensp;•Hooking float and int param accessor hooks to overwrite parameters
  <br>&emsp;&ensp;•Single-slotting parameters
<br><b>27. Hit Detection</b>
  <br>&emsp;&ensp;•Declaring global flags
  <br>&emsp;&ensp;•WorkModule::on_flag & WorkModule::off_flag
  <br>&emsp;&ensp;•sv_battle_object::kind
  <br>&emsp;&ensp;•utility::get_category
  <br>&emsp;&ensp;•Hooking NOTIFY_LOG_EVENT_COLLISION_HIT to detect hits
<br><b>28. Replacing Hitbox Data</b>
  <br>&emsp;&ensp;•Hooking sv_animcmd::ATTACK to replace certain parameters of every hitbox
  <br>&emsp;&ensp;•l2c_agent
  <br>&emsp;&ensp;•Vec arrays
  <br>&emsp;&ensp;•Stacks and how to clear, push, and pop them
  <br>&emsp;&ensp;•L2CValue::new_num & L2CValue::new_int
  <br>&emsp;&ensp;•for loops
<br><b>29. Ghidra & Status Scripts Part 1: How To Get Scripts</b>
  <br>&emsp;&ensp;•Downloading and setting up ghidra
  <br>&emsp;&ensp;•Using ghidra to get status scripts
<br><b>30. Ghidra & Status Scripts Part 2: How To Translate Scripts</b>
  <br>&emsp;&ensp;•Introduction to translating status scripts

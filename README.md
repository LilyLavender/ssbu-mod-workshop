# Smash Ultimate Modding Workshop
The Smash Ultimate Mod Workshop is the series where I go over everything from getting started to editing status scripts.

<a href="https://www.youtube.com/playlist?list=PLJ8C0Hk2ZKHvxjfFylRUIbVnDUvxLdejh">Here's</a> a link to the playlist

<br><h2>List of episodes and what's covered in them:</h2>
<b>1. Getting Started</b>
<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Setting up a workspace to start code modding
<br><b>2. Basic File Structure</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•How hitbox scripts are composed
<br><b>3. Changing Hitbox Data</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•macros::ATTACK and its parameters
<br><b>4. My Resources</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•<a href="https://github.com/WuBoytH/SSBU-Dumped-Scripts/tree/main/smashline">WuBoy's script dump</a>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•cpp.rs and lua_const.rs files
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•<a href="https://discord.gg/ASJyTrZ">Smash Ultimate Modding Discord</a>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•<a href="https://ultimateframedata.com/">Ultimate Frame Data</a>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•<a href="https://www.ssbwiki.com/">SSBWiki</a>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•<a href="https://smashultimatetools.com/">Smash Ultimate Tools</a>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•<a href="https://raw.githubusercontent.com/ultimate-research/param-labels/master/ParamLabels.csv">Hash List</a>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•<a href="https://ultimate-research.github.io/skyline-rs-template/doc/smash/index.html">Rust Docs</a>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•<a href="https://docs.google.com/spreadsheets/d/1q_TpWoQkr9YWgQ7fc3JpHuU9zKfCLtl80Uodcyc0NPY/edit#gid=0">AnimCMD Spreadsheet</a>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Github
<br><b>5. Effects & Backgrounds</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•macros::FOOT_EFFECT
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•macros::EFFECT_FOLLOW
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•macros::AFTER_IMAGE4_ON_arg29
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•macros::AFTER_IMAGE_OFF
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•macros::EFFECT_OFF_KIND
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•EffectModule::req_screen
<br><b>6. Porting Moves</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Putting one character's hitboxes on another
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Porting animations with arcexplorer
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Editing fighter_param.prc with smashultimatetools
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Editing motion_list.bin with Discord Forge
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Using the replace tool in an IDE
<br><b>7. Common Errors & Troubleshooting Tips</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Function cannot be found
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Mismatched types
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Wrong amount of args
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•const)&const
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•operatorbool
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•float var
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•hash40 error
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Unclosed/mismatched delimter
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•How to search in discord 
<br><b>8. Damage & Healing</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•DamageModule::add_damage
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•DamageModule::damage
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•If else statements
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Greater & less than statements
<br><b>9. Getting Player Inputs</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•ControlModule::check_button_on
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•ControlModule::get_stick_x & ControlModule::get_stick_y
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Declaring variables
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Usage of cpp.rs
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Replacing multiple attack scripts at once
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Replacing scripts for taunts
<br><b>10. Transitioning Into Moves</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•StatusModule::change_status_request_from_script
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•ControlModule::set_attack_air_kind
<br><b>11. Fighter Frames</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Global fighter frames
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Fighter-specific fighter frames
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Running code on players other than player 1
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•MotionModule::motion_kind
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•fighter.change_status
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•WorkModule::set_int
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Making a move always active
<br><b>12. All About Frames</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Using ||
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•MotionModule::frame
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•FighterMotionModuleImpl::get_cancel_frame
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•CancelModule::enable_cancel
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•macros::FT_MOTION_RATE
<br><b>13. Teleportation & Movement</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Global variables
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Mutable variables
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Saving values to variables
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Arrays
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•entry_id
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Vectors
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•PostureModule::set_pos
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•PostureModule::pos_x, PostureModule::pos_y, & PostureModule::pos_z
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Teleportation (part 1)
<br><b>14. Crits & Randomness</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•smash::app::sv_math::rand
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Using <b>as</b> to change the type of a variable
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Using sv_math::rand to make crits
<br><b>15. Super Armor & Invincibility</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Programming "modes" akin to monado arts
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•macros::WHOLE_HIT
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•damage!
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Invincibility, intangibility, super armor
<br><b>16. Reflectors</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Delcaring reflectors
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Removing reflectors
<br><b>17. Multipliers</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•AttackModule::set_power_up
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•AttackModule::set_reaction_mul
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•DamageModule::set_damage_mul
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•DamageModule::set_reaction_mul
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•kinetic_motion
<br><b>18. Velocity</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•KineticModule::add_speed
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•KineticModule::clear_speed_all
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•StatusModule::situation_kind
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Vectors (again)
<br><b>19. Direction</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•PostureModule::lr
<br><b>20. Changing Size</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•PostureModule::set_scale
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•PostureModule::scale
<br><b>21. Slowing Time</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Distance formula
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•.sqrt()
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•macros::SLOW_OPPONENT
<br><b>22. Spawning Items & Articles</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•ArticleModule::generate_article
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•ArticleModule::remove_exist
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•ItemModule::have_item
<br><b>23. Visibility</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•VisibilityModule::set_whole
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•ModelModule::set_mesh_visibility
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Meshes
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Basic StudioSB Usage
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•macros::EFFECT vs macros::EFFECT_FOLLOW
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Teleportaion (part 2)
<br><b>24. One-Slotting</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•WorkModule::get_int
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Using get_int to get a player's slot
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Modulo
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•One-slotting movesets
<br><b>25. Hooking Functions</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Hooking functions other that hitboxes
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Hooking StatusModule::situation_kind
<br><b>26. Parameter Edits</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Hooking float and int param accessor hooks to overwrite parameters
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Single-slotting parameters
<br><b>27. Hit Detection</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Declaring global flags
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•WorkModule::on_flag & WorkModule::off_flag
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•sv_battle_object::kind
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•utility::get_category
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Hooking NOTIFY_LOG_EVENT_COLLISION_HIT to detect hits
<br><b>28. Replacing Hitbox Data</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Hooking sv_animcmd::ATTACK to replace certain parameters of every hitbox
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•l2c_agent
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Vec arrays
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Stacks and how to clear, push, and pop them
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•L2CValue::new_num & L2CValue::new_int
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•for loops
<br><b>29. Ghidra & Status Scripts Part 1: How To Get Scripts</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Downloading and setting up ghidra
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Using ghidra to get status scripts
<br><b>30. Ghidra & Status Scripts Part 2: How To Translate Scripts</b>
  <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;•Introduction to translating status scripts

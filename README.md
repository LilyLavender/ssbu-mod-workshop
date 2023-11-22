# Smash Ultimate Modding Workshop
The Smash Ultimate Mod Workshop is the series where I go over everything from getting started to editing status scripts.

[Here's](https://www.youtube.com/playlist?list=PLJ8C0Hk2ZKHvxjfFylRUIbVnDUvxLdejh) a link to the playlist

## List of episodes and what's covered in them: 
<br>**1. Getting Started**
  <br>&emsp;&ensp;• Setting up a workspace to start code modding
<br>**2. Basic File Structure**
  <br>&emsp;&ensp;• How hitbox scripts are composed
<br>**3. Changing Hitbox Data**
  <br>&emsp;&ensp;• macros::ATTACK and its parameters
<br>**4. My Resources Part 1**
  <br>&emsp;&ensp;• [WuBoy's script dump](https://github.com/WuBoytH/SSBU-Dumped-Scripts/tree/main/smashline)
  <br>&emsp;&ensp;• cpp.rs and lua_const.rs files
  <br>&emsp;&ensp;• [Smash Ultimate Modding Discord](https://discord.gg/ASJyTrZ)
  <br>&emsp;&ensp;• [Ultimate Frame Data](https://ultimateframedata.com/)
  <br>&emsp;&ensp;• [SSBWiki](https://www.ssbwiki.com/)
  <br>&emsp;&ensp;• [Smash Ultimate Tools](https://smashultimatetools.com/)
  <br>&emsp;&ensp;• [Hash List](https://raw.githubusercontent.com/ultimate-research/param-labels/master/ParamLabels.csv)
  <br>&emsp;&ensp;• [Rust Docs](https://ultimate-research.github.io/skyline-rs-template/doc/smash/index.html)
  <br>&emsp;&ensp;• [AnimCMD Spreadsheet](https://docs.google.com/spreadsheets/d/1q_TpWoQkr9YWgQ7fc3JpHuU9zKfCLtl80Uodcyc0NPY/edit#gid=0)
  <br>&emsp;&ensp;• Github
<br>**5. Effects & Backgrounds**
  <br>&emsp;&ensp;• macros::FOOT_EFFECT
  <br>&emsp;&ensp;• macros::EFFECT_FOLLOW
  <br>&emsp;&ensp;• macros::AFTER_IMAGE4_ON_arg29
  <br>&emsp;&ensp;• macros::AFTER_IMAGE_OFF
  <br>&emsp;&ensp;• macros::EFFECT_OFF_KIND
  <br>&emsp;&ensp;• EffectModule::req_screen
<br>**6. Porting Moves**
  <br>&emsp;&ensp;• Putting one character's hitboxes on another
  <br>&emsp;&ensp;• Porting animations with arcexplorer
  <br>&emsp;&ensp;• Editing fighter_param.prc with smashultimatetools
  <br>&emsp;&ensp;• Editing motion_list.bin with Discord Forge
  <br>&emsp;&ensp;• Using the replace tool in an IDE
<br>**7. Common Errors & Troubleshooting Tips**
  <br>&emsp;&ensp;• Function cannot be found
  <br>&emsp;&ensp;• Mismatched types
  <br>&emsp;&ensp;• Wrong amount of args
  <br>&emsp;&ensp;• const)&const
  <br>&emsp;&ensp;• operatorbool
  <br>&emsp;&ensp;• float var
  <br>&emsp;&ensp;• hash40 error
  <br>&emsp;&ensp;• Unclosed/mismatched delimter
  <br>&emsp;&ensp;• How to search in discord 
<br>**8. Damage & Healing**
  <br>&emsp;&ensp;• DamageModule::add_damage
  <br>&emsp;&ensp;• DamageModule::damage
  <br>&emsp;&ensp;• DamageModule::heal
  <br>&emsp;&ensp;• If else statements
  <br>&emsp;&ensp;• Greater & less than statements
<br>**9. Getting Player Inputs**
  <br>&emsp;&ensp;• ControlModule::check_button_on
  <br>&emsp;&ensp;• ControlModule::get_stick_x & ControlModule::get_stick_y
  <br>&emsp;&ensp;• Declaring variables
  <br>&emsp;&ensp;• Usage of cpp.rs
  <br>&emsp;&ensp;• Replacing multiple attack scripts at once
  <br>&emsp;&ensp;• Replacing scripts for taunts
<br>**10. Transitioning Into Moves**
  <br>&emsp;&ensp;• StatusModule::change_status_request_from_script
  <br>&emsp;&ensp;• ControlModule::set_attack_air_kind
<br>**11. Fighter Frames**
  <br>&emsp;&ensp;• Global fighter frames
  <br>&emsp;&ensp;• Fighter-specific fighter frames
  <br>&emsp;&ensp;• Running code on players other than player 1
  <br>&emsp;&ensp;• MotionModule::motion_kind
  <br>&emsp;&ensp;• fighter.change_status
  <br>&emsp;&ensp;• WorkModule::set_int
  <br>&emsp;&ensp;• Making a move always active
<br>**12. All About Frames**
  <br>&emsp;&ensp;• Using ||
  <br>&emsp;&ensp;• MotionModule::frame
  <br>&emsp;&ensp;• FighterMotionModuleImpl::get_cancel_frame
  <br>&emsp;&ensp;• CancelModule::enable_cancel
  <br>&emsp;&ensp;• macros::FT_MOTION_RATE
<br>**13. Teleportation & Movement**
  <br>&emsp;&ensp;• Global variables
  <br>&emsp;&ensp;• Mutable variables
  <br>&emsp;&ensp;• Saving values to variables
  <br>&emsp;&ensp;• Arrays
  <br>&emsp;&ensp;• entry_id
  <br>&emsp;&ensp;• Vectors
  <br>&emsp;&ensp;• PostureModule::set_pos
  <br>&emsp;&ensp;• PostureModule::pos_x, PostureModule::pos_y, & PostureModule::pos_z
  <br>&emsp;&ensp;• Teleportation (part 1)
<br>**14. Crits & Randomness**
  <br>&emsp;&ensp;• smash::app::sv_math::rand
  <br>&emsp;&ensp;• Using **as** to change the type of a variable
  <br>&emsp;&ensp;• Using sv_math::rand to make crits
<br>**15. Super Armor & Invincibility**
  <br>&emsp;&ensp;• Programming "modes" akin to monado arts
  <br>&emsp;&ensp;• macros::WHOLE_HIT
  <br>&emsp;&ensp;• damage!
  <br>&emsp;&ensp;• Invincibility, intangibility, super armor
<br>**16. Reflectors**
  <br>&emsp;&ensp;• Delcaring reflectors
  <br>&emsp;&ensp;• Removing reflectors
<br>**17. Multipliers**
  <br>&emsp;&ensp;• AttackModule::set_power_up
  <br>&emsp;&ensp;• AttackModule::set_reaction_mul
  <br>&emsp;&ensp;• DamageModule::set_damage_mul
  <br>&emsp;&ensp;• DamageModule::set_reaction_mul
  <br>&emsp;&ensp;• smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul
  <br>&emsp;&ensp;• kinetic_motion
<br>**18. Velocity**
  <br>&emsp;&ensp;• KineticModule::add_speed
  <br>&emsp;&ensp;• KineticModule::clear_speed_all
  <br>&emsp;&ensp;• StatusModule::situation_kind
  <br>&emsp;&ensp;• Vectors (again)
<br>**19. Direction**
  <br>&emsp;&ensp;• PostureModule::lr
<br>**20. Changing Size**
  <br>&emsp;&ensp;• PostureModule::set_scale
  <br>&emsp;&ensp;• PostureModule::scale
<br>**21. Slowing Time**
  <br>&emsp;&ensp;• Distance formula
  <br>&emsp;&ensp;• .sqrt()
  <br>&emsp;&ensp;• macros::SLOW_OPPONENT
<br>**22. Spawning Items & Articles**
  <br>&emsp;&ensp;• ArticleModule::generate_article
  <br>&emsp;&ensp;• ArticleModule::remove_exist
  <br>&emsp;&ensp;• ItemModule::have_item
<br>**23. Visibility**
  <br>&emsp;&ensp;• VisibilityModule::set_whole
  <br>&emsp;&ensp;• ModelModule::set_mesh_visibility
  <br>&emsp;&ensp;• Meshes
  <br>&emsp;&ensp;• Basic StudioSB Usage
  <br>&emsp;&ensp;• macros::EFFECT vs macros::EFFECT_FOLLOW
  <br>&emsp;&ensp;• macros::LAST_EFFECT_SET_RATE
  <br>&emsp;&ensp;• Teleportaion (part 2)
<br>**24. Slotting Part 1**
  <br>&emsp;&ensp;• WorkModule::get_int
  <br>&emsp;&ensp;• Using get_int to get a player's slot
  <br>&emsp;&ensp;• Modulo
  <br>&emsp;&ensp;• Slotting movesets
<br>**25. Hooking Functions**
  <br>&emsp;&ensp;• Hooking functions other that hitboxes
  <br>&emsp;&ensp;• Hooking StatusModule::situation_kind
<br>**26. Parameter Edits**
  <br>&emsp;&ensp;• Hooking float and int param accessor hooks to overwrite parameters
  <br>&emsp;&ensp;• Single-slotting parameters
<br>**27. Hit Detection**
  <br>&emsp;&ensp;• Declaring global flags
  <br>&emsp;&ensp;• WorkModule::on_flag & WorkModule::off_flag
  <br>&emsp;&ensp;• sv_battle_object::kind
  <br>&emsp;&ensp;• utility::get_category
  <br>&emsp;&ensp;• Hooking NOTIFY_LOG_EVENT_COLLISION_HIT to detect hits
<br>**28. Replacing Hitbox Data**
  <br>&emsp;&ensp;• Hooking sv_animcmd::ATTACK to replace certain parameters of every hitbox
  <br>&emsp;&ensp;• l2c_agent
  <br>&emsp;&ensp;• Vec arrays
  <br>&emsp;&ensp;• Stacks and how to clear, push, and pop them
  <br>&emsp;&ensp;• L2CValue::new_num & L2CValue::new_int
  <br>&emsp;&ensp;• for loops
<br>**29. Ghidra & Status Scripts Part 1: How To Get Scripts**
  <br>&emsp;&ensp;• Downloading and setting up ghidra
  <br>&emsp;&ensp;• Using ghidra to get status scripts
<br>**30. Ghidra & Status Scripts Part 2: How To Translate Scripts**
  <br>&emsp;&ensp;• Introduction to translating status scripts
<br>**31. My Resources Part 2**
  <br>&emsp;&ensp;• [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/intro.html)
  <br>&emsp;&ensp;• [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)
  <br>&emsp;&ensp;• [global_table List](https://github.com/WuBoytH/The-WuBor-Patch/blob/dev/WuBor-Utils/src/table_const.rs)
  <br>&emsp;&ensp;• global_table
  <br>&emsp;&ensp;• status_frame 
  <br>&emsp;&ensp;• [motion_list dump](https://github.com/WuBoytH/SSBU-Dumped-Motion-Lists)
  <br>&emsp;&ensp;• [Status Script Dump](https://github.com/Coolsonickirby/SSBU-Dumped-Status-Scripts/tree/master)
  <br>&emsp;&ensp;• Accessing the shared Ghidra server
  <br>&emsp;&ensp;• [Dragdown Wiki](https://dragdown.wiki/wiki/Super_Smash_Bros._Ultimate)
<br>**32. Sounds**
  <br>&emsp;&ensp;• macros::PLAY_SE
<br>**33. Expressions**
  <br>&emsp;&ensp;• ItemModule::set_have_item_visibility
  <br>&emsp;&ensp;• ItemModule::set_attach_item_visibility
<br>**34. Items**
  <br>&emsp;&ensp;• Editing item scripts
  <br>&emsp;&ensp;• macros::ATTACK_FP
<br>**35. Getting Parameters**
  <br>&emsp;&ensp;• WorkModule::get_param_float
  <br>&emsp;&ensp;• WorkModule::get_param_int
  <br>&emsp;&ensp;• Delcaring functions
  <br>&emsp;&ensp;• macros::EFFECT_FOLLOW_arg11
<br>**36. Status Kinds**
  <br>&emsp;&ensp;• StatusModule::status_kind
<br>**37. Debugging with println**
  <br>&emsp;&ensp;• println!
  <br>&emsp;&ensp;• Detecting changes in damage
  <br>&emsp;&ensp;• WorkModule::get_float
  
  

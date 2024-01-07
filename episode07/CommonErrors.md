## Common Errors and How to Solve Them

| Error | Cause | Solution |
| --- | --- | --- |
| cannot find function `frame` in this scope | Your `use` section at the top is most likely incorrect | Copy the `use` section from one of the files in this repo |
| cannot find function `game_specialairlwhit_smashline_acmd_script_install` in this scope | You most likely named a function incorrectly | Double-check your names |
| mismatched types / expected struct `A` found struct `B` | Some variable has the wrong type | Depends on the situation. Either convert the struct to another type (eg `1` -> `1.0`) or, if the struct is something like `ItemKind`, you may have to wrap it in a special structure depending on the type. At the end of the day, you can always search for the function in [rust docs](https://ultimate-research.github.io/skyline-rs-template/doc/smash/index.html), the [SSBU Modding Discord](https://discord.gg/ASJyTrZ), or here on Github |
| this function takes X arguments but Y arguments were supplied | The wrong number of arguments were put into the function either too many or too few | Check the amount of arguments with rust analyzer, on [rust docs](https://ultimate-research.github.io/skyline-rs-template/doc/smash/index.html), the [SSBU Modding Discord](https://discord.gg/ASJyTrZ), or Github |
| Code has a weird `const&)const` bit | Usually an issue of the script dump | There should be a `WorkModule::is_flag` line above it. Check the [video at 1:38](https://youtu.be/i_AaufEb5vo?list=PLJ8C0Hk2ZKHvxjfFylRUIbVnDUvxLdejh&t=98) for more info |
| `operatorbool()` | Usually an issue of the script dump | There should be two consts with asterisks before them. Compare them with this line: `if *CONST_1 == *CONST_2 {` |
| `get_value_float` | Usually an issue of the script dump | Replace `get_value_float(SO_VAR_FLOAT_LR) if 0xXXXXXX(0, 0)){` with `if get_value_float(agent, *SO_VAR_FLOAT_LR) < 0.0 {` |
| Mismatched/unclosed Delimiter | You either are missing a delimiter ( `(, {, or [ `), or have an extra one | Use VSCode to match up an opening tag with its closing one. There should be a pretty simple structure to your code that should make it easy to tell where a missing delimiter is, or where an extra one is slotted |

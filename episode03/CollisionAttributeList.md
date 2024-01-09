## List of Every Collision Attribute and Usage
List by LilyLambda & PhazoGanon.  
An easier-to-copy-paste list can be found [here](https://github.com/ultimate-research/param-labels/blob/master/ParamLabels.csv).  
Collision Attributes with "📝" require more testing. If anyone wants to help, there's comments in this doc that will have general and per-move instructions.
 <!-- WIP: Instructions on helping with unknowns. -->

| Hash | Text | Description | Type | 
| --- | --- | --- | --- |
| 0x13313725f6 | collision_attr_aura | Emits blue flame-like particles. Primarily used on Lucario's moves, but can also be found on Richter's Holy Water | Visual |
| 0x193bdcb0cc | collision_attr_auto_shift | 📝 Most likely moves the player over slightly to help continue combos. Used only in Kazuya's Jabs 1-8. | Unknown | <!-- Need a general description of what it looks like and how it functions. If it's similar to another hit effect, mention how it differs. Remains in here until WuBoy's script dump stops using the hash. -->
| 0x13a0a379de | collision_attr_bind | Inflicts the dizzy status (eg. shield breaks). Only works on ground. | Hit |
| 0x199d57f593 | collision_attr_bind_extra | Inflicts a variant on the dizzy effect that calls to different params in common.prc. Used in Mewtwo's Disable and Marshadow. | Hit |
| 0x21016e5c76 | collision_attr_blaster_throw_down | 📝 Only used on Fox's Down Throw. | Visual | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. -->
| 0x1f468d5240 | collision_attr_blaster_throw_up | 📝 Only used on Fox's Up Throw. | Visual | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. -->
| 0x1330ee124e | collision_attr_bury | Inflicts the bury status (eg. DK Side Special, K. Rool's Down Tilt). Only works on ground. | Hit |
| 0x155fb26059 | collision_attr_bury_r | A variant on the bury effect that's used for throws. Only works on ground. | Hit |
| 0x13b3061dd0 | collision_attr_coin | Spawns a coin effect. Used on Mario's Super Jump Punch. | Visual |
| 0x1b2fb009bf | collision_attr_curse_poison | A damage over time effect used on Joker's Eiha/Eigaon. | Hit |
| 0x14860b063a | collision_attr_cutup | A crescent-shaped effect used on sword slashes. | Visual |
| 0x1a5242290d | collision_attr_cutup_metal | 📝 A variant on the cutup effect that's exclusively on Hero's Metal Slash, most likely visual, but could also be the cause of Metal Slash instantly KOing metal opponents. | Unknown | <!-- Figure out if this always instakills metal opponents, or if there's a Hero-specific function that causes it to work. If it's not visually identical to cutup, mention how it differs. See deathball also. -->
| 0x1474a84539 | collision_attr_death | Instantly KOs the opponent. | Hit |
| 0x18bef6339f | collision_attr_deathball | 📝 Similar to the instant KO effect. Used on Whack/Thwack, but always instakills when applied to other hitboxes, likely due to not being able to call to a Hero-specific function to determine when it'll KO. | Hit | <!-- Find the alleged Hero-specific function and, if possible, find a way to port it to other characters. -->
| 0x1c6d20b1f9 | collision_attr_dedede_hammer | 📝 Differs only slightly from the normal hit effect. Used on all of Dedede's moves that use his hammer, sans Down B. | Visual | <!-- Need a general description of what it looks like. If it actually is identical to the normal hit effect, or a different one, state so. -->
| 0x13462fcfe4 | collision_attr_elec | Emits sparks and causes the attacker to take 1.5x hitlag frames. Used on electric moves like Pichu Fair and Ganondorf Jab. | Hit |
| 0x184e3f7a79 | collision_attr_elec_whip | A variant on the electric effect where the visual effect has a shorter duration. Only used on Samus and Dark Samus's Throws. | Hit |
| 0x13beb18342 | collision_attr_fire | Emits flames. Used on fire moves like Mario's Fireball and Steve's Up Smash. | Visual |
| 0x184c223f47 | collision_attr_fist_down | 📝 Used only in Kazuya's Side Special. Most likely a special type of crumple. | Hit | <!-- Need a general description of what it looks like and how it functions. If it's similar to saving, mention how it differs. Remains in here until WuBoy's script dump stops using the hash. -->
| 0x19f2214801 | collision_attr_fist_down2 | 📝 Used only in Kazuya's Uncrouching Attack. Most likely a special type of crumple. | Hit | <!-- Need a general description of what it looks like and how it functions. If it's similar to saving, mention how it differs. Remains in here until WuBoy's script dump stops using the hash. -->
| 0x1985267897 | collision_attr_fist_down3 | 📝 Used only in Kazuya's Jab 9. Most likely a special type of crumple. | Hit | <!-- Need a general description of what it looks like and how it functions. If it's similar to saving, mention how it differs. Remains in here until WuBoy's script dump stops using the hash. -->
| 0x152497ab8d | collision_attr_flower | Inflicts the flower status, which puts a flower on the opponent's head. Used with Jigglypuff's Rest and the flower item. | Hit |
| 0x1c655b0ae7 | collision_attr_head_mushroom | Inflicts the mushroom status, which puts a mushroom on the opponent's head. Used exclusively on the Ramblin' Evil Mushroom item. | Hit |
| 0x12c7990841 | collision_attr_ice | Inflicts the frozen status. Used on ice moves like Kacrackle Slash and Ice Climbers Down Special. | Hit |
| 0x169c2f9aa4 | collision_attr_ink_hit | 📝 Coats the opponent in Ink. Used on Inkling moves. | Unknown | <!-- I'm skeptical that the visual effect is what coats the opponent. If it is, see if there's any other visual effects to accompany it. -->
| 0x1a6f665bee | collision_attr_jack_bullet | 📝 Only used on Joker's Neutral Special, needs testing. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. -->
| 0x19f9ff23e4 | collision_attr_jack_final | 📝 Only used on Joker's Final Smash, needs testing. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. -->
| 0x12e765f467 | collision_attr_lay | Inflicts the lay status, which puts opponent on the ground. Used on moves like Snake's Down Throw. | Hit |
| 0x1dc3c2ee10 | collision_attr_leviathan_wave | 📝 Related to Leviathan on the Midgar Stage. Needs testing. | Unknown |<!-- See this message from PhazoGanon: https://ibb.co/vcXdPt2. Figure out if it's Midgar-specific, what it looks like, and how it differs from the other leviathan wave. -->
| 0x239ec77f51 | collision_attr_leviathan_wave_owner | 📝 Related to Leviathan on the Midgar Stage. Needs testing. | Unknown | <!-- See this message from PhazoGanon: https://ibb.co/vcXdPt2. Figure out if it's Midgar-specific, what it looks like, and how it differs from the other leviathan wave. -->
| 0x149ae97939 | collision_attr_magic | Spawns magical effects. Used on moves like Zelda's Jab and Ness's Uair. | Visual |
| 0x1f9c885634 | collision_attr_mario_local_coin | A variant on the coin effect that summons the Odyssey Coins rather than the normal Mario Coins. | Visual |
| 0x23b3fb791e | collision_attr_marth_shield_breaker | 📝 Used only on Marth and Lucina's Shield Breaker. Needs testing. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. -->
| 0x15e069fac2 | collision_attr_noamal | Mispelling, does nothing, used exclusively on G&W Down Smash. | None |
| 0x1399ff8a42 | collision_attr_none | No visual effect. | None |
| 0x15a2c502b3 | collision_attr_normal | Basic hit visual effect. | Visual |
| 0x1cd9e295e5 | collision_attr_normal_bullet | 📝 Used only on Bayonetta bullet arts. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. -->
| 0x1c9a85bcd9 | collision_attr_normal_poison | 📝 Unsure. Used on the dracula2homingshot article from the Dracula boss and *possibly* the poison floor spirit battle effect. | Unknown | <!-- Figure out what this looks like, what effects it has, and how, if at all, it differs from curse_poison. -->
| 0x197da285b7 | collision_attr_odin_slash | 📝 Used only on the Odin summon on Midgar's slash attack. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. Figure out if it's Midgar-specific. -->
| 0x1e4c53bd48 | collision_attr_palutena_bullet | 📝 Used only on Palutena Neutral Special. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. -->
| 0x179c4e24c5 | collision_attr_paralyze | Inflicts the paralyze status, freezing the opponent in place for some time. Used on moves like Corrin's Neutral Special and ZSS' Down Smash. | Hit |
| 0x1d73cd1464 | collision_attr_paralyze_ghost | 📝 A variant on the paralyze effect that's exclusively used on the Yuri Kozukata (Fatal Frame) Assist Trophy. Not sure how it differs from paralyze. | Hit | <!-- Figure out how it differs from paralyze, whether purely visually or if there's mechanical differences, such as a different formula to calculate how long the stun lasts, using a different animation for the target, etc. -->
| 0x158507c5b5 | collision_attr_pierce | 📝 An effect similar to the crumple status, but with different params in common.prc. Only used on Corrin's Side Special. | Hit | <!-- Figure out exactly how it differs from other crumple statuses. -->
| 0x177f259087 | collision_attr_pit_fall | A variant on the bury effect that spikes in the air. Used on the pitfall item. | Hit |
| 0x14bf0ee04f | collision_attr_punch | Basic punch visual effect. | Visual |
| 0x1569c518b1 | collision_attr_purple | Emits shadowy flames. Used on moves like Ganondorf's Warlock Punch, Mewtwo's Fair, Ridley's Side Special, and Robin's Nosferatu. | Visual |
| 0x13135c5462 | collision_attr_rush | Smaller effect used in a lot of multihits. | Visual |
| 0x153a9c575b | collision_attr_saving | Inflicts the crumple status, which causes the opponent to be stunned and fall down. Used on moves like Ryu's Focus Attack. | Hit |
| 0x192a2f9ba6 | collision_attr_saving_ken | 📝 A variant on the crumple status that is used on Ken's Down Special, uncertain how this differs from standard crumple. | Hit | <!-- Figure out how it differs from Ryu's crumple, whether purely visually or if there's mechanical differences. -->
| 0x1537b0b1f0 | collision_attr_search | 📝 Used on the searchboxes of Falcon Side Special and Pit's Upperdash. | Unknown | <!-- Need a general description of what it looks like. Most likely doesn't have any visual effects, but if it's similar to another hit effect, mention how it differs. Most likely has additional effects. -->
| 0x149cdc52bb | collision_attr_sleep | Inflicts the sleep status, which puts opponents to sleep. Used on moves like Jigglypuff's Sing. | Hit |
| 0x1710166637 | collision_attr_sleep_ex | A variant on the sleep effect which pops the target up slightly. Used on moves like Hero's Snooze. | Hit |
| 0x131b56c975 | collision_attr_slip | Inflicts the slip status, which trips the opponent. Used on several moves, namely Diddy's Banana. | Hit |
| 0x133203c0fd | collision_attr_stab | 📝 Used only on Ridley's Skewer. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. -->
| 0x14604c88a3 | collision_attr_sting | Creates a horizontal piercing effect. Used on stabbing type moves like Roy's Down Tilt and Luigi's Forward Smash. | Visual |
| 0x1d7a481588 | collision_attr_sting_bowarrow | 📝 Used exclusively on Byleth Neutral Special (uncharged). Uncertain how it differs from the sting attribute. | Visual | <!-- Need a general description of what it looks like. If it's similar to sting, mention how it differs. May also have additional effects. -->
| 0x1aa2fd0729 | collision_attr_sting_flash | 📝 Used exclusively on Sephiroth's Scintilla. Uncertain how it differs from the sting attribute. | Visual |<!-- Need a general description of what it looks like. If it's similar to sting, mention how it differs. May also have additional effects. -->
| 0x135f399c3b | collision_attr_stop | 📝 Used exclusively on Sephiroth's Final Smash and Mega Man's Side Special. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. -->
| 0x18c8a6895d | collision_attr_taiyo_hit | Emits a golden particle ring and lens flare effect. Used on Wii Fit Trainer's Sun Salution. | Visual |
| 0x13c64f9fca | collision_attr_turn | Turns the opponent around. Used on moves like Mario's Cape and Mii Swordfighter's Reversal Slash. | Hit |
| 0x1468dc84cd | collision_attr_water | Emits water particles. Used on moves like Squirtle's Down Smash and Corrin's Up Special. | Visual |
| 0x13933df6fe | collision_attr_whip | Similar to the sting & cutup effects. Used on Simon/Richter's Whip Moves. | Visual |

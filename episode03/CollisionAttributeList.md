## List of Every Collision Attribute and Usage
List by LilyLambda, PhazoGanon, & zrksyd.  
An easier-to-copy-paste list can be found [here](https://github.com/ultimate-research/param-labels/blob/master/ParamLabels.csv).  
Collision Attributes with "📝" require more testing. If anyone wants to help, there's comments in this doc that will have general and per-move instructions.

| Hash | Text | Description | Type | 
| --- | --- | --- | --- |
| 0x13313725f6 | collision_attr_**aura** | Emits blue flame-like particles. Primarily used on Lucario's moves, but can also be found on Richter's Holy Water. | Visual |
| 0x193bdcb0cc | collision_attr_**auto_shift** | 📝 Most likely moves the player over slightly to help continue combos. Used only in the first 8 hits of Kazuya's 10-Hit Combo. | Unknown | <!-- Need a general description of what it looks like and how it functions. If it's similar to another hit effect, mention how it differs. -->
| 0x13a0a379de | collision_attr_**bind** | Inflicts the dizzy status (eg. shield breaks). Only works on ground. | Hit |
| 0x199d57f593 | collision_attr_**bind_extra** | Inflicts a variant on the dizzy effect that calls to different params in common.prc. Used in Mewtwo's Disable and Marshadow. | Hit |
| 0x21016e5c76 | collision_attr_**blaster_throw_down** | 📝 Only used on Fox's Down Throw. | Visual | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. -->
| 0x1f468d5240 | collision_attr_**blaster_throw_up** | 📝 Only used on Fox's Up Throw. | Visual | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. -->
| 0x1330ee124e | collision_attr_**bury** | Inflicts the bury status (eg. DK's Headbutt, K. Rool's Down Tilt). Only works on ground. | Hit |
| 0x154568b424 | collision_attr_**bury_f** | 📝 Identical to bury. | Hit | <!-- See if there is any difference from collision_attr_bury-->
| 0x155fb26059 | collision_attr_**bury_r** | A variant on the bury effect that's used for throws. Only works on ground. | Hit |
| 0x13b3061dd0 | collision_attr_**coin** | Spawns a coin effect. Used on Mario's Super Jump Punch. | Visual |
| 0x1b2fb009bf | collision_attr_**curse_poison** | A damage over time effect used on Joker's Eiha/Eigaon. | Hit |
| 0x14860b063a | collision_attr_**cutup** | A crescent-shaped effect used on sword slashes. | Visual |
| 0x1a5242290d | collision_attr_**cutup_metal** | A variant on the cutup effect that instantly KOs metal opponents. Used exclusively on Hero's Metal Slash. | Hit | 
| 0x1474a84539 | collision_attr_**death** | Instantly KOs the opponent. | Hit |
| 0x18bef6339f | collision_attr_**deathball** | 📝 Similar to death. Used on Hero's Whack/Thwack, but always instakills when applied to other hitboxes, likely due to not being able to call to a Hero-specific function to determine when it should KO. | Hit | <!-- Find the alleged Hero-specific function and, if possible, find a way to port it to other characters or prove that it's impossible. -->
| 0x1a5f94f889 | collision_attr_**deathscythe** | 📝 Most likely used on the Death's Scythe item. Needs testing | Unknown | <!-- Find out if this is purely visual or not and how it functions -->
| 0x1c6d20b1f9 | collision_attr_**dedede_hammer** | 📝 Differs only slightly from the normal hit effect. Used on all of Dedede's moves that use his hammer, sans Down B. | Visual | <!-- Need a general description of what it looks like. If it actually is identical to the normal hit effect, or a different one, state so. -->
| 0x19fa8a1d82 | collision_attr_**deep_sleep** | 📝 No hit effects. | Unknown | <!-- zrksyd did some testing and found this didn't have any hit effects. If someone could find out why, that would be great. -->
| 0x13462fcfe4 | collision_attr_**elec** | Emits sparks and causes the attacker to take 1.5x hitlag frames. Used on electric moves like Pichu Fair and Ganondorf Jab. | Hit |
| 0x184e3f7a79 | collision_attr_**elec_whip** | A variant on the electric effect where the visual effect has a shorter duration. Only used on Samus and Dark Samus's Throws. | Hit |
| 0x13beb18342 | collision_attr_**fire** | Emits flames. Used on fire moves like Mario's Fireball and Steve's Up Smash. | Visual |
| 0x184c223f47 | collision_attr_**fist_down** | 📝 Used only in Kazuya's Devil Fist. Most likely a special type of crumple. | Hit | <!-- Need a general description of what it looks like and how it functions. If it's similar to saving, mention how it differs. Remains in here until WuBoy's script dump stops using the hash. -->
| 0x19f2214801 | collision_attr_**fist_down2** | 📝 Used only in Kazuya's Demon God Fist. Most likely a special type of crumple. | Hit | <!-- Need a general description of what it looks like and how it functions. If it's similar to saving, mention how it differs. Remains in here until WuBoy's script dump stops using the hash. -->
| 0x1985267897 | collision_attr_**fist_down3** | 📝 Used only in Kazuya's 9th hit of 10-Hit Combo. Most likely a special type of crumple. | Hit | <!-- Need a general description of what it looks like and how it functions. If it's similar to saving, mention how it differs. Remains in here until WuBoy's script dump stops using the hash. -->
| 0x152497ab8d | collision_attr_**flower** | Inflicts the flower status, which puts a flower on the opponent's head. Used with Jigglypuff's Rest and the Lip's Stick item. | Hit |
| 0x143be20fe4 | collision_attr_**grass** | 📝 A variant of the cutup effect. Likely a remnant from *Brawl*. | Visual | <!-- See if there are any differences with cutup, whether purely visually or if there's mechanical differences. Guarantee that it doesn't have brawl's type-effectiveness aspect. Highly doubt it, but you never know. -->
| 0x15a03a1c5c | collision_attr_**grudge** | 📝 A variant of the purple effect. | Unknown | <!-- See if there are any differences with purple, whether purely visually or if there's mechanical differences. -->
| 0x1c655b0ae7 | collision_attr_**head_mushroom** | Inflicts the mushroom status, which puts a mushroom on the opponent's head. Used exclusively on the Ramblin' Evil Mushroom item. | Hit |
| 0x12c7990841 | collision_attr_**ice** | Inflicts the frozen status. Used on ice moves like Hero's Kacrackle Slash and Ice Climbers' Blizzard. | Hit |
| 0x169c2f9aa4 | collision_attr_**ink_hit** | 📝 Coats the opponent in Ink. Used on Inkling's moves and Master Hand's Paint Ball. | Unknown | <!-- I'm skeptical that the visual effect is what coats the opponent. If it is, see if there's any other visual effects to accompany it. -->
| 0x1ae690bd0d | collision_attr_**item_hammer** | 📝 Only used on the Hammer and Golden Hammer items. Produces visual effects from the Donkey Kong arcade. | Visual | <!-- Find out if this is purely visual or not and how it functions -->
| 0x1a6f665bee | collision_attr_**jack_bullet** | 📝 Only used on Joker's Gun, needs testing. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. -->
| 0x19f9ff23e4 | collision_attr_**jack_final** | 📝 Only used on Joker's Final Smash, needs testing. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. -->
| 0x13f8ba5588 | collision_attr_**kick** | 📝 No hit effects, only used on Galleom's Lariat. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. -->
| 0x12e765f467 | collision_attr_**lay** | Inflicts the lay status, which puts opponent on the ground. Used on moves like Snake's Down Throw. | Hit |
| 0x1dc3c2ee10 | collision_attr_**leviathan_wave** | 📝 Related to Leviathan on the Midgar Stage. Needs testing. | Unknown |<!-- See this message from PhazoGanon: https://ibb.co/vcXdPt2. Figure out if it's Midgar-specific, what it looks like, and how it differs from the other leviathan wave. -->
| 0x239ec77f51 | collision_attr_**leviathan_wave_owner** | 📝 Related to Leviathan on the Midgar Stage. Needs testing. | Unknown | <!-- See this message from PhazoGanon: https://ibb.co/vcXdPt2. Figure out if it's Midgar-specific, what it looks like, and how it differs from the other leviathan wave. -->
| 0x149ae97939 | collision_attr_**magic** | Spawns magical effects. Used on moves like Zelda's Jab and Ness's Uair. | Visual |
| 0x1f9c885634 | collision_attr_**mario_local_coin** | A variant on the coin effect that summons the Odyssey Coins rather than the normal Mario Coins. | Visual |
| 0x23b3fb791e | collision_attr_**marth_shield_breaker** | 📝 Used only on Marth and Lucina's Shield Breaker. Needs testing. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. -->
| 0x15e069fac2 | collision_attr_**noamal** | Mispelling, does nothing, used exclusively on G&W Down Smash. | None |
| 0x1399ff8a42 | collision_attr_**none** | No visual effect. | None |
| 0x15a2c502b3 | collision_attr_**normal** | Basic hit visual effect. | Visual |
| 0x1cd9e295e5 | collision_attr_**normal_bullet** | 📝 Used only on Bayonetta bullet arts. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. -->
| 0x1c9a85bcd9 | collision_attr_**normal_poison** | 📝 Unsure. Used on the dracula2homingshot article from the Dracula boss and *possibly* the poison floor spirit battle effect. | Unknown | <!-- Figure out what this looks like, what effects it has, and how, if at all, it differs from curse_poison. -->
| 0x197da285b7 | collision_attr_**odin_slash** | 📝 Used only on the Odin summon on Midgar's slash attack. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. Figure out if it's Midgar-specific. -->
| 0x1e4c53bd48 | collision_attr_**palutena_bullet** | 📝 Used only on Palutena's Autoreticle. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. -->
| 0x179c4e24c5 | collision_attr_**paralyze** | Inflicts the paralyze status, freezing the opponent in place for some time. Used on moves like Corrin's Dragon Fang Shot and ZSS' Down Smash. | Hit |
| 0x1d73cd1464 | collision_attr_**paralyze_ghost** | 📝 A variant on the paralyze effect that's exclusively used on the Yuri Kozukata (Fatal Frame) Assist Trophy. Not sure how it differs from paralyze. | Hit | <!-- Figure out how it differs from paralyze, whether purely visually or if there's mechanical differences, such as a different formula to calculate how long the stun lasts, using a different animation for the target, etc. -->
| 0x158507c5b5 | collision_attr_**pierce** | 📝 An effect similar to the crumple status, but with different params in common.prc. Only used on Corrin's Dragon Lunge. | Hit | <!-- Figure out exactly how it differs from other crumple statuses. -->
| 0x177f259087 | collision_attr_**pit_fall** | A variant on the bury effect that spikes in the air. Used on the Pitfall item. | Hit |
| 0x14bf0ee04f | collision_attr_**punch** | Basic punch visual effect. | Visual |
| 0x1569c518b1 | collision_attr_**purple** | Emits shadowy flames. Used on moves like Ganondorf's Warlock Punch, Mewtwo's Fair, Ridley's Space Pirate Rush, and Robin's Nosferatu. | Visual |
| 0x13135c5462 | collision_attr_**rush** | Smaller effect used in a lot of multihits. | Visual |
| 0x153a9c575b | collision_attr_**saving** | Inflicts the crumple status, which causes the opponent to be stunned and fall down. Used on moves like Ryu's Focus Attack. | Hit |
| 0x192a2f9ba6 | collision_attr_**saving_ken** | 📝 A variant on the crumple status that is used on Ken's Focus Attack, uncertain how this differs from standard crumple. | Hit | <!-- Figure out how it differs from Ryu's crumple, whether purely visually or if there's mechanical differences. -->
| 0x1537b0b1f0 | collision_attr_**search** | 📝 Used on the searchboxes of Falcon's Raptor Boost and Pit's Upperdash Dash. | Unknown | <!-- Need a general description of what it looks like. Most likely doesn't have any visual effects, but if it's similar to another hit effect, mention how it differs. Most likely has additional effects. -->
| 0x149cdc52bb | collision_attr_**sleep** | Inflicts the sleep status, which puts opponents to sleep. Used on moves like Jigglypuff's Sing. | Hit |
| 0x1710166637 | collision_attr_**sleep_ex** | A variant on the sleep effect which pops the target up slightly. Used on moves like Hero's Snooze. | Hit |
| 0x131b56c975 | collision_attr_**slip** | Inflicts the slip status, which trips the opponent. Used on Wario's Waft and the Banana item. | Hit |
| 0x133203c0fd | collision_attr_**stab** | 📝 Used only on Ridley's Skewer. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. -->
| 0x14604c88a3 | collision_attr_**sting** | Creates a horizontal piercing effect. Used on stabbing type moves like Roy's Down Tilt and Luigi's Forward Smash. | Visual |
| 0x1d7a481588 | collision_attr_**sting_bowarrow** | 📝 Used exclusively on Byleth's Neutral Special (phase 1). Uncertain how it differs from the sting attribute. | Visual | <!-- Need a general description of what it looks like. If it's similar to sting, mention how it differs. May also have additional effects. -->
| 0x1aa2fd0729 | collision_attr_**sting_flash** | 📝 Used exclusively on Sephiroth's Scintilla. Uncertain how it differs from the sting attribute. | Visual |<!-- Need a general description of what it looks like. If it's similar to sting, mention how it differs. May also have additional effects. -->
| 0x135f399c3b | collision_attr_**stop** | 📝 Used exclusively on Sephiroth's Supernova and Mega Man's Crash Bomber. | Unknown | <!-- Need a general description of what it looks like. If it's similar to another hit effect, mention how it differs. May also have additional effects. -->
| 0x18c8a6895d | collision_attr_**taiyo_hit** | Emits a golden particle ring and lens flare effect. Used on Wii Fit Trainer's Sun Salutation. | Visual |
| 0x13c64f9fca | collision_attr_**turn** | Turns the opponent around. Used on moves like Mario's Cape and Mii Swordfighter's Reversal Slash. | Hit |
| 0x1468dc84cd | collision_attr_**water** | Emits water particles. Used on moves like Squirtle's Down Smash and Corrin's Dragon Ascent. | Visual |
| 0x13933df6fe | collision_attr_**whip** | Similar to the sting & cutup effects. Used on Simon/Richter's Whip Moves. | Visual |

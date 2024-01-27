# Article Cloning Guide (Smashline 2 **only**)
#### Courtesy of PhazoGanon
### Disclaimers:
- Some articles cannot be cloned currently
- Some characters cannot get new articles

### Steps: 
**1. Use Smashline 2**
   - Download the [smashline 2 plugin](https://github.com/HDR-Development/smashline/releases) and put it in your plugins folder
   - Make sure your cargo.toml includes the smashline 2 line (`smashline = { git = "https://github.com/hdr-development/smashline.git" }`) and not the smashline 1 one (`smashline = { git = "https://github.com/blu-dev/smashline.git", branch = "development" }`)

**2. Clone the article using the following line of code:**
```
smashline::clone_weapon(
  /*Agent you want to clone from, such as "mario"*/,
  /*Article you want to clone from, such as "fireball"*/,
  /*Agent you want to clone to, such as "marth"*/,
  /*Name of the new article, such as "landmine"*/,
  /*Use the original code of the article, uses false/true*/
);
```
This code segment can go anywhere in your code, but it's best practice do it in the install section for the character.  
(Notice, if you're cloning something like Mario's Fireball, or any other article that calls to a fighter specific effect, you need to transplant said effect to the agent you're cloning to)

**3. Create a custom lua_const for the article you want to clone. It's best to format it as the following:**
`pub const FIGHTER_<fightername>_GENERATE_ARTICLE_<articlename>: i32 = 0x3;`  
The value (0x3 in this example), should be taken from [this](https://github.com/ultimate-research/param-labels/blob/master/articles/Labels.txt) site.
Go down the list of articles for the character you want to put an article on. Each one has a value in order starting at 0x0. If a character has three articles, they'll have the values 0x0, 0x1, and 0x2, so the one you'd use is 0x3

**4. Copy over any associated assets of the article using a config.json, formatted as the following:**
```
{
    "new_dir_files": {
        "fighter/marth/c00": [
            "fighter/marth/model/landmine/c00/dark_model.numatb",
            "fighter/marth/model/landmine/c00/light_model.numatb",
            "fighter/marth/model/landmine/c00/metamon_model.numatb",
            "fighter/marth/model/landmine/c00/model.nuanmb",
            "fighter/marth/model/landmine/c00/model.nuhlpb",
            "fighter/marth/model/landmine/c00/model.numatb",
            "fighter/marth/model/landmine/c00/model.numdlb",
            "fighter/marth/model/landmine/c00/model.nusktb",
            "fighter/marth/model/landmine/c00/model.nusrcmdlb",
            "fighter/marth/model/landmine/c00/model.xmb",
            "fighter/marth/motion/landmine/c00/motion_list.bin",
            "fighter/marth/motion/landmine/c00/main.nuamb",
            "fighter/marth/motion/landmine/c00/update.prc"
        ]
    }
}
```
Then drag the associated files into the respective folders, and adjust them to how you need them to work.

**5. If it's an article that has a vl.prc entry, add a new entry into the vl.prc of the character you wish to clone the article to, then rename it to the new name of the article**

**6. Generate the article in the same manner you'd generate any other article**

#### Examples of all of this can be found in the other files in the episode46 folder. Happy Modding!

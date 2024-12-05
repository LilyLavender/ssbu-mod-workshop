# Article Cloning Guide (Smashline 2 **only**)
> By PhazoGanon & LilyLambda
### Disclaimers:
- Some articles cannot currently be cloned
- Some characters cannot get new articles
- See [this site](https://github.com/CSharpM7/ArticleCloningInfo) for a list of everything we know about article cloning so far

### Steps: 
___
### 1. Use Smashline 2  
- Download the [smashline 2 plugin](https://github.com/HDR-Development/smashline/releases) and put it in your plugins folder  
- Make sure your cargo.toml includes the line for Smashline 2 (`smashline = { git = "https://github.com/hdr-development/smashline.git" }`) and NOT Smashline 1 (`smashline = { git = "https://github.com/blu-dev/smashline.git", branch = "development" }`)
___
### 2. Clone the article using the following line of code:  
```rust
smashline::clone_weapon(
  /*Agent you want to clone from*/,
  /*Article you want to clone from*/,
  /*Agent you want to clone to*/,
  /*Name of the new article*/,
  /*Whether to use the original code of the article*/
);
```
Example usage:
```rust
smashline::clone_weapon("mario", *WEAPON_KIND_MARIO_FIREBALL, "marth", "landmine", true);
```
> This code segment can go anywhere in your code, but it's best practice do it in the install section for the character.  
(Notice, if you're cloning something like Mario's Fireball, or any other article that calls to a fighter specific effect, you need to copy said effect to the agent you're cloning to)
___
### 3. Create a custom lua_const for the article  
It's best to format it like the following:  
```rust
pub const FIGHTER_<fightername>_GENERATE_ARTICLE_<articlename>: i32 = 0x3;
```
The value (0x3 in this example), should be taken from [this](https://github.com/ultimate-research/param-labels/blob/master/articles/Labels.txt) site.
- Go down the list of articles for the character you want to put an article on.
- Each one has a value in order starting at 0x0.
- If a character has three articles, they'll have the values 0x0, 0x1, and 0x2, so the one you'd use is 0x3
___
### 4. Copy over any associated assets of the article using a config.json, formatted such as the following:
```json
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
___
### 5. If the article has a vl.prc entry... 
- Add a new entry into the vl.prc of the character you wish to clone the article to using param-xml, then rename it to the new name of the article
___
### 6. Generate the article
- You can now generate the article like you would any other: with `ArticleModule::generate_article`
- Any function in `ArticleModule` can now be used with the article
___
> Examples of all of this can be found in the other files in the episode46 folder. Happy Modding!

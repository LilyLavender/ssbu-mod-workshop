# Status Script Formatting
### Before translating a status script, I would recommend doing the following 
Replace the text in the left column with the corresponding in the "Replace with" column. A blank cell means replace with nothing.

| Text | Replace with | Notes |
| ---: | --- | --- |
| `double space` | `\t` | In VSCode, make sure to activate regex |
| `this->moduleAccessor` | `fighter.module_accessor` | Doesn't work in FUN_ functions |
| `__` | `::` | Note that that's two underscores |
| `_impl` |
| `app::lua_bind::` |
| `(float)` |

Remove any full lines with a tilde (`~`) in it.

**These replacements should help format the script better and make translation faster. Happy Modding!**

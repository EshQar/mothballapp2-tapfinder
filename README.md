# A Tap Brute-Forcer
This is a tap brute-forcer integrated into the mothball app by anonnoob. It also has some small changes to the overall mothball app such as `||` being recognized as equivalent to `x(0) z(0) vx(0) vz(0)` and `min()`, `max()` and `abs()` working inside expressions.

>**Known Limitations and issues**\
>Currently there is no handling of walls or edges. All taps in any tap strat must be assumed to be ordered in such a way that they never hit a wall or an edge if it is possible. If it is not possible the strat should be disregarded.
>
>Loading files that were made with earlier versions can be made impossible by some updates
>
>Probably something else so if you suspect you got a bogus output of some kind do notify me! @EshQar on discord
>

# Overview of brute-forcing parameters and functionality

## New buttons
Any XZ simulation section will have three extra buttons: `Goal Type`, `Axis` and `Taps`. 

### Enabling the brute-forcer
By pressing the `Taps` button you can toggle whether the tap brute-forcer should try running when you press the 'Run' button. When enabled it will add fields for you enter your brute-forcing settings to the end of the input field. These fields are simply raw text that get parsed so you can change or remove them, but doing this will make it impossible for the parser to find your settings so only change empty lines or the fields after the colon.

### Changing goal type and axis
The two other buttons are for two special settings for the brute-forcing. They both hide and show certain setting fields and are therefore placed as buttons above. Pressing the axis button alternates the axis from `XZ -> X -> Z` in a loop and updates any potential fields to display only relevant fields. The axis determines which axes the brute-forcer checks for when checking a tap strat and also modifies the output to exclude any axes that are not active (i.e if `axis == "X"` then Z offset is not displayed etc.). The goal type button alternates the goal type from `Mothball -> Min and max -> Target and error` and also updates the fields to hide irrelevant fields. On the `Min and max` setting you will have to enter the minimum and maximum value the active axes should have. Try not to use +-inf if possible although it should be supported. On the `Target and error` setting you will have to enter a target value on the active axes and a corresponding error value. All strats such that $|\text{target} - \text{offset}| < \text{error}$ on the active axes will be returned. If the goal type is `Mothball` then the goal will be determined by the mothball command entered before the partition `-----`. Further information on how to set the goal constraints with mothball can be found in the syntax guide section.
## Main tap brute-forcing settings
The brute-forcer expects every available parameter to be given. If you want to leave a field empty enter `...` and it will ignore this field. Some fields are still mandatory and will error if they are `...` or any other invalid string.

1.
    ### Max taps

    The maximum amount taps a valid tap strat should have.

    
1.
    ### Facing
    
    The facing the brute-forcer should run with. This facing must always be entered if you have a facing other than zero. Even if you set the facing in the mothball command above it must be written here as well.\
    If you wish to check every strat within a range of facings from some `fstart` to some `fend` write `fstart, fend` into the field. You can specify a step `fstep` by writing `fstart, fend, fstep`. For example if we want all facings from $13$ to $20$ we write `13, 20`. To specify a step of $0.01$ we would write `13, 20, 0.01`.

1.
    ### Packages
    Determines the tap packages the brute-forcer will be allowed to use when finding tap strats. The parser will search for packages in your command. The packages should be seperated by spaces. Each package can take extra arguments that modify the package to your specific use case. An example of a package package command is `std air 1`. Further information on what extra arguments each package can take and the syntax for passing these arguments can be found in the syntax guide section.

1.
    ### Corners
    Determines the different corners that tap strats can start from. Write the side length(s) of a rectangular momentum in the X axis and/or the Z axis separated by a space. The side length(s) can be written as such: `[length][axis][type]` where length is a float such as `1.5` or `-.04375`, axis is either `x` or `z` and type is either `mm`, `b` or `""`. Say we are stratting a p2p tyrone in 1.8 with a 1bm sidewalled pane as momentum. We would write `.125zmm -.5x` , assuming the sidewall is on the left and `(0, 0)` is the back left corner, and the brute-forcer checks every strat from the coordinates `(0, 0)`, `(-0.5, 0)`, `(0, 0.725)` and `(-0.5, 0.725)`. If we are trying to stratfind a Z facing 3bm backwalled neo and only care about the corners on one side we can write `3zb` for the corners `(0, 0)` and `(0, 2.4)`.

1.
    ### Sorting type
    Determines the order in which tap strats are displayed in the output field. Can be either `xmin`, `zmin`, `xmax` or `zmax`. The sorting key is determined by the sorting type entered and the list is then sorted from lowest to highest. For example if the sorting type is `xmax` it will show the strats with the lowest distance from `xmax` on the X axis at the top and those with the higest distance from `xmax` at the bottom.

1.
    ### Version
    The version used when finding the tap offsets. Similarly to facing it must be entered independently of the version specified in the mothball above the partition.

1.
    ### Decimal precision
    The decimal precision used when displaying floats in the output field. This should not affect any computation as it is applied after the results have been found.

1.
    ### Slip
    This has not yet been implemented!

### Technical details
| Parameter | Internal alias | Special Constraints | Default value
|:-----|:-----:|:------------:|:------------:|
| Max taps | `n` | Must be a nonzero positive integer | `5` |
| Facing | `f` | none | `0` |
| Facing range start | `fstart` | Must be less than `fend`, goal type must be `mothball` | `0` |
| Facing range end | `fend` | Must be greater than `fstart`, goal type must be `mothball` | `0` |
| Facing range step | `fstep` | Goal type must be `mothball` | `0.05` |
| Packages | `packages` | Cannot be omitted by `...` | `""` |
| Corners | `corners` | none | `""` |
| Sorting type | `sortby` | none | `xmin` |
| Version | `version` | none | `1.8` |

## Syntax guide

### Mothball defined goals
By using mothball defined goals you can define your goal without manually entering in the min and max or target and error values. If you have already found a strat by using mothball you have likely already encoded the constraints in your output commands. For the brute-forcer to parse these constraints out of your mothball command you have to add an argument to every output command that encodes for a specific constraint. This extra argument says whether the output should be greater than or equal to a user-defined constant `x`. For example: in `fmm(12, 1) zmm(1)` the `zmm(1)` command encodes the constraint of using less than one block of momentum. We can add `>0` and get `zmm(1, >0)` to use this constraint. `zmm(1, >0)` states that "the output, `output`, of the command `zmm(1)` should satisfy the inequality $\text{output} > 0$". In general the syntax is `output_cmd(..., <x, ...)` or `output_cmd(..., >x, ...)`. The ellipses signfy that `>x` or `<x` can be in front of or behind every other argument. Its position does not matter so long as it's separated from the other arguments by commas. It also does not need any other arguments to work. With this we can find the best tap strat for max fmm on 1bm like this: `outz(>0) fmm(12, 1) zmm(1, <0)` or equivalently `outz(>0) fmm(12, 1) zmm(<1)`. With `n = 10` and `packages = "std"` we get 3uS 2sS 3rW which leaves us with 0.0006475 unused zmm. Again, tap strats are not ordered automatically so we really got 3rW 3uS 2sS instead.

If your mothball command contains position resets such as `x()`, `z()`, `|` or `||` you will need to take a bit more care. Make sure that right before the position reset there is an output with a point it's centered about. This just means that you need to write something like `zmm(1)` instead of something like `zmm` which would be invalid. The reason for this is that for the brute-forcer to understand any outputs after a reset it needs to know what you "expect" the coordinates to be after the reset. Say we switch to a newer version with our fmm example. In the command `v("1.21.5") outz(>0) fmm(12, 1) zmm(1, <0) | sj(11) zb(4)` the brute-forcer knows to expect a `z` of about one block of momentum forwared which would be `z = 1.6`. Therefore the constraint after the reset, namely `zb(4)` becomes the same as `zmm(1)` + `zb(4)` in a sense, which is equivalent to `outz(5)` which means `v("1.21.5") outz(>0) fmm(12, 1) zmm(1, <0) | sj(11) zb(4)` is the same as `v("1.21.5") outz(>0) fmm(12, 1) zmm(1, <0) sj(11) outz(5)`. If you think about it this makes sense because the distance from the back of momentum block to the front of the landing block is the same as `outz(5)`. If you wish to make the reference point zero you can simply add the parentheses without any argument: `outz()`. If you wish to not use a reference point at all you can replace `||` with `!!`, `|` with `!`, `x()` with `x!()` and `z()` with `z!()`. With this we have `v("1.21.5") outz(>0) fmm(12, 1) zmm(1, <0) | sj(11) zb(4, >0)` which encodes for 1bm tier 4b with max fmm in newer versions. For a use case of something like `!!` we can try finding a tap strat for 1bm backwall 4b with 3/4t pfmm in newer versions. To do this we can for example do this `v("1.21.5") sa(8) s sj(12) zb(5, >0) !! sa(9) s outz(1, <0)`. After you've encoded your constraints in a mothball command simply set the rest of your settings and press run! (Do make sure your goal type is `mothball` of course)

### Using the tap packager
When specifying what packages should be used you write the name of the appropriate package separated by spaces, such as `std`. As of right now all packages have their default value for key set to `wasd`. This means that it will try to add every tap for the directions `W`, `A`, `S` and `D`. `std` contains a shifted, unshifted and sprinted tap, so with `key = wasd` we get all the taps `1sW`, `1sA`, `1sS`, `1sD`, `1uW`, `1uA`, `1uS`, `1uD` and `1rW`. To change the value of `key` from the default we can write `std.[new_key]`. For example `std.wasd+` gives us the same package but for the directions `W`, `A`, `S`, `D`, `WA`, `SA`, `SD` and `WD`. A full table of valid values for `key` can be found below. If you wish to add a modifier to the movement such as being in a ladder you can use the same modifier syntax as in mothball. Hence, `std[ladder]` gives you the same taps but assuming youre in a ladder, although ladder doesn't actually change anything unless you get to a high enough speed, such as with 2trW. I do not recommend using this to add modifiers such as `bl` because these modifiers cannot affect the name of the tap. Instead you should use the dedicated blocked tap packages as they will have distinct names. If you wish to add taps such as 2trW, i.e. timed/longer taps, you can use the syntax `n-mt[package]` or `nt[package]` where n and m are integers. `n-mt` means you will include all taps from nt to mt inclusively in the same package. For example to get 2trW we can do `2tstd`, or if we don't mind anything from 1 to 3 ticks inclusively we can do `1-3tstd`. Some packages can accept extra arguments with `()`. To set these you should do something like `a7(2)`. Full details on all the packages and their possible extra arguments can be found below. If you feel like you're getting too many taps or want to exclude some try using `\` to remove taps. For example `std\u` gives `1sW`, `1sA`, `1sS`, `1sD`, and `1rW` and `std\u\r` gives only `1rW`. Do note that this name is determined without prefix so `a7\a7u` is just `a7` but `a7\u` would actually remove the `a7u` taps. In my opinion the best way to restrict taps however is by restricting the amount from any given package. To restrict the amount enter a valid integer separated by spaces after the rest of the package command. For example, `std a7 1` where std can give up to `Max taps` taps, but a7 is restricted to at most 1. Do note it doesn't like when the maximum amount of taps for every package sums to less than `Max taps` so `std 1 a7 1` would be invalid.

The order of the special arguments such as `[modifiers]` or `.[key]` does not matter (except for `nt` and `n-mt` needing to prefix a package), so `std.wasd+[wt]` is just as valid as `std[wt].wasd+`. If your packages overlap and contain some repeated taps the repeats will be cleared from the larger package. So `std.wasd+ std` would generate one package with only the 45 taps and the other with only the default `.wasd`, so keep this in mind when adding constraints. If you two different packages contain taps that are opposites that will make it possible for the output to give something like `1uD 1uA` and will in general slow down the search. The brute-forcer currently has no way of avoiding cancellations like `1sWD 1sS 1sA` which is the same as nothing in pre-1.21.5, however if you're using this for 1.21.5+ `1sWD 1sS 1sA` can actually be productive. If the search for taps is taking a while and youre on pre-1.21.5 consider not including 45 shifted taps and normal shifted taps at the same time to speed up the search by avoiding combinations like `1sWD 1sS 1sA`.

For anyone who's read this far (and for anyone who hasn't as well) I'm very open to suggestions for new packages or naming conventions.

### Available packages
| Package name | Description | Taps | Prefix | Extra arguments
|:-----|:----------:|:-----:|:----------:|:-----------------:|
| std | All the "normal" taps which are most frequently used. | `s`, `u`, `r` | `""` | none |
| a7 | Taps performed 1t before landing back on the ground. | `s`, `u`, `r` | a7 for default xair, otherwise it is a | `(xair)` : `xair=0`, gives `xair` extra ticks of airtime before landing. |
| air | Taps performed directly after jumping and that stop by the time they land, i.e. 1arW is rarely included because it rarely stops in time. | `s`, `u`, `r` | a | `(airtime, leniency)` : `airtime=12`, determines the assumed airtime of a still standing jump. `leniency=2`, air taps can change if you tap too late, `leniency` removes taps with less than `leniency` distinct ticks that give the same distance. |
| p | Pessi taps, the same as air, but without the requirement of being grounded (grounded meaning that they have to stop before ground slip changes the length of the tap). | `s`, `u`, `r` | p | `(timing, airtime)` : `timing=1`, the amount of ticks after the jump that the tap is performed. `airtime=12`, determines the assumed airtime of a still standing jump. |
| jam | A normal tap, but you perform a jump alongside the tap, the default of this is the same as a 1t jam. | `s`, `u`, `r` | j | `(airtime, run_ticks)` : `airtime=12`, determines the assumed airtime of a still standing jump. `run_ticks=0`, the amount of ticks run before jumping. |
| bstd | All the "normal" taps performed while blocking. | `s`, `u` | `b` | none |
| ba7 | a7 taps performed while blocking | `s`, `u` | a7b for default xair, otherwise it is ab | `(xair)` : `xair=0`, gives `xair` extra ticks of airtime before landing. |
| bair | Air taps performed while blocking. | `s`, `u` | ab | `(airtime, leniency)` : `airtime=12`, determines the assumed airtime of a still standing jump. `leniency=2`, air taps can change if you tap too late, `leniency` removes taps with less than `leniency` distinct ticks that give the same distance. |
| bp | Pessi taps performed while blocking. | `s`, `u` | pb | `(timing, airtime)` : `timing=1`, the amount of ticks after the jump that the tap is performed. `airtime=12`, determines the assumed airtime of a still standing jump. |
| bjam | Jam tap performed while blocking. | `s`, `u` | jb | `(airtime, run_ticks)` : `airtime=12`, determines the assumed airtime of a still standing jump. `run_ticks=0`, the amount of ticks run before jumping. |

### Values for `key` argument
| Value | Iterates over |
|:-----|:------------:|
| `wasd` | `W`, `A`, `S`, `D` |
| `wasd+` | `W`, `A`, `S`, `D`, `WA`, `SA`, `SD`, `WD` |
| `wad` | `W`, `WA`, `WD` |
| `ws` | `W`, `S` |
| `ad` | `A`, `D` |
| `wdwa` | `WA`, `SA`, `SD`, `WD` |
| `w` | `W` |
| `a` | `A` |
| `s` | `S` |
| `d` | `D` |

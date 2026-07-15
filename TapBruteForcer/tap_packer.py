from Interface import mothball_fetch_XZ_as_tap as mothball_tap
from TapBruteForcer.tap import Tap
from TapBruteForcer.parser import packing_parser
import re
import TapBruteForcer.helper as helper
from ExprEval import evaluate
import os
import sys
from collections import defaultdict

def package_path(relative_path):
    if hasattr(sys, "_MEIPASS"):
        return os.path.join(sys._MEIPASS, relative_path)
    return os.path.join(os.path.abspath("."), relative_path)

directional_suffixes = {
    "wasd" :  ("w", "s", "a", "d"), 
    "wad" : ("w", "wa", "wd"), 
    "wasd+" : ("w", "s", "a", "d", "wa", "wd", "sd", "sa"), 
    "ws" : ("w", "s"), 
    "ad" : ("a", "d"), 
    "w" : ("w",), 
    "s" : ("s",), 
    "a" : ("a",), 
    "d" : ("d",),
    "wdwa" : ("wd", "wa", "sa", "sd"),
}

reversible_directional_suffixes = {
    "wasd" :  ("w", "d"), 
    "wasd+" : ("w", "d", "wa", "wd"), 
    "ws" : ("w",), 
    "ad" : ("d",),
    "wad" : (), 
    "w" : (), 
    "s" : (), 
    "a" : (), 
    "d" : (),
    "wdwa" : ("wd", "wa"),
}

def validate_command(cmd):
    cmd = helper.remove_spaces_inside_brackets_and_strip_and_lower(cmd)
    valid_for_directions = ["w", "s", "a", "d", "wa", "wd", "sd", "sa"]
    is_reversible = True

    disallowed_expressions = {
        "|"
        " z("
        " x("
    }

    for expr in disallowed_expressions:
        if expr in cmd:
            raise ValueError(f"\"{expr}\" is not allowed in the mothball command for taps!")
    
    cmd_without_modifier = re.sub(r'\[.*?\]', '', cmd)
    if "s.key" in cmd_without_modifier or "sa.key" in cmd_without_modifier or "sj.key" in cmd_without_modifier:
        valid_for_directions = [direction for direction in valid_for_directions if direction in {"w", "wa", "wd"}]
        is_reversible = False


    funcs: list = cmd.split()
    keys = []

    air = False
    for i in reversed(range(len(funcs))):
        if " air " in funcs[i]:
            air = True

            del funcs[i]

    for func in funcs:
        excluded_funcs = {"st", "sta"}
        _, index = helper.find_next_critical_char(func, 0, {"(", ".", "[", " "})
        func_name = func[:index]
        if func_name in excluded_funcs:
            continue
        if "." in func:
            _, i = helper.find_next_critical_char(func, 0, {"."})
            _, j = helper.find_next_critical_char(func, i, {"(", "["})

            key = func[i+1:j]
            keys.append(key)
        else:
            raise ValueError("Every command must be a movement command ending in \".key\"!")

    if not set(keys) == {"key"}:
        raise ValueError("Only \"key\" is allowed to express direction of keys!")

    validated_command = " ".join(funcs)

    return validated_command, is_reversible, valid_for_directions, air

def package_to_taps(package_name, directions, modifiers, exclude, sim_params, _n, arg_vals):
    def cmd_to_taps(cmd, tap_name, prefix, notation, arg_dict, ground):
        reversible_taps = []
        irreversible_taps = []

        cmd, reversible, valid_directions, air = validate_command(cmd)

        keys = None
        if reversible:
            keys = reversible_directional_suffixes[directions]
            if keys == ():
                keys = directional_suffixes[directions]
                reversible = False
        else:
            keys = directional_suffixes[directions]


        for key in keys:
            if key in valid_directions:
                if not modifiers == "":
                    command = cmd.replace("key", key).replace("modifiers", modifiers)
                else:
                    command = cmd.replace("key", key).replace(",modifiers", "").replace("modifiers", "")
                offset, is_satisified = mothball_tap(command, air, sim_params, ground)
                if is_satisified:
                    tap = Tap(tap_name, prefix, notation, key, offset, arg_dict)
                else:
                    continue

                if reversible:
                    reversible_taps.append(tap)
                else:
                    irreversible_taps.append(tap)
    
        return reversible_taps, irreversible_taps

    rtaps, itaps = [], []
    with open(package_path(f"TapBruteForcer\\packages\\{package_name}.txt"), encoding="utf-8") as package:
        directions, notation, prefix, args, conditions = directions, "[count](_n|×{_n}t)[prefix][name][key]", "", "", ""

        for line in package:
            if not ":" in line:
                break
            
            param, value = line.strip().split(":")
            match param:
                case "key":
                    if directions == None:
                        directions = value
                case "notation":
                    notation = value
                case "prefix":
                    prefix = value
                case "args":
                    args = list(map(lambda x: x.strip(),value.split(",")))
                case "conditions":
                    conditions = value

        arg_dict = {}
        for arg in args:
            if arg == "":
                continue
            if "=" in arg:
                arg_name, arg_val = arg.split("=")
            else:
                arg_name = arg
                arg_val = None
            
            arg_dict[arg_name] = ("default", arg_val)

        if _n == 1:
            arg_dict["_n"] = ("default", _n)
        else:
            arg_dict["_n"] = ("custom", _n)

        if arg_vals != None:
            for arg_val, arg_name in zip(arg_vals, arg_dict.keys()):
                arg_dict[arg_name] = ("custom", arg_val)

        force_stop_on_ground = False
        if "ground" in conditions:
            force_stop_on_ground = True
        if "rejectn" in conditions and _n != 1:
            raise f"The package \"{package_name}\" received the argument _n but did not expect it!"

        for line in package:
            # -----Name Line------ #

            if line.strip() == "":
                continue

            line = line.replace("_", "")

            name = line.strip()
            if name in exclude:
                package.readline()
                continue

            # ----Command Line-----#

            line = package.readline().strip()
            for arg_name in arg_dict.keys():
                line = line.replace(f"{{{arg_name}}}", f"{arg_dict[arg_name][1]}")

            line_rtaps, line_itaps = cmd_to_taps(line, name, prefix, notation, arg_dict, force_stop_on_ground)

            rtaps.extend(line_rtaps)
            itaps.extend(line_itaps)


    if rtaps == itaps == []:
        e1 = f"Empty packages are not allowed but \"{package_name}\" is empty!"
        e2 = " Perhaps none of the taps stopped before landing back on the ground?" if force_stop_on_ground else ""
        raise ValueError(e1 + e2)

    return rtaps, itaps

def packer(cmd, sim_params):
    cmds = cmd.strip().split()

    pools = []
    is_reversible = []
    r_unrestricted_pool = []
    i_unrestricted_pool = []
    max_counts = []
    package_sizes = []

    iterator = iter(range(len(cmds)))
    for i in iterator:
        package, n, m, direction, modifiers, exclusions, args = packing_parser(cmds[i])

        rtaps, itaps = [], []
        for _n in range(n, m + 1):
            temp_rtaps, temp_itaps = package_to_taps(package, direction, modifiers, exclusions, sim_params, _n, args)
            rtaps.extend(temp_rtaps)
            itaps.extend(temp_itaps)
        rtaps, itaps = list((len(package_sizes), tap) for tap in rtaps), list((len(package_sizes), tap) for tap in itaps)
        package_sizes.append(len(rtaps) + len(itaps))


        try:
            max_count = int(cmds[i+1])
            max_counts.append(max_count)

            if rtaps != []:
                pools.append(rtaps)
                is_reversible.append(True)

            if itaps != []:
                pools.append(itaps)
                is_reversible.append(False)

            if rtaps != [] and itaps != []:
                max_counts.append(0)

            next(iterator)
        except:
            r_unrestricted_pool.extend(rtaps)
            i_unrestricted_pool.extend(itaps)

    if r_unrestricted_pool != []:
        max_counts.append("u")
        pools.append(r_unrestricted_pool)
        is_reversible.append(True)

    if i_unrestricted_pool != []:
        max_counts.append("u")
        pools.append(i_unrestricted_pool)
        is_reversible.append(False)

    pools, pools_offset, deleted_pools = cleanse_repeat_taps(pools, package_sizes, sim_params["do_frange"], sim_params["axis"], helper.get_zero_offset(sim_params))
    for index in reversed(deleted_pools):
        del max_counts[index]
        del is_reversible[index]

    max_counts[:] = [int(sim_params["n"]) if count == "u" else count for count in max_counts]
    return max_counts, pools, pools_offset, is_reversible

def cleanse_repeat_taps(pools, sizes, do_frange, axis, zero_offset):
    to_delete = set()
    pools_offset = None
    if do_frange or axis == "XZ":
        pools_offset = list(list(tap[1].offset for tap in pool) for pool in pools)
    elif axis == "X":
        pools_offset = list(list(tap[1].offset[0] for tap in pool) for pool in pools)
    elif axis == "Z":
        pools_offset = list(list(tap[1].offset[1] for tap in pool) for pool in pools)
    pools_package_index = list(list(tap[0] for tap in pool) for pool in pools)
    pools = list(list(tap[1] for tap in pool) for pool in pools)

    for i in range(len(pools)):
        for j in range(len(pools[i])):
            if pools_offset[i][j] == zero_offset:
                to_delete.add((i,j))

    for i1 in range(len(pools)):
        for j1 in range(len(pools[i1])):
            for j2 in range(j1 + 1, len(pools[i1])): # i2 == i1
                if pools_offset[i1][j1] == pools_offset[i1][j2]:
                    if sizes[pools_package_index[i1][j1]] <= sizes[pools_package_index[i1][j2]]:
                        to_delete.add((i1, j2))
                    else:
                        to_delete.add((i1, j1))


            for i2 in range(i1 + 1, len(pools)):
                for j2 in range(len(pools[i2])):
                    if pools_offset[i1][j1] == pools_offset[i2][j2]:
                        if sizes[pools_package_index[i1][j1]] <= sizes[pools_package_index[i2][j2]]:
                            to_delete.add((i2, j2))
                        else:
                            to_delete.add((i1, j1))

    rows = defaultdict(list)
    for i, j in to_delete:
        rows[i].append(j)

    for i, js in rows.items():
        for j in sorted(js, reverse=True):
            del pools[i][j]
            del pools_offset[i][j]

    deleted_pools = []
    for i in reversed(range(len(pools))):
        if pools[i] == []:
            deleted_pools.append(i)
            del pools[i]
            del pools_offset[i]


    return pools, pools_offset, deleted_pools

if __name__ == "__main__":
    params = {

        "n" : 0,
        "f" : "0",
        "do_frange" : False,
        "fstart" : 0,
        "fend" : 0,
        "fstep" : 0.05,
        "xmin" : float("-inf"),
        "xmax" : float("inf"),
        "zmin" : float("-inf"),
        "zmax" : float("inf"),
        "xtarget" : 0,
        "xerror" : float("inf"),
        "ztarget" : 0,
        "zerror" : float("inf"),
        "packages" : "",
        "corners" : "",
        "sortby" : "",
        "version" : "1.21.5",
        "dp" : 6,
        "slip" : 0.6,
        "modifiers" : "",
    }

    text = input()
    print(packer(text, params))
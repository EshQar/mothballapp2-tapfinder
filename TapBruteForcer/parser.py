from Interface import mothball
import re
import TapBruteForcer.helper as helper
import numpy as np
import math
from ExprEval import evaluate

def get_new_params(curr_params, curr_text, strict=True):
    param_names = {
        "Max taps" : "n",
        "Facing" : "f",
        "X min" : "xmin",
        "X max" : "xmax",
        "Z min" : "zmin",
        "Z max" : "zmax",
        "X target" : "xtarget",
        "X max error" : "xerror",
        "Z target" : "ztarget",
        "Z max error" : "zerror",
        "Packages" : "packages",
        "Corners" : "corners",
        "Sorting type" : "sortby",
        "Version" : "version",
        "Decimal precision" : "dp",
        "Slip" : "slip",
    }

    params = curr_params

    mothball_cmd, _, args = curr_text.partition("-----")
    params["mothball"] = mothball_cmd

    if args == "":
        raise SyntaxError("Either partition, \"-----\", between mothball and \'Brute force\' was not found or args were completely empty!")


    expected_params = {
        "n",
        "f",
        "xmin" if (params["axis"] == "X" or params["axis"] == "XZ") and params["goal_type"] == "minmax" else "",
        "zmin" if (params["axis"] == "Z" or params["axis"] == "XZ") and params["goal_type"] == "minmax" else "",
        "xmax" if (params["axis"] == "X" or params["axis"] == "XZ") and params["goal_type"] == "minmax" else "",
        "zmax" if (params["axis"] == "Z" or params["axis"] == "XZ") and params["goal_type"] == "minmax" else "",
        "xtarget" if (params["axis"] == "X" or params["axis"] == "XZ") and params["goal_type"] == "target" else "",
        "xerror" if (params["axis"] == "X" or params["axis"] == "XZ") and params["goal_type"] == "target" else "",
        "ztarget" if (params["axis"] == "Z" or params["axis"] == "XZ") and params["goal_type"] == "target" else "",
        "zerror" if (params["axis"] == "Z" or params["axis"] == "XZ") and params["goal_type"] == "target" else "",
        "packages",
        "corners",
        "sortby",
        "version",
        "dp",
        "slip",
    }

    expected_params.discard("")
    found_params = set()

    for line in args.splitlines():
        line = line.split(":")
        if len(line) == 1:
            continue

        arg_name, arg_val = line
        arg_name, arg_val = arg_name.strip(), arg_val.strip().lower()

        if arg_name in param_names.keys():
            arg_name = param_names[arg_name]
        else:
            continue

#        if "..." in arg_val:
#            found_params.add(arg_name)
#            continue
        if arg_val == "" or arg_val is None:
            continue


        try:
            params[arg_name] = arg_val
            found_params.add(arg_name)

        except:
            print(arg_name, ":", arg_val, "wasnt added")
            pass

        if arg_name == "f":
            if "," in arg_val:
                expected_params.add("fend")
                expected_params.add("fstart")
                expected_params.add("fstep")
                arg_val = tuple(map(lambda x: x.strip(), arg_val.split(",")))

                if len(arg_val) == 3:
                    params["fstart"], params["fend"], params["fstep"] = arg_val
                else:
                    params["fstart"], params["fend"] = arg_val

                params["do_frange"] = True
                if float(params["fstep"]) != 0:
                    params["fsteps"] = math.ceil((float(params["fend"]) - float(params["fstart"]))/float(params["fstep"]))
                else:
                    if strict:
                        raise ZeroDivisionError("fstep cannot be 0!")
                    params["fsteps"] = 0
                found_params.add("f")
                found_params.add("fend")
                found_params.add("fstart")
                found_params.add("fstep")
            else:
                params["do_frange"] = False

    if strict:
        if expected_params - found_params:
            element = next(iter(expected_params - found_params))
            raise SyntaxError(f"Expected paramater {element} but it was not found!")
        elif found_params - expected_params:
            element = next(iter(found_params - expected_params))
            raise SyntaxError(f"Did not expect parameter {element} but it was found!")

    return params

def get_bf_text(current_text, params):
    if params["active"]:
        current_text = current_text.partition("-----")[0].rstrip("\n")

    command = "----------\nBrute force {\n"

    n, f, gt, axis, xmin, zmin, xmax, zmax, xt, xe, zt, ze, pack, corn, sort, ver, dp, slip = params["n"], params["f"], params["goal_type"], params["axis"], params["xmin"], params["zmin"], params["xmax"], params["zmax"], params["xtarget"], params["xerror"], params["ztarget"], params["zerror"], params["packages"], params["corners"], params["sortby"], params["version"], params["dp"], params["slip"]

    args1 = f"Max taps: {n}\nFacing: {f}\n"
    args2 = ""
    if gt == "mothball":
        pass
    elif gt == "minmax":
        if axis == "X":
            args2 = f"X min: {xmin}\nX max: {xmax}\n\n"
        elif axis == "Z":
            args2 = f"Z min: {zmin}\nZ max: {zmax}\n\n"
        else:
            args2 = f"X min: {xmin}\nX max: {xmax}\nZ min: {zmin}\nZ max: {zmax}\n\n"
    elif gt == "target":
        if axis == "X":
            args2 = f"X target: {xt}\nX max error: {xe}\n\n"
        elif axis == "Z":
            args2 = f"Z target: {zt}\nZ max error: {ze}\n\n"
        else:
            args2 = f"X target: {xt}\nX max error: {xe}\nZ target: {zt}\nZ max error: {ze}\n\n"
    else:
        raise SyntaxError("What? Value for 'goal_type' wasn't recognized!")

    args3 = f"Packages: {pack}\nCorners: {corn}\n"
    args4 = f"Sorting type: {sort}\n"
    args5 = f"Version: {ver}\nDecimal precision: {dp}\nSlip: {slip}\n"

    return current_text.rstrip("\n") + "\n\n\n" + command + "\n" + args1 + "\n" + args2 + args3 + "\n" + args4 + "\n" + args5 + "\n}"

def mothball_to_goal(mothball_cmd, axis="XZ"):
    assert axis == axis.upper()
    xpos_output_cmds = {"outx", "xmm", "xb"}
    zpos_output_cmds = {"outz", "zmm", "zb"}
    constraints = []
    xmin, zmin, xmax, zmax = float("-inf"), float("-inf"), float("inf"), float("inf")

    mothball_cmd = helper.remove_spaces_inside_brackets_and_strip_and_lower(mothball_cmd, include_chevrons=False)
    mothball_cmd = helper.remove_superfluous_output_commands(mothball_cmd)
    helper.check_mothball_cmd_for_ref_compatibility(mothball_cmd, axis)

    captured, sanitized_mothball_cmd = helper.capture_critical_commands(mothball_cmd)
    output = mothball(sanitized_mothball_cmd)

    outs = []
    for line in output:
        out_cmd, val = helper.unpack_mothball_output(line)
        if out_cmd in xpos_output_cmds:
            outs.append(val)
        elif out_cmd in zpos_output_cmds:
            outs.append(val)

    def constrain(axis, ineq, x):
        nonlocal xmin, zmin, xmax, zmax
        if axis == "X" and ineq == ">":
            xmin = max(xmin, x)
        elif axis == "Z" and ineq == ">":
            zmin = max(zmin, x)
        elif axis == "X" and ineq == "<":
            xmax = min(xmax, x)
        elif axis == "Z" and ineq == "<":
            zmax = min(zmax, x)

    def init_constraints(constraints, outs):
        for i, constraint in enumerate(constraints):
            if constraint == None:
                continue

            ineq, x, axis, ref = constraint
            tot_ref = sum(map(lambda i: outs[i], ref))

            constrain(axis, ineq, evaluate(x, variables={"px" : 0.0625}) - outs[i] - tot_ref)

    constraints = helper.parse_constraints(captured)
    init_constraints(constraints, outs)


    if xmin == float("-inf") and xmax == float("inf"):
        if axis == "Z":
            goal = (zmin, zmax)
        else:
            raise SyntaxError("Expected X or XZ goal but got no X constraints!")
    elif zmin == float("-inf") and zmax == float("inf"):
        if axis == "X":
            goal = (xmin, xmax)
        else:
            raise SyntaxError("Expected Z or XZ goal but got no Z constraints!")
    elif axis == "XZ":
        goal = ((xmin, zmin), (xmax, zmax))

    if axis == "X":
        goal = (xmin, xmax)
    elif axis == "Z":
        goal = (zmin, zmax)
    elif axis == "XZ":
        goal = ((xmin, zmin), (xmax, zmax))
    else:
        raise ValueError("axis wasn't recognized!")
    
    return goal

def xz_get_goals(params):
    goal_type = params["goal_type"]
    do_frange = params["do_frange"]
    mothball_cmd = params["mothball"]

    goals = []
    if goal_type == "minmax":
        xmin, xmax, zmin, zmax = float(params["xmin"]), float(params["xmax"]), float(params["zmin"]), float(params["zmax"])
        goal = ((xmin, zmin), (xmax, zmax))

        goals.append(goal)

    elif goal_type == "target":
        xt, xe, zt, ze = float(params["xtarget"]), float(params["xerror"]), float(params["ztarget"]), float(params["zerror"])
        goal = ((xt - xe, zt - ze), (xt + xe, zt + ze))

        goals.append(goal)

    elif goal_type == "mothball":
        if mothball_cmd == "":
            ValueError("Mothball command is empty so goal type of mothball does not work!")

        if do_frange:
            goals = [[]]
            start, end, step = float(params["fstart"]), float(params["fend"]), float(params["fstep"])
            assert (step > 0 and start <= end) or (step < 0 or start >= end), "Step needs to go in the direction of end from start!"

            for facing in start + step * np.arange(math.ceil((end - start) / step) + 1):
                mothball_cmd = helper.update_facing(mothball_cmd, facing)
                goal = mothball_to_goal(mothball_cmd)
                goals[0].append(goal)
                
        else:
            goal = mothball_to_goal(mothball_cmd)

            goals.append(goal)

    if do_frange:
        if goal_type != "mothball":
            raise ValueError("A facing range requires mothball defined constraints!")
    
    return goals

def single_axis_get_goals(params):
    goal_type = params["goal_type"]
    do_frange = params["do_frange"]
    mothball_cmd = params["mothball"]
    axis = params["axis"]

    goals = []
    if goal_type == "minmax":
        min, max = float(params[f"{axis.lower()}min"]), float(params[f"{axis.lower()}max"])
        goal = (min, max)


        goals.append(goal)

    elif goal_type == "target":
        t, e = float(params[f"{axis.lower()}target"]), float(params[f"{axis.lower()}error"])
        goal = (t - e, t + e)

        goals.append(goal)

    elif goal_type == "mothball":
        if mothball_cmd == "":
            ValueError("Mothball command is empty so goal type of mothball does not work!")

        if do_frange:
            goals = [[]]
            start, end, step = float(params["fstart"]), float(params["fend"]), float(params["fstep"])

            for facing in start + step * np.arange(math.ceil((end - start) / step) + 1):
                mothball_cmd = helper.update_facing(mothball_cmd, facing)
                goal = mothball_to_goal(mothball_cmd, axis)
                goals[0].append(goal)
                
        else:
            goal = mothball_to_goal(mothball_cmd, axis)

            goals.append(goal)

    if do_frange:
        if goal_type != "mothball":
            raise ValueError("A facing range requires mothball defined constraints!")
        
    return goals

def packing_parser(cmd):
    def isolate_duration(s):
        pattern = re.compile(r"^(\d+)(?:-(\d+))?t")

        m = pattern.match(s)
        if not m:
            return s, 1, 1

        n = int(m.group(1))
        m_val = int(m.group(2)) if m.group(2) is not None else n
        rest = s[m.end():]
        return rest, n, m_val
    
    package, n, m, direction, modifiers, exclusions, args = None, None, None, None, "", [], None
    command, n, m = isolate_duration(cmd)

    char, i = "__", -1
    while True:
        char_prev, i_prev = char, i
        char, i = helper.find_next_critical_char(command, i, {".", "\\", "(", ")", "[", "]"})

        segment = command[i_prev + 1: i]
        match char_prev:
            case "__":
                package = segment
            case ".":
                direction = segment
            case "\\":
                if segment == "u":
                    exclusions.append("_")
                exclusions.append(segment)
            case "[":
                if char != "]":
                    raise SyntaxError("Expected closing bracket but the next critical character did not match!")
                modifiers = segment.strip("[")
            case "(":
                if char != ")":
                    raise SyntaxError("Expected closing parenthesis but the next critical character did not match!")

                args = segment.strip("(").split(",")
                args = tuple(map(lambda arg: arg.strip(), args))

        if char_prev == None and i == len(command):
            break

    return package, n, m, direction, modifiers, exclusions, args



if __name__ == "__main__":
    print(packing_parser("1-3tstd\\r.wasd(1,5,3)"))
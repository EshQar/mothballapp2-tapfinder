import math
import re
from collections import defaultdict
from BaseMothballSimulation import parse as split_mothball_cmd

def sign(x):
    if x > 0:
        return 1
    elif x < 0:
        return -1
    else:
        return 0


def update_facing(mothball, facing):
    if "f(" in mothball or "face(" in mothball or "facing(" in mothball:

        mothball = re.sub(
            r'\b(f|face|facing)\([^()]*\)',
            rf'\1({facing})',
            mothball,
            count=1
        )
    else:
        mothball = f"f({facing}) " + mothball

    return mothball

def unpack_mothball_output(line):
    try:
        return line[1][0], float(line[1][-1]) * float(line[1][-2].strip() + "1")
    except ValueError:
        return line[1][0], float(line[1][-1])
    
def facings_to_string(facings, step, dp=3):
    segments = []
    prev_i = 0
    facings = tuple(map(lambda f: round(f, dp), facings))

    for i in range(len(facings) - 1):
        if not math.isclose(facings[i] + step, facings[i + 1]):
            segments.append((facings[prev_i], facings[i]))
            prev_i = i + 1

    segments.append((facings[prev_i], facings[-1]))

    facing_segments = "for facings between "
    for i in range(len(segments) - 1):
        facing_segments += f"f{segments[i][0]} and f{segments[i][1]} or between "

    facing_segments += f"f{segments[-1][0]} and f{segments[-1][1]}"

    return facing_segments

def remove_spaces_inside_brackets_and_strip_and_lower(s, include_chevrons=True):
    s = s.strip().lower()

    result = []
    stack = []

    if include_chevrons:
        pairs = {"(": ")", "[": "]", "{": "}", "<": ">"}
    else:
        pairs = {"(": ")", "[": "]", "{": "}"}

    for c in s:
        if c in pairs:
            stack.append(pairs[c])
            result.append(c)
        elif stack and c == stack[-1]:
            stack.pop()
            result.append(c)
        elif c == " " and stack:
            continue
        else:
            result.append(c)

    return "".join(result)

def capture_critical_commands(mothball_cmd):
    def sanitize_args(args):
        args = tuple(map(lambda x: x.strip(), args.split(",")))

        new_args = []
        for arg in args:
            if "<" in arg or ">" in arg:
                continue
            else:
                new_args.append(arg)

        return ",".join(new_args)

    def repl(match):
        if match.group(3):
            captured.append(("|", ()))
            return match.group(3)
        
        cmd = match.group(1)
        args = match.group(2)

        captured.append((cmd, args))

        return f"{cmd}({sanitize_args(args)})"

    cmds = {"outx", "outz", "xmm", "zmm", "xb", "zb", "x", "z"}

    captured = []

    pattern = re.compile(
        rf"\b({'|'.join(map(re.escape, cmds))})\((.*?)\)|( \| )",
        re.DOTALL,
    )

    sanitized_mothball_cmd = pattern.sub(repl, mothball_cmd).replace(" !| ", " | ")

    return captured, sanitized_mothball_cmd

def check_mothball_cmd_for_ref_compatibility(mothball_cmd):
    split_mothball_cmd = mothball_cmd.split()
    xpos_output_cmds = {"outx", "xmm", "xb"}
    zpos_output_cmds = {"outz", "zmm", "zb"}

    e1 = SyntaxError("Incorrect syntax: have you made sure | or z() or x() is preceded by corresponding output commands with reference points?")
    e2 = SyntaxError("Error while processing constraints: have you made sure | or z() or x() is preceded by corresponding output commands with reference points?")
    try:
        for i, cmd in enumerate(split_mothball_cmd):
            if cmd == "|":
                prev_1, prev_2 = split_mothball_cmd[i - 1].split("("), split_mothball_cmd[i - 2].split("(")
                cmd_prev_1, cmd_prev_2 = prev_1[0], prev_2[0]
                args_prev_1, args_prev_2 = prev_1[1].split(","), prev_2[1].split(",")

                if not (cmd_prev_1 in xpos_output_cmds or cmd_prev_2 in xpos_output_cmds):
                    raise e1
                elif not (cmd_prev_1 in zpos_output_cmds or cmd_prev_2 in zpos_output_cmds):
                    raise e1
                
#                if all(map(lambda x: not ("<" in x or ">" in x), args_prev_1)):
#                    raise e1
#                elif all(map(lambda x: not ("<" in x or ">" in x), args_prev_2)):
#                    raise e1
                
    except IndexError as e:
        print(e, "\n", e2)
    except SyntaxError as e:
        print(e)
    
def remove_superfluous_output_commands(mothball_cmd):
    output_cmds = {"outx", "outz", "xmm", "zmm", "xb", "zb", "outvx", "outvz", "vec", "print"}
    other_output_cmds = {"outvx", "outvz", "vec", "print"}

    for cmd in output_cmds:
        mothball_cmd = mothball_cmd.replace(f"{cmd} ", "")

    mothball_cmd = mothball_cmd.split()
    for i in range(len(mothball_cmd)):
        for cmd in other_output_cmds:
            if cmd in mothball_cmd[i]:
                mothball_cmd[i] = ""

    mothball_cmd = " ".join(mothball_cmd)
    return mothball_cmd

def parse_constraints(captured):
    xpos_output_cmds = {"outx", "xmm", "xb"}
    zpos_output_cmds = {"outz", "zmm", "zb"}
    other_output_cmds = {"outvx", "outvz", "vec", "print"}

    def constraint(args, axis, ref):
        args = tuple(map(lambda x: x.strip(), args.split(",")))

        for arg in args:
            if "<" in arg:
                x = arg.split("<")[1]
                return ("<", x, axis, ref)
            elif ">" in arg:
                x = arg.split(">")[1]
                return (">", x, axis, ref)
            else:
                continue
        
        return None

    constraints = []

    x_ref = set()
    z_ref = set()

    cmd_indx = -1
    prev_xcmd = 0
    prev_zcmd = 0
    for cmd, args in captured:

        contains_constraint = any(map(lambda x: ("<" in x or ">" in x), args))
        if cmd in xpos_output_cmds and contains_constraint:
            constraints.append(constraint(args, "x", tuple(x_ref)))
            cmd_indx += 1
            prev_xcmd = cmd_indx
        elif cmd in zpos_output_cmds and contains_constraint:
            constraints.append(constraint(args, "z", tuple(z_ref)))
            cmd_indx += 1
            prev_zcmd = cmd_indx
        elif cmd in xpos_output_cmds and not contains_constraint:
            constraints.append(None)
            cmd_indx += 1
            prev_xcmd = cmd_indx
        elif cmd in zpos_output_cmds and not contains_constraint:
            constraints.append(None)
            cmd_indx += 1
            prev_zcmd = cmd_indx
        elif cmd in other_output_cmds:
            raise SyntaxError(f"{cmd} is not supported for mothball constraints")
        elif cmd == "|":
            x_ref.add(prev_xcmd)
            z_ref.add(prev_zcmd)
        elif cmd == "x":
            x_ref.add(prev_xcmd)
        elif cmd == "z":
            z_ref.add(prev_zcmd)

    return constraints

def cleanse_repeat_taps(pools, pools_offset):
    to_delete = set()

    for i1 in range(len(pools)):
        for j1 in range(len(pools[i1])):
            for j2 in range(j1 + 1, len(pools[i1])): # i2 == i1
                if pools_offset[i1][j1] == pools_offset[i1][j2]:
                    to_delete.add((i1, j2))


            for i2 in range(i1 + 1, len(pools)):
                for j2 in range(len(pools[i2])):
                    if pools_offset[i1][j1] == pools_offset[i2][j2]:
                        to_delete.add((i2, j2))

    rows = defaultdict(list)
    for i, j in to_delete:
        rows[i].append(j)

    for i, js in rows.items():
        for j in sorted(js, reverse=True):
            del pools[i][j]
            del pools_offset[i][j]

    return pools, pools_offset

def find_next_critical_char(s, i, crit):
    critical_characters = crit

    for i in range(i + 1, len(s)):
        if s[i] in critical_characters:
            return s[i], i
        
    return None, len(s)

def get_inbetween(s: str, i, start, end, return_extra=False):
    index_start = s.find(start, i) + len(start) - 1
    index_end = s.find(end, index_start + 1)

    if index_start == -1 or index_end == -1:
        if return_extra:
            return None, None, None, None
        else:
            return None

    if return_extra:
        return s[index_start + 1:index_end], s[:index_start] + s[index_end + 1:], index_start, index_end

    return s[index_start + 1:index_end]

def replace_between(s: str, i, start, end, func):
    insides, _, i, j = get_inbetween(s, i, start, end, return_extra=True)
    if insides == None:
        return s

    return s[:i] + func(insides) + s[j + 1:]

def fetch_parts(s):
    match = re.match(r'([+-]?(?:\d+\.\d*|\.\d+|\d+)(?:[eE][+-]?\d+)?)(.*)', s)

    if match:
        number = float(match.group(1))
        rest = match.group(2)

    return number, rest

def corner_parser(s):
    tokens = s.split()
    if len(tokens) > 2:
        SyntaxError("Didn't expect more than 2 args for corners for multi axis!")
    
    extra_dists =  {"mm" : 0.6, "b" : -.6, "" : 0}

    corners = [(0, 0)]
    for token in tokens:
        length, rest = fetch_parts(token)
        axis = rest[0].lower()

        length += extra_dists[rest[1:]] * sign(length)

        new_corners = []
        if axis == "x":
            new_corners = list(map(lambda p: (p[0] + length, p[1]), corners))
        elif axis == "z":
            new_corners = list(map(lambda p: (p[0], p[1] + length), corners))
        else:
            ValueError("Expected either x or z in corner arg after the float!")

        corners.extend(new_corners)

    return corners

def single_axis_corner_parser(s):
    tokens = s.split()
    if len(tokens) > 1:
        SyntaxError("Didn't expect more than 1 args for corners for single axis!")
    
    extra_dists =  {"mm" : 0.6, "b" : -.6}


    length, rest = fetch_parts(s)
    length = extra_dists[rest[0:]] * sign(length)
    corners = [0, length]

    return corners

def shift_by_point(q, point):
    if type(q) == float:
        return point + q
    elif type(q) == list or type(q) == tuple:
        return (point[0] + q[0], point[1] + q[1])
    else:
        raise TypeError("Argument q wasn't of an anticipated type!")

def shift_goal_by_point(point, goal):
    start, end = goal
    return (shift_by_point(point, start), shift_by_point(point, end))

def shift_list_of_goals_by_point(point, list):
    return tuple(map(lambda goal: shift_goal_by_point(point, goal), list))

def sanitize_mothball_cmd(mothball_cmd: str):
    n = len(mothball_cmd)
    i = mothball_cmd.find("<")
    while i != -1:
        j1 = max(mothball_cmd.rfind("(", 0, i) + 1, mothball_cmd.rfind(",", 0, i))
        temp1 = mothball_cmd.find(")", i, n) - 1
        temp2 = mothball_cmd.find(",", i, n)
        j2 = min(temp1 if temp1 != -2 else float("inf"), temp2 if temp2 != -1 else float("inf"))
        mothball_cmd = mothball_cmd[:j1] + mothball_cmd[j2 + 1:]

        i = mothball_cmd.find("<")

    i = mothball_cmd.find(">")
    while i != -1:
        j1 = max(mothball_cmd.rfind("(", 0, i) + 1, mothball_cmd.rfind(",", 0, i))
        temp1 = mothball_cmd.find(")", i, n) - 1
        temp2 = mothball_cmd.find(",", i, n)
        j2 = min(temp1 if temp1 != -2 else i, temp2 if temp2 != -1 else i)
        mothball_cmd = mothball_cmd[:j1] + mothball_cmd[j2 + 1:]

        i = mothball_cmd.find(">")

    return mothball_cmd

if __name__ == "__main__":
    print(remove_spaces_inside_brackets_and_strip_and_lower("v(\"1.21.5\") f(-16) sa.wd(8) s.wd zmm(.125, <0) outx(-.5625, >0) | sj sa.wa(8) outx(.5625, <0) x(0) sa.wa outz(2.4375, >0) sa.wa(2) outx(.4375, >0)", include_chevrons=False))
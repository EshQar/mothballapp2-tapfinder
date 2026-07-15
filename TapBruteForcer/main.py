from TapBruteForcer.tap_packer import packer
from brute_forcer import brute_force
from TapBruteForcer.tap import TapStrat
from TapBruteForcer.parser import xz_get_goals, single_axis_get_goals
import TapBruteForcer.helper as helper
import math

def find_tap_strats(params):
    for key in params.keys():
        if params[key] == "...":
            params[key] = None
        elif params[key] == "" or params[key] is None:
            raise RuntimeError("Found empty param, but it was not specified as empty by using '...'!")

    assert not params["n"] is None and 0 < int(params["n"]), "Max taps must be a nonzero positive integer!"
    assert 0 < int(params["dp"]), "dp must be a nonzero positive integer!"
    assert not params["packages"] is None , "Packages cannot be empty!"

    n = int(params["n"])
    dp = int(params["dp"])

    packing_cmd = params["packages"]
    goal_type = params["goal_type"]

    do_frange = params["do_frange"]

    fstart, fend, fstep = float(params["fstart"]), float(params["fend"]), float(params["fstep"])
    if do_frange:
        assert fstart < fend
    fsteps = math.ceil((fend - fstart)/fstep)

    max_counts, pools, pools_offset, is_reversible = packer(packing_cmd, params)

    corners = None
    match params["axis"]:
        case "XZ":
            axis = 2
            goals = xz_get_goals(params)

            pools_offset = [[tap.offset for tap in pool] for pool in pools]

            corners_cmd = params["corners"]
            if corners_cmd != "" and not corners_cmd is None:
                corners = helper.corner_parser(corners_cmd)
        case "X":
            axis = 0
            goals = single_axis_get_goals(params)

            if do_frange:
                pools_offset = [[tap.offset for tap in pool] for pool in pools]
            else:
                pools_offset = [[tap.offset[0] for tap in pool] for pool in pools]

            corners_cmd = params["corners"]
            if corners_cmd != "" and not corners_cmd is None:
                corners = helper.single_axis_corner_parser(corners_cmd)
        case "Z":
            axis = 1
            goals = single_axis_get_goals(params)

            if do_frange:
                pools_offset = [[tap.offset for tap in pool] for pool in pools]
            else:
                pools_offset = [[tap.offset[1] for tap in pool] for pool in pools]

            corners_cmd = params["corners"]
            if corners_cmd != "" and not corners_cmd is None:
                corners = helper.single_axis_corner_parser(corners_cmd)

    if corners != None:
        if do_frange:
            assert len(goals) == 1, "Something went wrong, annoy esh"
            goals = [helper.shift_list_of_goals_by_point(corner, goals[0]) for corner in corners]
        else:
            assert len(goals) == 1, "Something went wrong, annoy esh"
            goals = [helper.shift_goal_by_point(corner, goals[0]) for corner in corners]

    sort_key = lambda x: None
    match params["sortby"]:
        case None:
            if axis != 1:
                sort_key = lambda strat: strat.dists["xmin"]
            else:
                sort_key = lambda strat: strat.dists["zmin"]
        case "xmin":
            sort_key = lambda strat: strat.dists[params["sortby"]]
        case "zmin":
            sort_key = lambda strat: strat.dists[params["sortby"]]
        case "xmax":
            sort_key = lambda strat: strat.dists[params["sortby"]]
        case "zmax":
            sort_key = lambda strat: strat.dists[params["sortby"]]


    #----------------------------------------------------------------------------------------------------#


    assert 0 < n, "Required: Max taps > 0"
    assert isinstance(n, int)
    assert all((count >= 0) for count in max_counts), "Invalid max_counts or is_reversible"
    assert sum(max_counts) >= n, "The sum of max counts cannot be less than n"
    try:
        strats = brute_force(n, max_counts, pools_offset, is_reversible, goals, axis, fstart, fstep, fsteps)
    except BaseException as e:
        raise RuntimeError(f"The brute-forcer encountered an error: {e}")
    tap_strats = [TapStrat(*strat, params) for strat in strats]
    TapStrat.pools = pools

    if tap_strats == []:
        return [(8, ("No strats found!",))]

    def printer(tap_strat):
        def to_addable_string(potential_string):
            if potential_string == None:
                return ""
            else:
                return str(potential_string)

        axis = params["axis"]

        taps = to_addable_string(tap_strat)
        if axis == "XZ":
            offset = to_addable_string(f"({round(tap_strat.offset[0], dp)}, {round(tap_strat.offset[1], dp)})")
            dists = to_addable_string(f"Dists: xmin = {round(tap_strat.dists['xmin'], dp)}, zmin = {round(tap_strat.dists['zmin'], dp)} xmax = {round(tap_strat.dists['xmax'], dp)}, zmax = {round(tap_strat.dists['zmax'], dp)}")
        elif axis == "X":
            offset = to_addable_string(f"{round(tap_strat.offset, dp)}")
            dists = to_addable_string(f"Dists: xmin = {round(tap_strat.dists['xmin'], dp)}, xmax = {round(tap_strat.dists['xmax'], dp)}")
        elif axis == "Z":
            offset = to_addable_string(f"{round(tap_strat.offset, dp)}")
            dists = to_addable_string(f"Dists: zmin = {round(tap_strat.dists['zmin'], dp)}, zmax = {round(tap_strat.dists['zmax'], dp)}")

        facings = to_addable_string(tap_strat.facings)
        goal = to_addable_string(tap_strat.goal)

        parts = (taps, offset, dists, facings, goal)
        output = " ".join(parts)
        return output

    tap_strats.sort(key=sort_key)
    return list(map(lambda strat: (8, (printer(strat),)), tap_strats))
    




if __name__ == "__main__":
    xmin, xmax = -0.071248, -0.065248
    zmin, zmax = -0.520, -0.503319

    sim_params = {}
    sim_params["version"] = "1.21.5"
    sim_params["facing"] = 46.3

    goal = ((xmin, zmin), (xmax, zmax), "XZ")
    find_tap_strats(5, "a7 1 air 1 std", (goal,), sim_params)

    #pools, max_counts = packer("air 2 std\\u.wasd+")

    #print(tuple(map(lambda x: tuple(map(lambda y: y.get_name(-1), x)), pools)))
    #print(tuple(map(lambda x: x, max_counts)))

    #print(tuple(map(lambda x: x.get_name(1),rtaps)),tuple(map(lambda x: x.get_name(1),itaps)))
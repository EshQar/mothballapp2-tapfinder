from MothballSimulationXZ import PlayerSimulationXZ

def mothball(input_str):
    p = PlayerSimulationXZ()
    p.simulate(input_str, suppress_exception=False)

    return p.output

def mothball_fetch_XZ_as_tap(input_str, air, sim_params, ground):
    satisfied = True

    if air:
        input_str += f" sta(50)"
        if ground:
            raise SyntaxError("Grounded condition makes no sense if the tap waits permanently in air!")
    elif ground:
        input_str += " sta(2)"
    else:
        input_str += f" st(10)"

    version = sim_params["version"]
    facing = sim_params["f"]

    if "," in facing:
        facing = facing.split(",")[0].strip()


    input_str = f"version(\"{version}\") sndel(false) f({facing}) " + input_str + " outx outz outvx outvz"
    p = PlayerSimulationXZ()
    p.simulate(input_str, suppress_exception=False)
    
    data = p.output

    z = [float(value) for _, (name, _, value) in data if name == "outz"][-1]
    x = [float(value) for _, (name, _, value) in data if name == "outx"][-1]

    vz = [float(value) for _, (name, _, value) in data if name == "vx"][-1]
    vx = [float(value) for _, (name, _, value) in data if name == "vz"][-1]

        

    if not (vx == vz == 0):
        if ground:
            satisfied = False
        else:
            raise Exception("Program didn't wait long enough for taps to stop, add more st or sta to commands")

    return (x, z), satisfied
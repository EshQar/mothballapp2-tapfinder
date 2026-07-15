from MothballSimulationXZ import PlayerSimulationXZ
import math
import numpy as np

def mothball(input_str):
    p = PlayerSimulationXZ()
    p.simulate(input_str, suppress_exception=False)

    return p.output

def mothball_fetch_XZ_as_tap(base_input_str, air, sim_params, ground):
    satisfied = True

    if air:
        base_input_str += f" sta(50)"
        if ground:
            raise SyntaxError("Grounded condition makes no sense if the tap waits permanently in air!")
    elif ground:
        base_input_str += " sta(2)"
    else:
        base_input_str += f" st(10)"

    version = sim_params["version"]
    facing = sim_params["f"]

    offset = []
    if sim_params["do_frange"]:
        start, end, step = float(sim_params["fstart"]), float(sim_params["fend"]), float(sim_params["fstep"])

        input_str = f"version(\"{version}\") sndel(false) "
        for facing in start + step * np.arange(math.ceil((end - start) / step) + 1):
            input_str += f"f({facing}) " + base_input_str + " outx outz outvx outvz"
            input_str += " || "

        p = PlayerSimulationXZ()
        p.simulate(input_str, suppress_exception=False)
        data = p.output

        x, z, vx, vz = [], [], [], []
        for _, (name, _, val) in data:
            if name == "outx":
                x.append(float(val))
            elif name == "outz":
                z.append(float(val))
            elif name == "vx":
                vx.append(float(val))
            elif name == "vz":
                vz.append(float(val))

        for x0, z0, vx0, vz0 in zip(x, z, vx, vz):
            if not (vx0 == vz0 == 0):
                if ground:
                    satisfied = False
                else:
                    raise Exception("Program didn't wait long enough for taps to stop, add more st or sta to commands")
        
            offset.append((x0, z0))
    else:
        input_str = f"version(\"{version}\") sndel(false) f({facing}) " + base_input_str + " outx outz outvx outvz"
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

        offset = (x, z)

    return offset, satisfied
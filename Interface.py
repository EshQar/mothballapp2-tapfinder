from MothballSimulationXZ import PlayerSimulationXZ
import math
import numpy as np
from itertools import zip_longest
from dummy_mothball import mothball as mothball_d

def mothball(input_str):
    p = PlayerSimulationXZ()
    p.simulate(input_str, suppress_exception=False)

    return p.output

def mothball_fetch_XZ_as_tap(base_input_str, air, sim_params, ground):
    assert base_input_str != "", "Didn't receive a string!"
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
    slip = None
    try:
        slip = float(sim_params["slip"])
    except:
        ValueError(f"Value {sim_params['slip']} for slip is not valid!")

    offset = []
    if sim_params["do_frange"]:
        start, end, step = float(sim_params["fstart"]), float(sim_params["fend"]), float(sim_params["fstep"])

        input_str = f"pre(16) version(\"{version}\") sndel(false) slip({slip}) "
        for facing in start + step * np.arange(math.ceil((end - start) / step) + 1):
            input_str += f"f({facing}) " + base_input_str + " outx outz outvx outvz"
            input_str += " || "

#        p = PlayerSimulationXZ()
#        p.simulate(input_str, suppress_exception=False)
#        data = p.output
        data = mothball_d(input_str, False, {}, False)
#        check_outputs(data, data_d)

        x, z, vx, vz = [], [], [], []
        for name, _, val in data:
            if name == "outx":
                x.append(float(val))
            elif name == "outz":
                z.append(float(val))
            elif name == "vx":
                vx.append(float(val))
            elif name == "vz":
                vz.append(float(val))

        assert len(x) == len(z) == len(vx) == len(vz), "lens werent equal!"
        for x0, z0, vx0, vz0 in zip(x, z, vx, vz):
            if not (vx0 == vz0 == 0):
                if ground:
                    satisfied = False
                else:
                    raise Exception("Program didn't wait long enough for taps to stop, add more st or sta to commands")
        
            offset.append((x0, z0))

    else:
        input_str = f"pre(16) version(\"{version}\") sndel(false) slip({slip}) f({facing}) " + base_input_str + " outx outz outvx outvz"
        data = mothball_d(input_str, False, {}, False)

        z = [float(value) for name, _, value in data if name == "outz"][-1]
        x = [float(value) for name, _, value in data if name == "outx"][-1]

        vz = [float(value) for name, _, value in data if name == "vx"][-1]
        vx = [float(value) for name, _, value in data if name == "vz"][-1]
        
        if not (vx == vz == 0):
            if ground:
                satisfied = False
            else:
                raise Exception("Program didn't wait long enough for taps to stop, add more st or sta to commands")

        offset = (x, z)

    return offset, satisfied

def check_outputs(data, data_d):
    x, z, vx, vz = [], [], [], []
    xd, zd, vxd, vzd = [], [], [], []
    for _, (name, _, val) in data:
        if name == "outx":
            x.append(float(val))
        elif name == "outz":
            z.append(float(val))
        elif name == "vx":
            vx.append(float(val))
        elif name == "vz":
            vz.append(float(val))

    for name, _, val in data_d:
        if name == "outx":
            xd.append(float(val))
        elif name == "outz":
            zd.append(float(val))
        elif name == "vx":
            vxd.append(float(val))
        elif name == "vz":
            vzd.append(float(val))

    print("bef loop")
    for x0, z0, vx0, vz0, xd0, zd0, vxd0, vzd0 in zip_longest(x, z, vx, vz, xd, zd, vxd, vzd):
        if (x0, z0, vx0, vz0) != (xd0, zd0, vxd0, vzd0):
            print("################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################")
        print("1", (x0, z0, vx0, vz0) == (xd0, zd0, vxd0, vzd0))
        print("2", f"\t{(x0, z0, vx0, vz0)}")
        print("3", f"\t{(xd0, zd0, vxd0, vzd0)}")
        print()

if __name__ == "__main__":
    string = 'version("1.21.5") sndel(false) snj.d[](1) outx outz outvx outvz'
    out = mothball_d(string, False, {}, False)
    p = PlayerSimulationXZ()
    p.simulate(string, suppress_exception=False)
    check_outputs(p.output, out)
    
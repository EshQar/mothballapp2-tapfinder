from itertools import chain
from TapBruteForcer.helper import facings_to_string, get_inbetween
from ExprEval import evaluate
import TapBruteForcer.helper as helper

class Tap():
    def __init__(self, name, prefix, notation, key, offset, arg_dict):
        self.name = name
        self.prefix = prefix
        self.notation = notation
        self.key = key
        self.offset = offset
        self.args = arg_dict

    def get_name(self, count):
        opposites = {"w" : "s", "s" : "w", "a" : "d", "d" : "a", "wa" : "sd", "wd" : "sa", "sd" : "wa", "sa" : "wd"}

        if not "[" in self.notation:
            raise ValueError(f"Invalid notation syntax for tap {self.name}")
        if count == 0:
            return ""
        
        key = self.key
        if count < 0:
            key = opposites[key]
            count *= -1

        notation = self.notation
        i = 0

        def potentially_evaluate(s):
            try:
                return str(evaluate(s))
            except:
                return s

        while True:
            inside, clean_notation, i, j = get_inbetween(notation, i, "(", ")", return_extra=True)

            if inside == None:
                break

            bool_expr, syntax = inside.split("|")
            for arg_name in self.args.keys():
                statement = self.args[arg_name][0] == "custom"
                bool_expr = bool_expr.replace(f"{arg_name.strip()}", f"{statement}")

            bool_expr = bool_expr.replace("count", str(count)).replace("key", str(key)).replace("name", str(self.name)).replace("prefix", str(self.prefix))

            bool_expr = helper.replace_between(bool_expr, 0, "<", ">", potentially_evaluate)

            if not eval(bool_expr): # OOooOOOoOhh spOOoooOOky!
                notation = clean_notation
                continue
            
            for arg in self.args.keys():
                syntax = syntax.replace(f"{{{arg}}}", f"{self.args[arg][1]}")

            syntax = helper.replace_between(syntax, 0, "<", ">", potentially_evaluate)

            notation = notation[:i] + syntax + notation[j + 1:]
            i += len(syntax)

        return notation.replace("[count]", f"{count}").replace("[name]", self.name).replace("[key]", key.upper()).replace("[prefix]", self.prefix)
    
    def __repr__(self):
        return self.get_name(1)
    
class TapStrat():
    pools = None

    def __init__(self, weights, offset, dists, for_goal, facings, params):
        self.weights = weights

        axis = params["axis"]
        if axis == "XZ" :
            self.offset = offset
            self.dists = {"xmin" : dists[0], "zmin" : dists[1], "xmax" : dists[2], "zmax" : dists[3]}
        else:
            if axis == "X":
                self.offset = offset[0]
                self.dists = {"xmin" : dists[0], "xmax" : dists[2]}
            elif axis == "Z":
                self.offset = offset[1]
                self.dists = {"zmin" : dists[1], "zmax" : dists[3]}


        if for_goal != None:
            self.goal = "for goal: " + str(for_goal)
        else:
            self.goal = None

        if facings != None:
            self.facings = facings_to_string(facings, float(params["fstep"]))
        else:
            self.facings = None
    
    def __str__(self):
        pools_names = [[self.pools[i][j].get_name(self.weights[i][j]) for j in range(len(self.pools[i]))] for i in range(len(self.pools))]
        taps = tuple(chain.from_iterable(pools_names))

        taps = [tap for tap in taps if tap != ""]
        return " ".join(taps)

    def __repr__(self):
        pools_names = [[self.pools[i][j].get_name(self.weights[i][j]) for j in range(len(self.pools[i]))] for i in range(len(self.pools))]
        taps = tuple(chain.from_iterable(pools_names))

        taps = [tap for tap in taps if tap != ""]
        return " ".join(taps)
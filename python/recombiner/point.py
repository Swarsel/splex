from abc import ABC

import math
from random import randint
from random import sample
from instance import Instance
from solution import Solution
from recombiner.recombiner import Recombiner


class PointCrossoverRecombiner(Recombiner, ABC):

    def recombine(self, instance: Instance, population: list[Solution], size: int) -> list[Solution]:
        out = []
        num_offspring = math.ceil(instance.parameters["offspring_ratio"] * size)
        for _ in range(num_offspring):
            parents: list[Solution] = sample(population, 2)
            kid = Solution(instance)
            point = randint(1, instance.n)
            at_point = 0
            for i in range(1, instance.n + 1):
                for j in range(i + 1, instance.n + 1):
                    if at_point < point:
                        if parents[0].graph.get_edge_status(i, j):
                            kid.add_edge(i, j)
                        else:
                            kid.remove_edge(i, j)
                    else:
                        if parents[1].graph.get_edge_status(i, j):
                            kid.add_edge(i, j)
                        else:
                            kid.remove_edge(i, j)
                    at_point += 1
            # kid.construct()
            out.append(kid)
        return out

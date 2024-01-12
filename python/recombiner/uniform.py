from abc import ABC

import math
from random import randint
from random import sample
from instance import Instance
from solution import Solution
from recombiner.recombiner import Recombiner
import multiprocessing

class UniformCrossoverRecombiner(Recombiner, ABC):

    def recombine(self, instance: Instance, population: list[Solution], size: int) -> list[Solution]:
        num_offspring = math.ceil(instance.parameters["offspring_ratio"] * size)
        with multiprocessing.Pool() as p:
            out = p.starmap(recombine, [(instance, population) for _ in range(num_offspring)])
        # print(f"after repairing {time1 - time()}")

        return out


def recombine(instance, population):
    parents: list[Solution] = sample(population, 2)
    kid = Solution(instance)
    for i in range(1, instance.n + 1):
        for j in range(i + 1, instance.n + 1):
            if randint(0, 1) == 1:
                if parents[0].graph.get_edge_status(i, j):
                    kid.add_edge(i, j)
                else:
                    kid.remove_edge(i, j)
            else:
                if parents[1].graph.get_edge_status(i, j):
                    kid.add_edge(i, j)
                else:
                    kid.remove_edge(i, j)
    kid.construct()
    return kid

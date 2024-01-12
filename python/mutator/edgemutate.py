from abc import ABC

from random import random
from instance import Instance
from solution import Solution
from mutator.mutator import Mutator


class EdgeMutator(Mutator, ABC):

    def mutate(self, instance: Instance, population: list[Solution]) -> list[Solution]:
        population = population.copy()
        for pop in population:
            for i in range(1, instance.n + 1):
                for j in range(i + 1, instance.n + 1):
                    if random() < instance.parameters["mutation_chance"]:
                        pop.flip_edge(i, j)
            pop.construct()
        return population

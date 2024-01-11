import math
from abc import ABC

from random import shuffle, randint
from instance import Instance
from solution import Solution
from selector.selector import Selector


class TournamentSelector(Selector, ABC):

    def __init__(self):
        pass

    def select(self, instance: Instance, population: list[Solution], size: int) -> list[Solution]:
        selection_size = instance.parameters["selection_size"]
        num_selected = math.ceil(selection_size * size)

        out = []
        while len(out) < num_selected:
            contestants = population.copy()
            shuffle(contestants)
            while len(out) < num_selected:
                contestant_1 = population.pop()
                contestant_2 = population.pop()
                if contestant_1.fitness > contestant_2.fitness:
                    out.append(contestant_1)
                elif contestant_2.fitness > contestant_1.fitness:
                    out.append(contestant_2)
                else:
                    if randint(0, 1) == 1:
                        out.append(contestant_1)
                    else:
                        out.append(contestant_2)

        return sorted(out, key=lambda x: x.fitness, reverse=True)

import math
from abc import ABC

from instance import Instance
from solution import Solution
from selector.selector import Selector


class RankSelector(Selector, ABC):

    def __init__(self):
        pass

    def select(self, instance: Instance, population: list[Solution], size: int) -> list[Solution]:
        selection_size = instance.parameters["selection_size"]
        selected = math.ceil(selection_size * size)
        sortsel = sorted(population, key=lambda x:x.fitness, reverse=True)
        return sortsel[:selected]

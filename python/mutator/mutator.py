from abc import ABC, abstractmethod

from instance import Instance
from solution import Solution


class Mutator(ABC):

    @abstractmethod
    def mutate(self, instance: Instance, population: list[Solution]) -> list[Solution]:
        pass

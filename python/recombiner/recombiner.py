from abc import ABC, abstractmethod

from instance import Instance
from solution import Solution


class Recombiner(ABC):

    @abstractmethod
    def recombine(self, instance: Instance, population: list[Solution], size: int) -> list[Solution]:
        pass

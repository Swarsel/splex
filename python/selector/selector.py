from abc import ABC, abstractmethod

from instance import Instance
from solution import Solution


class Selector(ABC):

    @abstractmethod
    def select(self, instance: Instance, population: list[Solution], size: int) -> list[Solution]:
        pass

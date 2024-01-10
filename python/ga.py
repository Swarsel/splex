from solution import Solution
from instance import Instance
from selector.selector import Selector
from recombiner.recombiner import Recombiner
from mutator.mutator import Mutator


class GeneticAlgorithm:

    def __init__(self,
                 instance: Instance,
                 selector: Selector,
                 recombiner: Recombiner,
                 mutator: Mutator,
                 n_pop=30):
        self.n_pop = n_pop
        self.generation = 1
        self.instance = instance
        self.population = self.make_population()
        self.selector = selector
        self.recombiner = recombiner
        self.mutator = mutator

    def make_population(self):
        out = []
        for _ in range(self.n_pop):
            new = Solution(self.instance)
            new.construct()
            out.append(new)
        out.sort()
        return out

    def next_generation(self):
        selected = self.selector.select(self.instance, self.population, self.n_pop)
        # kids = self.recombiner.recombine(self.instance, selected, self.n_pop)
        kids = self.recombiner.recombine(self.instance, selected, self.n_pop)
        i = 1
        while len(kids) < self.n_pop - 1:
            kids.append(selected[i])
            i += 1
        population = self.mutator.mutate(self.instance, kids)
        population.append(selected[0])
        population.sort()
        self.population = population
        self.generation += 1

    def __str__(self):
        out = f"Population Size: {self.n_pop}\n"
        out += f"Generation: {self.generation}\n"
        out += f"Population Top5: {self.population[:5]}\nBottom5: {self.population[-5:]}"
        return out

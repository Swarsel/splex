from solution import Solution
from instance import Instance
from selector.selector import Selector
from recombiner.recombiner import Recombiner

class GeneticAlgorithm:

    def __init__(self, instance: Instance, selector: Selector, recombiner: Recombiner, n_pop=30):
        self.n_pop = n_pop
        self.generation = 1
        self.instance = instance
        self.population = self.make_population()
        self.selector = selector
        self.recombiner = recombiner

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
        print(kids)
        for kid in kids:
            print(kid.is_feasible_solution())
        i = 0
        while len(kids) < self.n_pop:
            kids.append(selected[i])
            i += 1
        kids.sort()
        self.population = kids
        self.generation += 1

    def __str__(self):
        out = f"Population Size: {self.n_pop}\n"
        out += f"Generation: {self.generation}\n"
        out += f"Population: {self.population}"
        return out

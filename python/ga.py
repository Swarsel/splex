from solution import Solution
from instance import Instance
from selector.selector import Selector
from recombiner.recombiner import Recombiner
from mutator.mutator import Mutator
import multiprocessing
from time import time


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
        # for _ in range(self.n_pop):
        with multiprocessing.Pool() as p:
            out = p.map(generate_solution, [self.instance for _ in range(self.n_pop)])
        out.sort()
        return out

    def next_n_generations(self, n):
        for _ in range(n):
            self.next_generation()

    def next_generation(self):
        selected = self.selector.select(self.instance, self.population, self.n_pop)
        # kids = self.recombiner.recombine(self.instance, selected, self.n_pop)
        # time1 = time()
        kids = self.recombiner.recombine(self.instance, selected, self.n_pop)
        # print(f"after recombine {time1 - time()}")
        i = 0
        while len(kids) < self.n_pop:
            kids.append(selected[i])
            i += 1

        # time1 = time()
        population: list[Solution] = self.mutator.mutate(self.instance, kids)
        # print(f"after mutation {time1 - time()}")
        # time1 = time()
        # with multiprocessing.Pool() as p:
            # population = p.map(construct_solution, population)
        # print(f"after repairing {time1 - time()}")
        population.sort()
        self.population = population
        # print(len(self.population))
        self.generation += 1

    def get_best_member(self):
        return self.population[0].cost

    def __str__(self):
        out = f"Population Size: {self.n_pop}\n"
        out += f"Generation: {self.generation}\n"
        out += f"Population Top5: {self.population[:5]}\nBottom5: {self.population[-5:]}"
        return out

def generate_solution(instance):
    solution = Solution(instance)
    solution.construct()
    return solution

def construct_solution(solution):
    solution.construct()
    return solution

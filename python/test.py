from input import read_input
from ga import GeneticAlgorithm
from selector.RankSelector import RankSelector
from recombiner.uniform import UniformCrossoverRecombiner
from mutator.edgemutate import EdgeMutator
import matplotlib.pyplot as plt
from solution import Solution

# instance = read_input("/home1/hot01427399/splex/instances/tuning_instances/heur040_n_300_m_13358.txt")
# instance2 = read_input("/home1/hot01427399/splex/instances/tuning_instances/heur040_n_300_m_13358.txt")
instance = read_input("../instances/tuning_instances/heur044_n_300_m_3234.txt")

GA = GeneticAlgorithm(instance,
                      RankSelector(),
                      UniformCrossoverRecombiner(),
                      EdgeMutator(),
                      n_pop=10)
print(GA)
GA.next_generation()
print(GA)
# GA.next_n_generations(10)
# print(GA.get_best_member())
# iterations = [it for it in range(5)]
# best_cost = []
# for _ in iterations:
#     GA.next_generation()
#     print(GA.get_best_member())
#     best_cost.append(GA.get_best_member())

# plt.grid()
# plt.title("heur002_n_100_m_3274.txt")
# plt.xlabel("Generation")
# plt.ylabel("Cost of best member")
# plt.plot(iterations, best_cost)
# plt.grid()
# plt.show()
# solution = Solution(instance)
# solution.construct()
# print(solution.graph.get_components())
# print(solution)

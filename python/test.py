from input import read_input
from ga import GeneticAlgorithm
from selector.RankSelector import RankSelector
from recombiner.uniform import UniformCrossoverRecombiner
from mutator.edgemutate import EdgeMutator
import matplotlib.pyplot as plt
from solution import Solution

instance = read_input("/home1/hot01427399/splex/instances/inst_competition/heur051_n_300_m_20122.txt")
# instance2 = read_input("/home1/hot01427399/splex/instances/inst_competition/heur051_n_300_m_20122.txt")
# instance = read_input("../instances/inst_competition/heur050_n_300_m_19207.txt")

GA = GeneticAlgorithm(instance,
                      RankSelector(),
                      UniformCrossoverRecombiner(),
                      EdgeMutator(),
                      n_pop=25)
print(GA)
print(GA.get_mean_member())
GA.next_generation()
print(GA)
print(GA.get_mean_member())
GA.next_generation()
print(GA)
print(GA.get_mean_member())
GA.next_generation()
print(GA)
print(GA.get_mean_member())
GA.next_generation()
print(GA)
print(GA.get_mean_member())
GA.next_generation()
print(GA)
print(GA.get_mean_member())
GA.next_generation()
print(GA)
print(GA.get_mean_member())
GA.next_generation()
print(GA)
print(GA.get_mean_member())
GA.next_generation()
print(GA)
print(GA.get_mean_member())
GA.next_generation()
print(GA)
print(GA.get_mean_member())
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

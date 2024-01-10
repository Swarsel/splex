from input import read_input
from ga import GeneticAlgorithm
from selector.RankSelector import RankSelector
from recombiner.uniform import UniformCrossoverRecombiner
from mutator.edgemutate import EdgeMutator

instance = read_input("../instances/test_instances/heur002_n_100_m_3274.txt")

GA = GeneticAlgorithm(instance,
                      RankSelector(),
                      UniformCrossoverRecombiner(),
                      EdgeMutator(),
                      n_pop=100)

print(GA)
for _ in range(10):
    GA.next_generation()
    print(GA)

# solution = Solution(instance)
# solution.construct()
# print(solution.graph.get_components())
# print(solution)

from input import read_input
from ga import GeneticAlgorithm
from selector.RankSelector import RankSelector
from recombiner.uniform import UniformCrossoverRecombiner
from mutator.edgemutate import EdgeMutator
import matplotlib.pyplot as plt

instance = read_input("/home1/hot01427399/splex/instances/tuning_instances/heur040_n_300_m_13358.txt")
instance2 = read_input("/home1/hot01427399/splex/instances/tuning_instances/heur040_n_300_m_13358.txt")
#instance = read_input("../instances/test_instances/heur002_n_100_m_3274.txt")

tunables = list(instance.parameters.keys())

iterations = 10
gens = 7
best = 4600
# for _ in range(iterations):
#     GA = GeneticAlgorithm(instance,
#                           RankSelector(),
#                           UniformCrossoverRecombiner(),
#                           EdgeMutator(),
#                           n_pop=instance.parameters["popsize"])
    # GA.next_n_generations(7)
    # best += GA.get_best_member()
# print(best / iterations)

key = tunables[1]
print(f"Checking {key}")
before = instance.get_parameter(key)
instance.set_parameter(key, before * 1.05)
best = 0
for _ in range(iterations):
    GA = GeneticAlgorithm(instance,
                          RankSelector(),
                          UniformCrossoverRecombiner(),
                          EdgeMutator(),
                          n_pop=instance.parameters["popsize"])
    GA.next_n_generations(7)
    best += GA.get_best_member()
print(f"{key} changed to {instance.get_parameter(key)}, yielded  {best / iterations}")
instance.set_parameter(key, before)

before = instance2.get_parameter(key)
instance2.set_parameter(key, before * 0.95)
best = 0
for _ in range(iterations):
    GA2 = GeneticAlgorithm(instance2,
                          RankSelector(),
                          UniformCrossoverRecombiner(),
                          EdgeMutator(),
                          n_pop=instance2.parameters["popsize"])
    GA2.next_n_generations(7)
    best += GA2.get_best_member()
print(f"{key} changed to {instance2.get_parameter(key)}, yielded  {best / iterations}")
instance.set_parameter(key, before)

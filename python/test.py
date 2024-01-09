from input import read_input
from graph import Graph
from solution import Solution

instance = read_input("../instances/test_instances/heur002_n_100_m_3274.txt")

solution = Solution(instance)

solution.construct()
print(solution.cost)

from input import read_input
from graph import Graph
from solution import Solution

instance = read_input("../instances/test_instances/heur001_n_10_m_31.txt")

solution = Solution(instance)

solution.construct()
print(solution.cost)

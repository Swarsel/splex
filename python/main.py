from input import read_input
from graph import Graph
from solution import Solution

instance = read_input("../instances/test_instances/heur001_n_10_m_31.txt")

graph = Graph(instance)
solution = Solution(instance)
components = list(graph.get_components())
for component in components:
    print(solution.is_feasible_component(component))

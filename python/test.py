from input import read_input
from graph import Graph
from solution import Solution

instance = read_input("../instances/test_instances/heur_test.txt")

solution = Solution(instance)
print(solution.cost)
components = list(solution.graph.get_components())
print(components)

print(solution.graph.get_edge_status(1,3), solution.instance.get_edge_status(1,3))
solution.add_edge(1, 3)
components = list(solution.graph.get_components())

print()
print(solution.graph.get_edge_status(1,3), solution.instance.get_edge_status(1,3))
solution.remove_edge(1, 3)
print(solution.graph.get_edge_status(1,3), solution.instance.get_edge_status(1,3))
components = list(solution.graph.get_components())

print(solution.cost)

from input import read_input
from graph import Graph

instance = read_input("../instances/test_instances/heur002_n_100_m_3274.txt")

graph = Graph(instance)
print(graph.get_node_component(1))
print()
print(graph.get_edges())

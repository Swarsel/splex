from graph import Graph
from instance import Instance
from random import shuffle

class Solution:

    def __init__(self, instance: Instance):
        self.instance = instance
        self.graph = Graph(instance)
        self.node_splex = np.zeros(instance.n)
        self.cost = self.compute_cost()

    def compute_cost(self):
        cost = 0
        for i in range(1, self.instance.n + 1):
            for j in range(1, self.instance.n + 1):
                if self.edge_differs_from_initial(i, j):
                    cost += abs(self.instance.get_edge_weight(i, j))
        return cost

    def edge_differs_from_initial(self, i, j):
        return self.instance.get_edge_status(i, j) != self.graph.get_edge_status(i, j)

    def is_feasible_solution(self):
        for node in range(1, self.instance.n + 1):
            if not self.node_splex(node):
                return False
        return True

    def is_feasible_node(self, i):
        return self.graph.get_node_degree(i) >= len(self.graph.get_node_component(i)) - self.instance.s

    def construct(self):
        components = list(self.graph.get_components())
        shuffle(components)


    def __str__(self):
        return f"Solution cost: {self.cost}"

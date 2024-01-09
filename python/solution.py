from graph import Graph
from instance import Instance
from random import shuffle
import numpy as np

class Solution:

    def __init__(self, instance: Instance):
        self.instance = instance
        self.graph = Graph(instance)
        self.node_splex = self.set_feasible_nodes()
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
        return self.node_splex[i - 1]

    def is_now_feasible_node(self, i):
        if self.graph.get_node_degree(i) >= len(self.graph.get_node_component(i)) - self.instance.s:
            self.node_splex[i - 1] = 1
            return True
        return False

    def get_component_required_degree(self, component):
        return len(component) - self.instance.s

    def is_feasible_component(self, component):
        for node in component:
            if self.node_splex[node - 1] != 1:
                return False
        return True

    def set_feasible_nodes(self):
        splex_tracker = np.zeros(self.instance.n, dtype=int)
        for node in range(1, self.instance.n + 1):
            if self.graph.get_node_degree(node) >= len(self.graph.get_node_component(node)) - self.instance.s:
                splex_tracker[node - 1] = 1
        return splex_tracker

    def construct(self):
        components = list(self.graph.get_components())
        shuffle(components)
        for component in components:
            if not self.is_feasible_component(component):
                self.repair_component(component, construct=True)

    def repair_component(self, component, construct=False, threshold=0.7):
        required_degree = self.get_component_required_degree(component)
        avg_degree = self.graph.get_component_avg_degree_from_component(component)
        if avg_degree > threshold * required_degree:
            self.add_edges(component, construct=construct)

    def add_edge(self, i, j):
        if not self.graph.get_edge_status(i, j):
            if self.edge_differs_from_initial(i, j):
                self.cost -= abs(self.instance.weights[i - 1][j - 1])
            else:
                self.cost += abs(self.instance.weights[i - 1][j - 1])
            self.graph.add_edge(i, j)
            self.update_splex_in_component(self.graph.node_component[i])

    def update_splex_in_component(self, component):
        # print(f"Updating splex for component {component}")
        for node in component:
            # print(f"Updating node {node}, ...", end="")
            if self.is_now_feasible_node(node):
                # print("feasible")
                self.node_splex[node - 1] = 1
            else:
                # print("not feasible")
                self.node_splex[node - 1] = 0

    def remove_edge(self, i, j):
        if self.graph.get_edge_status(i, j):
            if self.edge_differs_from_initial(i, j):
                self.cost -= abs(self.instance.weights[i - 1][j - 1])
            else:
                self.cost += abs(self.instance.weights[i - 1][j - 1])
            self.graph.remove_edge(i, j)
            self.update_splex_in_component(self.graph.node_component[i])

    def add_edges(self, component_set, construct=False):
        while not self.is_feasible_component(component_set):
            if construct:
                component = list(component_set)
                shuffle(component)
            for node in component:
                while not self.is_feasible_node(node):
                    # print(f"Checking node {node}, has degree {self.graph.get_node_degree(node)}, requred {self.get_component_required_degree(component)}")
                    pool_full = self.graph.get_missing_connections(self.get_possible_full_non_splex_node_connections_in_component(node))
                    pool = self.graph.get_missing_connections(self.get_possible_non_splex_node_connections_in_component(node))
                    # print(pool)
                    edge_member_1, edge_member_2 = pool[0][1], pool[0][2]
                    if pool_full and abs(pool_full[0][0]) <=pool[0][0]:
                        edge_member_1, edge_member_2 = pool_full[0][1], pool_full[0][2]
                    self.add_edge(edge_member_1, edge_member_2)

    def get_possible_full_non_splex_node_connections_in_component(self, i):
        return [edge for edge in self.graph.get_possible_node_connections_in_component(i) if (not self.node_splex[edge[1] - 1] and not self.node_splex[edge[2] - 1])]

    def get_possible_non_splex_node_connections_in_component(self, i):
        return [edge for edge in self.graph.get_possible_node_connections_in_component(i) if (not self.node_splex[edge[1] - 1] or not self.node_splex[edge[2] - 1])]

    def __str__(self):
        return f"Solution cost: {self.cost}"

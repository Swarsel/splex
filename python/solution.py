from graph import Graph
from instance import Instance
from random import shuffle, randint
import numpy as np
import functools


@functools.total_ordering
class Solution:

    def __init__(self, instance: Instance):
        self.instance = instance
        self.graph = Graph(instance)
        self.node_splex = self.set_feasible_nodes()
        self.cost = 0
        self.fitness = 0

    def compute_fitness(self):
        "higher number is better"
        return 1000/(self.cost + self.node_splex.count(0) * self.instance.parameters["penalty"])

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
            if self.node_splex[node - 1] != 1:
                return False
        return True

    def is_feasible_node(self, i):
        return self.node_splex[i - 1]

    def is_now_feasible_node(self, i):
        if self.graph.get_node_degree(i) >= len(self.graph.get_node_component(i)) - self.instance.s:
            # self.node_splex[i - 1] = 1
            return True
        return False

    def get_component_required_degree(self, component):
        check = len(component) - self.instance.s
        if check < 1:
            check = 1
        return check

    def is_feasible_component(self, component):
        for node in component:
            if self.node_splex[node - 1] != 1:
                return False
        return True

    def set_feasible_nodes(self):
        splex_tracker = []
        for node in range(1, self.instance.n + 1):
            if self.graph.get_node_degree(node) >= len(self.graph.get_node_component(node)) - self.instance.s:
                splex_tracker.append(1)
            else:
                splex_tracker.append(0)
        return splex_tracker

    def construct(self):
        self.fitness = self.compute_fitness()
        while not self.is_feasible_solution():
            components = list(self.graph.get_components())
            shuffle(components)
            # print(components)
            for component in components:
                # print(f"Checking {component}")
                # testlist = list(component)
                # print(self.graph.get_node_neighbors(testlist[0]))
                if not self.is_feasible_component(component):
                    work_component = component
                    break
            self.repair_component(work_component, construct=True)

    def repair_component(self, component, construct=False):
        required_degree = self.get_component_required_degree(component)
        avg_degree = self.graph.get_component_avg_degree_from_component(component)
        if avg_degree > self.instance.parameters["threshold"] * required_degree or randint(0, 1) == 1:
            # print(f"Adding edges to {component}")
            self.add_edges(component, construct=construct)
        else:
            # print(f"Removing edges to {component}")
            self.remove_edges(component, construct=construct)

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

    def flip_edge(self, i, j):
        if self.graph.get_edge_status(i, j):
            self.graph.remove_edge(i, j)
        else:
            self.graph.add_edge(i, j)

    def remove_edge(self, i, j):
        if self.graph.get_edge_status(i, j):
            if self.edge_differs_from_initial(i, j):
                self.cost -= abs(self.instance.weights[i - 1][j - 1])
            else:
                self.cost += abs(self.instance.weights[i - 1][j - 1])
            self.graph.remove_edge(i, j)
            self.update_splex_in_component(self.graph.node_component[i])
            self.update_splex_in_component(self.graph.node_component[j])

    def remove_edges(self, component_set, construct=False):
        if not self.is_feasible_component(component_set):
            edges = self.graph.get_existing_connections(self.graph.get_possible_node_connections_in_component(self.graph.get_component_min_degree_node_from_component(component_set)))
            for edge in edges:
                self.remove_edge(edge[1], edge[2])

    def add_edges(self, component_set, construct=False):
        '''
        Process a component that needs to have edges added. construct flag enables randomization of processing order to generate different solutions.
        Check pool of edge pairs that are both not in splex status first, if those are cheaper than splex-non-splex pairs, create those edges, otherwise create conservative edges
        '''
        if not self.is_feasible_component(component_set):
            changes = False
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
                    changes = True
                if changes:
                    break

    def get_possible_full_non_splex_node_connections_in_component(self, i):
        "get list of edges in component of i of nodes which are both noth splex"
        return [edge for edge in self.graph.get_possible_node_connections_in_component(i) if (not self.node_splex[edge[1] - 1] and not self.node_splex[edge[2] - 1])]

    def get_possible_non_splex_node_connections_in_component(self, i):
        "get list of edges in component of i of which at least one is not splex"
        return [edge for edge in self.graph.get_possible_node_connections_in_component(i) if (not self.node_splex[edge[1] - 1] or not self.node_splex[edge[2] - 1])]

    def __eq__(self, other):
        return self.cost == other.cost

    def __lt__(self, other):
        return self.cost < other.cost

    def __repr__(self):
        return f"{self.cost} ({self.fitness})"

    def __str__(self):
        return f"Solution cost: {self.cost}"

import numpy as np
from instance import Instance


class Graph:

    def __init__(self, instance: Instance):
        self.n = instance.n
        self.edges = instance.edges
        self.node_degree = np.zeros(instance.n)
        self.connected = instance.connected
        self.node_neighbors = {node: self.compute_node_neighbors(node) for node in range(1, instance.n+1)}
        self.node_component = {node: self.compute_components(node) for node in range(1, instance.n+1)}

    def get_edges(self):
        return self.edges

    def get_edge_status(self, i, j):
        return self.connected[i - 1, j - 1]

    def get_components(self):
        components = []
        for component in self.node_component.values():
            if component not in components:
                components.append(component)
        return components

    def get_node_component(self, i):
        return self.node_component[i]

    def get_component_avg_degree(self, i):
        component = self.get_node_component(i)
        degree_sum = self.get_node_degree(i)
        for node in component:
            degree_sum += self.get_node_degree(node)
        return degree_sum / component.len()

    def get_component_min_degree(self, i):
        component = self.get_node_component(i)
        degree_min = self.get_node_degree(i)
        for node in component:
            if self.get_node_degree(node) < degree_min:
                degree_min = self.get_node_degree(node)
        return degree_min

    def get_node_neighbors(self, i):
        return self.node_neighbors[i]

    def get_node_degree(self, i):
        return self.node_degree[i]

    def add_edge(self, i, j):
        if not self.connected[i - 1, j - 1]:
            self.node_degree[i - 1] += 1
            self.node_degree[j - 1] += 1
            self.connected[i - 1, j - 1] = True
            self.connected[j - 1, i - 1] = True
            self.node_neighbors[i].add(j)
            self.node_neighbors[j].add(i)
            self.join_components(i, j)

    def remove_edge(self, i, j):
        if self.connected[i - 1, j - 1]:
            self.node_degree[i - 1] -= 1
            self.node_degree[j - 1] -= 1
            self.connected[i - 1, j - 1] = False
            self.connected[j - 1, i - 1] = False
            self.node_neighbors[i].remove(j)
            self.node_neighbors[j].remove(i)
            self.unjoin_components(i, j)

    def set_node_components(self, component):
        for node in component:
            self.node_component[node] = component

    def compute_node_neighbors(self, i):
        neighbors = set()
        for j in range(1, self.n + 1):
            if self.get_edge_status(i, j):
                neighbors.add(j)
        return neighbors

    def join_components(self, i, j):
        component_i = self.get_node_component(i)
        component_j = self.get_node_component(j)
        if not component_i == component_j:
            merged_component = component_i.union(component_j)
            self.set_node_components(merged_component)

    def compute_components(self, i):
        component = {i}
        check = list(self.get_node_neighbors(i))
        added = [False for _ in range(self.n)]
        added[i - 1] = True
        while check:
            node = check.pop()
            if not added[node - 1]:
                component.add(node)
                check.extend(self.get_node_neighbors(node))
                added[node - 1] = True
        return component

    def unjoin_component(self, i, j):
        component_i = self.compute_component(i)
        component_j = self.compute_component(j)
        if not component_i == component_j:
            self.set_node_components(component_i)
            self.set_node_components(component_j)

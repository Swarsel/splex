mod vnd;
use vnd as hybrid_vnd;

use std::{
    collections::{BTreeSet, HashMap},
    vec, time::Duration,
};

use itertools::Itertools;

use crate::neighborhood::{
    nflip::NFlip,
    oneflip::OneFlip,
    stepfunction::StepFunction::{self, *},
};
use crate::solution::Solution;
use crate::{
    construction::ConstructionHeuristic,
    neighborhood::{self, neighborhood::Neighborhood},
};
use crate::{graph::Graph, grasp::GRASP, symmat::SymMat, vnd::VND};

/// Hybrid 2-stage approach:
/// 1. Group vertices
/// 2. Find optimal s-plex for each group
pub struct Hybrid {
    graph: Graph,
    group_eval: GroupEval,
    one_flip_step_function: StepFunction,
    two_flip_step_function: StepFunction,
    phase1_time: Duration,
}

impl Hybrid {
    pub fn new(
        graph: Graph,
        one_flip_step_function: StepFunction,
        two_flip_step_function: StepFunction,
        phase1_time: Duration,
    ) -> Self {

        Self {
            graph,
            group_eval: GroupEval {
                cache: HashMap::new(),
            },
            one_flip_step_function,
            two_flip_step_function,
            phase1_time,
        }
    }

    /// split graph into groups to be turned into splexes
    /// returns cost of separating groups and groups
    pub fn phase1(&mut self) -> Vec<Group> {
        let hvnd = hybrid_vnd::VND::new(vec![
            Box::new(hybrid_vnd::MoveVertex),
            Box::new(hybrid_vnd::SplitGroup),
            Box::new(hybrid_vnd::MergeGroups),
        ]);

        // use connected components as starting groups
        let groups: Vec<Group> = Graph::get_connection_components(&self.graph.initial)
            .into_iter()
            .map(|component| Group {
                vertices: component.indices.into_iter().collect(),
            })
            .collect();

        let graph = self.graph.clone();
        
        // initial solution
        let solution = hybrid_vnd::Solution {
            graph: &graph,
            groups: groups
                .iter()
                .map(|group| {
                    let subgraph = self
                        .graph
                        .subgraph(&group.vertices.iter().map(|v| *v).collect::<Vec<_>>());

                    let cost = self.group_eval.construct(&subgraph).cost;

                    (group.clone(), cost)
                })
                .collect(),
        };

        let start = std::time::Instant::now();

        let mut best_solution = hvnd.run(solution);
        let mut best_cost = self.phase2(best_solution.groups.clone().iter().map(|(g, _)| g.clone()).collect()).cost;

        while start.elapsed() < self.phase1_time {
            let solution = hvnd.run(best_solution.clone());
            
            let cost = self.phase2(solution.groups.clone().iter().map(|(g, _)| g.clone()).collect()).cost;

            if cost < best_cost {
                best_solution = solution;
                best_cost = cost;
            }
        }

        let groups = best_solution
            .groups
            .drain(..)
            .map(|(g, _)| g)
            .collect::<Vec<_>>();

        // println!("groups: {:?}", groups);

        // let mut cost = 0;

        // for group in groups.iter() {
        //     for vertex in group.vertices.iter() {
        //         for adjacent in Graph::adjacent(&self.solution, *vertex) {
        //             if !group.vertices.contains(&adjacent) {
        //                 // edge needs to be removed
        //                 cost += self.graph.weights.get(*vertex, adjacent);
        //                 self.solution.set(*vertex, adjacent, false);
        //             }
        //         }
        //     }
        // }

        groups
    }

    /// Determine cost to turn all groups into splexes
    pub fn phase2(&mut self, groups: Vec<Group>) -> Solution {
        let mut solution: Solution = Solution::new(&self.graph);
        solution.recalculate_degrees();
        solution.recalculate_connection_components();

        // remove all edges between groups
        for g in groups.iter() {
            for vertex in g.vertices.iter() {
                for adjacent in Graph::adjacent(&solution.edges, *vertex) {
                    if !g.vertices.contains(&adjacent) {
                        // edge needs to be removed
                        solution.remove_edge(*vertex, adjacent);
                    }
                }
            }
        }

        for group in groups {
            if group.vertices.len() as u32 <= self.graph.s {
                // group is already an splex
                continue;
            }

            // turn group into subgraph
            let g = self
                .graph
                .subgraph(&group.vertices.iter().map(|v| *v).collect::<Vec<_>>());

            let const_heur = Box::new(GroupEval {
                cache: HashMap::new(),
            }) as Box<dyn ConstructionHeuristic>;

            let mut neighborhoods: Vec<(Box<dyn Neighborhood>, StepFunction)> = vec![];

            if self.one_flip_step_function != Skip {
                neighborhoods.push((Box::new(OneFlip), self.one_flip_step_function));
            }

            if self.two_flip_step_function != Skip {
                neighborhoods.push((Box::new(NFlip::<2>), self.two_flip_step_function));
            }

            let vnd = VND::new(neighborhoods);

            let grasp = GRASP::new(const_heur, vnd);

            let mut sol = grasp.run(&g);
            
            sol.recalculate_connection_components();
            sol.recalculate_degrees();
            
            // incorporate solution into self.solution
            for vertices in group.vertices.iter().enumerate().combinations(2) {
                let from = vertices[0].1;
                let to = vertices[1].1;

                if *sol.edges.get(vertices[0].0, vertices[1].0) {
                    solution.add_edge(*from, *to);
                } else {
                    solution.remove_edge(*from, *to);
                }
            }

            // println!("group: {:?}", group);
            // println!("solution: {:?}", sol);

            assert!(sol.is_valid());
        }

        solution.recalculate_connection_components();
        solution.recalculate_degrees();

        solution
    }

    pub fn run(&mut self) -> Solution {
        let groups = self.phase1();

        let g = self.graph.clone();

        let mut solution = self.phase2(groups);

        if !solution.is_valid() {
            // println!("invalid solution");
    
            // println!("{:?}", solution);
            

            // find invalid groups
            let mut invalid_groups = vec![];

            for (i, group) in solution.connection_components.iter().enumerate() {
                if !solution.is_connection_component_splex(group, solution.graph.s as u32) {
                    // println!("group {} is not a {}-plex", i, solution.graph.s);
                    // println!("group: {:?}", group);
    
                    // for vertex in group.indices.iter() {
                    //     println!("vertex {} degree: {}", vertex, solution.vertices[*vertex].degree);
                    // }

                    // run groupeval on group
                //     let subgraph = g
                //         .subgraph(&group.indices.iter().map(|v| *v).collect::<Vec<_>>());

                //     let sol = GroupEval::new().construct(&subgraph);

                //     // incorporate solution into self.solution
                    
                    invalid_groups.push(i);

                }
            }

            for group in invalid_groups {
                let group = &solution.connection_components[group].clone();

                let subgraph = g
                    .subgraph(&group.indices.iter().map(|v| *v).collect::<Vec<_>>());

                let sol = GroupEval::new().construct(&subgraph);

                // incorporate solution into self.solution
                for vertices in group.indices.iter().enumerate().combinations(2) {
                    let from = vertices[0].1;
                    let to = vertices[1].1;

                    if *sol.edges.get(vertices[0].0, vertices[1].0) {
                        solution.add_edge(*from, *to);
                    } else {
                        solution.remove_edge(*from, *to);
                    }
                }
            }
        }

        solution
    }
}

/// Holds information about a group of vertices
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Group {
    vertices: BTreeSet<usize>,
}

impl Group {
    fn empty() -> Self {
        Self {
            vertices: BTreeSet::new(),
        }
    }
}

pub struct GroupEval {
    cache: HashMap<Group, u32>,
}

impl GroupEval {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
}

impl<'a> ConstructionHeuristic<'a> for GroupEval {
    fn construct(&self, graph: &'a Graph) -> Solution<'a> {
        let mut solution = Solution::new(graph);

        solution.recalculate_degrees();

        if solution.vertices.len() as u32 > graph.s {
            // determine unsatisfied vertice
            let required = solution.vertices.len() as u32 - graph.s;
            // println!("required: {}", required);
            let mut edges = vec![];

            for (id, _) in solution
                .vertices
                .iter()
                .enumerate()
                .filter(|(_, v)| v.degree < required)
            {
                for v in 0..graph.initial.len() {
                    if id == v {
                        continue;
                    }

                    if !*solution.edges.get(id, v) {
                        edges.push((id, v));
                    }
                }
            }

            edges.sort_unstable_by_key(|(a, b)| {
                // get weight of edge
                let weight = *graph.weights.get(*a, *b);

                // if both vertices are unsatisfied, halve the weight
                if solution.vertices[*a].degree < required
                    && solution.vertices[*b].degree < required
                {
                    weight / 2
                } else {
                    weight
                }
            });
            edges.dedup_by_key(|(vertex, adjacent)| {
                if vertex < adjacent {
                    (*vertex, *adjacent)
                } else {
                    (*adjacent, *vertex)
                }
            });

            // println!("edges: {:?}", edges);

            // add edges when needed, starting with the cheapest
            for (vertex, adjacent) in edges {
                if solution.vertices[vertex].degree >= required
                    && solution.vertices[adjacent].degree >= required
                {
                    // println!("skipping: {} {} [{} {}]", vertex, adjacent, solution.vertices[vertex].degree, solution.vertices[adjacent].degree);
                    continue;
                }

                if *solution.edges.get(vertex, adjacent) {
                    // println!("skipping: {} {} [already exists]", vertex, adjacent);
                    continue;
                }
                
                solution.vertices[vertex].degree += 1;
                solution.vertices[adjacent].degree += 1;
                solution.edges.set(vertex, adjacent, true);
                solution.cost += graph.weights.get(vertex, adjacent);
            }
        } else {
            // println!("skipping: {} <= {}", solution.vertices.len(), graph.s);
        }

        solution.recalculate_connection_components();
        solution.recalculate_degrees();

        if !solution.is_valid() {
            // find unsatisfied vertices and print their id

            let required = solution.vertices.len() as u32 - graph.s;

            for (id, v) in solution.vertices.iter().enumerate() {
                if v.degree < required {
                    println!("unsatisfied: {} [{} < {}]", id, v.degree, required);
                }
            }

            // println!("s: {}", graph.s);
            // println!("solution: {:?}", solution);
        }

        solution
    }
}

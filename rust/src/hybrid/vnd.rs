use super::Group;
use super::GroupEval;
use crate::{construction::ConstructionHeuristic, graph::Graph};
use rand::prelude::*;

#[derive(Clone)]
pub struct Solution<'a> {
    pub graph: &'a Graph,
    pub groups: Vec<(Group, u32)>,
}

pub trait Neighborhood {
    fn get_solution<'a>(&self, solution: &mut Solution<'a>, eval: &mut GroupEval) -> bool;
}

pub struct VND {
    neighborhoods: Vec<Box<dyn Neighborhood>>,
}

impl VND {
    pub fn new(neighborhoods: Vec<Box<dyn Neighborhood>>) -> Self {
        Self { neighborhoods }
    }

    pub fn run<'a>(&self, mut solution: Solution<'a>) -> Solution<'a> {
        let mut eval = GroupEval::new();

        let mut k = 0;

        while k < self.neighborhoods.len() {
            let neighborhood = &self.neighborhoods[k];

            if neighborhood.get_solution(&mut solution, &mut eval) {
                k = 0;
            } else {
                k += 1;
            }
        }

        solution
    }
}

pub struct MoveVertex;

impl Neighborhood for MoveVertex {
    fn get_solution<'a>(&self, solution: &mut Solution<'a>, eval: &mut GroupEval) -> bool {
        let mut rng = thread_rng();

        // pick random vertex
        let vertex = rng.gen_range(0..solution.graph.initial.len());

        // pick random group
        let dest_index = rng.gen_range(0..solution.groups.len());

        // find group of vertex
        if let Some(source_index) = solution
            .groups
            .iter_mut()
            .enumerate()
            .find(|(_, (group, _))| group.vertices.contains(&vertex))
            .map(|(index, _)| index)
        {
            // remove vertex from group
            solution.groups[source_index].0.vertices.remove(&vertex);
            // pick random group

            // add vertex to group
            solution.groups[dest_index].0.vertices.insert(vertex);

            // evaluate solution
            // 2 groups changed, so we only need to evaluate those
            let (source, source_cost) = &solution.groups[source_index];
            let (dest, dest_cost) = &solution.groups[dest_index];

            let subgraph = solution
                .graph
                .subgraph(&source.vertices.iter().map(|v| *v).collect::<Vec<_>>());

            let new_cost_source = eval.construct(&subgraph).cost;

            let subgraph = solution
                .graph
                .subgraph(&dest.vertices.iter().map(|v| *v).collect::<Vec<_>>());

            let new_cost_dest = eval.construct(&subgraph).cost;

            if new_cost_source + new_cost_dest < source_cost + *dest_cost {
                solution.groups[source_index].1 = new_cost_source;
                solution.groups[dest_index].1 = new_cost_dest;

                // if source is now empty, remove it
                if solution.groups[source_index].0.vertices.len() == 0 {
                    solution.groups.remove(source_index);
                }

                true
            } else {
                // undo changes
                solution.groups[source_index].0.vertices.insert(vertex);
                solution.groups[dest_index].0.vertices.remove(&vertex);

                false
            }
        } else {
            return false;
        }
    }
}

pub struct SplitGroup;

impl Neighborhood for SplitGroup {
    fn get_solution<'a>(&self, solution: &mut Solution<'a>, eval: &mut GroupEval) -> bool {
        let mut rng = thread_rng();

        // pick random group
        let group_index = rng.gen_range(0..solution.groups.len());

        let old_cost = solution.groups[group_index].1;

        // split group
        let mut new_group_a = Group::empty();
        let mut new_group_b = Group::empty();

        for vertex in solution.groups[group_index].0.vertices.iter() {
            if rng.gen_bool(0.5) {
                new_group_a.vertices.insert(*vertex);
            } else {
                new_group_b.vertices.insert(*vertex);
            }
        }

        // evaluate solution
        let subgraph = solution
            .graph
            .subgraph(&new_group_a.vertices.iter().map(|v| *v).collect::<Vec<_>>());

        let new_cost_a = eval.construct(&subgraph).cost;

        let subgraph = solution
            .graph
            .subgraph(&new_group_b.vertices.iter().map(|v| *v).collect::<Vec<_>>());

        let new_cost_b = eval.construct(&subgraph).cost;

        if new_cost_a + new_cost_b < old_cost {
            solution.groups[group_index].0 = new_group_a;
            solution.groups[group_index].1 = new_cost_a;

            solution.groups.push((new_group_b, new_cost_b));

            true
        } else {
            false
        }
    }
}

pub struct MergeGroups;

impl Neighborhood for MergeGroups {
    fn get_solution<'a>(&self, solution: &mut Solution<'a>, eval: &mut GroupEval) -> bool {
        if solution.groups.len() <= 1 {
            return false;
        }

        let mut rng = thread_rng();

        // pick random group
        let group_index_a = rng.gen_range(0..solution.groups.len());
        let group_index_b = {
            let mut index = rng.gen_range(0..solution.groups.len());

            while index == group_index_a {
                index = rng.gen_range(0..solution.groups.len());
            }

            index
        };

        let old_cost_a = solution.groups[group_index_a].1;
        let old_cost_b = solution.groups[group_index_b].1;

        // merge groups
        let mut new_group = Group::empty();

        for vertex in solution.groups[group_index_a].0.vertices.iter() {
            new_group.vertices.insert(*vertex);
        }

        for vertex in solution.groups[group_index_b].0.vertices.iter() {
            new_group.vertices.insert(*vertex);
        }

        // evaluate solution
        let subgraph = solution
            .graph
            .subgraph(&new_group.vertices.iter().map(|v| *v).collect::<Vec<_>>());

        let new_cost = eval.construct(&subgraph).cost;

        if new_cost < old_cost_a + old_cost_b {
            solution.groups[group_index_a].0 = new_group;
            solution.groups[group_index_a].1 = new_cost;

            solution.groups.remove(group_index_b);

            true
        } else {
            false
        }
    }
}

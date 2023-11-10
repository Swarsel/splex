use crate::graph::Graph;
use nom::{self, IResult, character::complete::{self, multispace1}, character::complete::line_ending, sequence::terminated, multi::separated_list1};


fn parse_metadata(input: &str) -> IResult<&str, GraphMetadata> {
    let (input, s) = terminated(complete::u32, multispace1)(input)?;
    let (input, n) = terminated(complete::u32, multispace1)(input)?;
    let (input, m) = terminated(complete::u32, multispace1)(input)?;
    let (input, _) = terminated(complete::u32, line_ending)(input)?;

    Ok((input, GraphMetadata {s, num_vertices: n, _num_edges: m}))
}

fn parse_edge(input: &str) -> IResult<&str, EdgeData> {
    let (input, start) = terminated(complete::u32, multispace1)(input)?;
    let (input, end) = terminated(complete::u32, multispace1)(input)?;
    let (input, present) = terminated(complete::u32, multispace1)(input)?;
    let (input, weight) = complete::u32(input)?;

    Ok((input, EdgeData {start: start as usize - 1, end: end as usize - 1, present: present == 1, weight}))
}

pub fn parse(input: &str) -> Option<Graph> {
    let (input, metadata) = parse_metadata(input).ok()?;
    let (_, edges) = separated_list1(line_ending, parse_edge)(input).ok()?;


    let mut adjacency = (0..(metadata.num_vertices)).into_iter().map(|_|
        (0..(metadata.num_vertices)).into_iter().map(|_| false).collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    let mut weights = (0..(metadata.num_vertices)).into_iter().map(|_|
        (0..(metadata.num_vertices)).into_iter().map(|_| 0u32).collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    for edge in edges {
        adjacency[edge.start][edge.end] = edge.present;
        adjacency[edge.end][edge.start] = edge.present;
        weights[edge.start][edge.end] = edge.weight;
        weights[edge.end][edge.start] = edge.weight;
    }

    let initial = adjacency.clone();

    Some(Graph {s: metadata.s, adjacency, initial, weights})
}

struct GraphMetadata {
    s: u32,
    num_vertices: u32,
    _num_edges: u32,
}

struct EdgeData {
    start: usize,
    end: usize,
    present: bool,
    weight: u32,
}
use crate::{graph::Graph, symmat::SymMat};
use nom::{
    self,
    character::complete::line_ending,
    character::complete::{self, multispace1},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

fn parse_metadata(input: &str) -> IResult<&str, GraphMetadata> {
    let (input, s) = terminated(complete::u32, multispace1)(input)?;
    let (input, n) = terminated(complete::u32, multispace1)(input)?;
    let (input, m) = terminated(complete::u32, multispace1)(input)?;
    let (input, _) = terminated(complete::u32, line_ending)(input)?;

    Ok((
        input,
        GraphMetadata {
            s,
            num_vertices: n,
            _num_edges: m,
        },
    ))
}

fn parse_edge(input: &str) -> IResult<&str, EdgeData> {
    let (input, start) = terminated(complete::u32, multispace1)(input)?;
    let (input, end) = terminated(complete::u32, multispace1)(input)?;
    let (input, present) = terminated(complete::u32, multispace1)(input)?;
    let (input, weight) = complete::u32(input)?;

    Ok((
        input,
        EdgeData {
            start: start as usize - 1,
            end: end as usize - 1,
            present: present == 1,
            weight,
        },
    ))
}

pub fn parse(input: &str) -> Option<Graph> {
    let (input, metadata) = parse_metadata(input).ok()?;
    let (_, edges) = separated_list1(line_ending, parse_edge)(input).ok()?;

    let mut initial = SymMat::new(metadata.num_vertices as usize);
    let mut weights = SymMat::new(metadata.num_vertices as usize);

    for edge in edges {
        initial.set(edge.start, edge.end, edge.present);
        weights.set(edge.start, edge.end, edge.weight);
    }

    Some(Graph {
        s: metadata.s,
        initial,
        weights,
    })
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

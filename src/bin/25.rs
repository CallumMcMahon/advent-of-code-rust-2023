advent_of_code::solution!(25);

use petgraph::{graph::UnGraph, graphmap::UnGraphMap, Graph};

mod input_parser {
    use super::Line;
    use nom::{
        bytes::complete::{tag, take},
        character::complete::multispace0,
        character::{
            self,
            complete::{alpha1, space1},
        },
        error::Error as NomError,
        multi::{many1, separated_list1},
        Finish, IResult,
    };

    fn parse_hand(input: &str) -> IResult<&str, Line> {
        let (input, component) = alpha1(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, connected) = separated_list1(space1, alpha1)(input)?;
        let (input, _) = multispace0(input)?;
        Ok((
            input,
            Line {
                component: component,
                connected: connected,
            },
        ))
    }

    pub fn parse_input(input: &str) -> Result<Vec<Line>, NomError<&str>> {
        let (_input, lines) = many1(parse_hand)(input).finish()?;
        Ok(lines)
    }
}

pub struct Line<'a> {
    component: &'a str,
    connected: Vec<&'a str>,
}
use itertools::Itertools;
use petgraph::algo::*;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input_parser::parse_input(input).unwrap();
    let edges: Vec<(&str, &str)> = lines
        .iter()
        .flat_map(|line| {
            line.connected
                .iter()
                .map(move |&connected| (line.component, connected))
        })
        .collect();

    let mut gr = UnGraphMap::<_, ()>::from_edges(&edges);
    for removed_edges in edges.iter().combinations(3) {
        for (a, b) in removed_edges.iter() {
            gr.remove_edge(a, b);
        }

        if connected_components(&gr) == 2 {
            let components = tarjan_scc(&gr);
            print!("{:?}", removed_edges);
            return Some(
                components
                    .iter()
                    .map(|component| component.len() as u32)
                    .product(),
            );
        }

        for (a, b) in removed_edges.iter() {
            gr.add_edge(a, b, ());
        }
    }
    panic!("Edge combination not found!")
    // let components_1 = connected_components(&gr);
    // gr.remove_edge("pzl", "hfx");
    // gr.remove_edge("bvb", "cmg");
    // gr.remove_edge("nvd", "jqt");
    // // let pg = deps.add_node("petgraph");
    // // let pg2 = deps.add_node("petgraph");
    // let components_2 = connected_components(&gr);
    // 1 + 1;
    // todo!();
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

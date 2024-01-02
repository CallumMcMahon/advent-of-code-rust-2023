use std::collections::HashMap;
use num::integer::lcm;

use itertools::Itertools;

advent_of_code::solution!(8);

mod input_parser {
    use std::collections::HashMap;

    use crate::Instruction;

    use super::{Document, Node};
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, alphanumeric1, multispace0, multispace1},
        error::Error as NomError,
        multi::many1,
        Finish, IResult,
    };

    fn parse_node(input: &str) -> IResult<&str, (&str, Node)> {
        let (input, name) = alphanumeric1(input)?;
        let (input, _) = tag(" = (")(input)?;
        let (input, left) = alphanumeric1(input)?;
        let (input, _) = tag(", ")(input)?;
        let (input, right) = alphanumeric1(input)?;
        let (input, _) = tag(")")(input)?;
        let (input, _) = multispace0(input)?;
        let node = Node {
            left: left.to_string(),
            right: right.to_string(),
        };
        return Ok((input, (name, node)));
    }

    fn parse_text(input: &str) -> IResult<&str, Document> {
        let (input, instructions) = alpha1(input)?;
        let (input, _) = multispace1(input)?;
        let (input, nodes) = many1(parse_node)(input)?;
        let mut node_arena = HashMap::new();
        // nodes.into_iter().map(|(name, node)| node_arena.insert(name.to_string(), node));
        for (name, node) in nodes {
            node_arena.insert(name.to_string(), node);
        }
        let instructions = instructions
            .chars()
            .map(|c| Instruction::from(&c).unwrap())
            .collect();
        return Ok((
            input,
            Document {
                instructions,
                nodes: node_arena,
            },
        ));
    }

    pub fn parse_input(input: &str) -> Result<Document, NomError<&str>> {
        let (_input, document) = parse_text(input).finish()?;
        Ok(document)
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Left,
    Right,
}
impl Instruction {
    fn from(c: &char) -> Result<Self, &str> {
        match c {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err("Invalid Instruction"),
        }
    }
}

struct Node {
    left: String,
    right: String,
}
impl Node {
    fn get(&self, side: Instruction) -> &String {
        match side {
            Instruction::Left => &self.left,
            Instruction::Right => &self.right,
        }
    }
}

pub struct Document {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, Node>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let document = input_parser::parse_input(input).unwrap();
    let mut step_count = 0;
    let mut current_node = document.nodes.get("AAA").unwrap();
    loop {
        let instruction_idx = step_count % document.instructions.len();
        if instruction_idx == 0 {
            println!("{:?}", step_count)
        }
        step_count += 1;
        let instruction = document.instructions[instruction_idx];
        let next_node_name = current_node.get(instruction);
        if next_node_name == "ZZZ" {
            break;
        }
        current_node = document.nodes.get(next_node_name).unwrap();
    }
    return Some(step_count as u32);
}

// (11A, L) => 2n + 2
// (11B, R) => 2n + 1 for all n
// A1 -> A2 -> A3 -> B -> C -> Z -> B -> C -> Z
// cycle of 3n,
// A1 = 3n + (5,)
// A2 = 3n + (4,)
// A3 = 3n + (3,)
// B = 3n + (2,)
// C = 3n + (1,)
// Z = 3n + (0,)
// gather all starting values XXA

pub fn part_two_old(input: &str) -> Option<u32> {
    let document = input_parser::parse_input(input).unwrap();
    let mut starting_nodes: Vec<&String> = document
        .nodes
        .keys()
        .filter(|node| node.ends_with("A"))
        .collect();
    let mut ending_nodes: Vec<&String> = document
        .nodes
        .keys()
        .filter(|node| node.ends_with("Z"))
        .collect();
    println!("{:?}", starting_nodes);
    println!("{:?}", ending_nodes);

    let mut step_count = 0;
    loop {
        let instruction_idx = step_count % document.instructions.len();
        if step_count % 1000000 == 0 {
            println!("{:?}", step_count)
        }
        step_count += 1;
        let instruction = document.instructions[instruction_idx];
        starting_nodes = starting_nodes
            .iter()
            .map(|&node| document.nodes.get(node).unwrap().get(instruction))
            .collect();

        if starting_nodes.iter().all(|&x| x.ends_with("Z")) {
            break;
        }
    }
    return Some(step_count as u32);
}

#[derive(Debug)]
struct Loop {
    start: String,
    z_offsets: Vec<u32>,
    loop_length: u32,
}

pub fn part_two(input: &str) -> Option<u64> {
    let document = input_parser::parse_input(input).unwrap();
    let mut starting_nodes: Vec<&String> = document
        .nodes
        .keys()
        .filter(|node| node.ends_with("A"))
        .collect();
    println!("{:?}", starting_nodes);

    let mut loops = Vec::new();

    for node in starting_nodes {
        let mut seen = vec![(node, 0)];
        let mut step_count = 0;
        let mut current_node = document.nodes.get(node).unwrap();
        let mut z_offsets = Vec::new();
        loop {
            let instruction_idx = step_count % document.instructions.len();
            // if instruction_idx == 0 { println!("{:?}", step_count)}
            step_count += 1;
            let instruction = document.instructions[instruction_idx];
            let next_node_name = current_node.get(instruction);
            if next_node_name.ends_with("Z") {
                z_offsets.push(step_count as u32);
            }
            // else if seen.contains(&next_node_name) {
            //     let loop_length = seen.len() - seen.iter().position(|&x| x == next_node_name).unwrap();
            //     loops.push(Loop {start: node.clone(), z_offsets, loop_length: loop_length as u32});
            //     break
            // } else {
            //     seen.push(next_node_name)
            // }
            if let Some(loop_start_pos) = seen
                .iter()
                .position(|&x| x == (next_node_name, instruction_idx))
            {
                let loop_length = seen.len() - loop_start_pos;
                loops.push(Loop {
                    start: node.clone(),
                    z_offsets,
                    loop_length: loop_length as u32,
                });
                break;
            } else {
                seen.push((next_node_name, instruction_idx))
            }
            current_node = document.nodes.get(next_node_name).unwrap();
        }
    }
    let loop_lengths: Vec<u64> = loops.iter().map(|x| x.loop_length as u64).collect();
    println!("{:?}", loop_lengths);
    let result_2 = loop_lengths.into_iter().fold(1_u64, |acc, x| lcm(acc, x));
    // let result = loops.into_iter().fold(1, |acc, x| lcm(acc, x.loop_length));
    return Some(result_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}

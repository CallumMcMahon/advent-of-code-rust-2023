use std::collections::HashMap;

advent_of_code::solution!(8);

mod input_parser {
    use std::collections::HashMap;

    use crate::Instruction;

    use super::{Document, Node};
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, multispace0, multispace1},
        error::Error as NomError,
        multi::many1,
        Finish, IResult,
    };

    fn parse_node(input: &str) -> IResult<&str, (&str, Node)> {
        let (input, name) = alpha1(input)?;
        let (input, _) = tag(" = (")(input)?;
        let (input, left) = alpha1(input)?;
        let (input, _) = tag(", ")(input)?;
        let (input, right) = alpha1(input)?;
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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

advent_of_code::solution!(4);

use std::collections::HashSet;

#[derive(Debug)]
pub struct Card {
    winning: HashSet<u32>,
    held: HashSet<u32>,
}

impl Card {
    pub fn new(winning: HashSet<u32>, held: HashSet<u32>) -> Self {
        Card { winning, held }
    }

    pub fn copies(&self) -> u32 {
        let overlap = self.held.intersection(&self.winning).count();
        let overlap: u32 = overlap.try_into().unwrap();
        return overlap;
    }

    pub fn points(&self) -> u32 {
        let overlap = self.copies();
        match overlap {
            0 => 0,
            points => 2_u32.pow(points - 1),
        }
    }
}

mod input_parser {
    use super::Card;
    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, multispace0, multispace1, space0, space1},
        combinator::map_res,
        error::Error as NomError,
        multi::{many1, separated_list1},
        Finish, IResult,
    };

    fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
        separated_list1(multispace1, map_res(digit1, str::parse))(input)
    }

    fn parse_card(input: &str) -> IResult<&str, Card> {
        let (input, _) = tag("Card")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = digit1(input)?;

        let (input, _) = tag(":")(input)?;
        let (input, _) = space1(input)?;

        let (input, winning_numbers) = parse_numbers(input)?;

        let (input, _) = tag(" | ")(input)?;
        let (input, _) = space0(input)?;

        let (input, held_numbers) = parse_numbers(input)?;
        let (input, _) = multispace0(input)?;

        Ok((
            input,
            Card::new(
                winning_numbers.into_iter().collect(),
                held_numbers.into_iter().collect(),
            ),
        ))
    }

    pub fn parse_cards(input: &str) -> Result<Vec<Card>, NomError<&str>> {
        let (_, cards) = many1(parse_card)(input).finish()?;
        Ok(cards)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards: Vec<_> = input_parser::parse_cards(input).unwrap();
    let count = cards.into_iter().fold(0, |acc, e| acc + e.points());
    return Some(count);
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<_> = input_parser::parse_cards(input).unwrap();
    let mut counts = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let copies = card.copies();
        for copy in 1..(copies + 1) {
            counts[i + copy as usize] += counts[i]
        }
    }
    return Some(counts.iter().sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}

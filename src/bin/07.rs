use std::collections::HashMap;

advent_of_code::solution!(7);

mod input_parser {
    use super::Hand;
    use nom::{
        bytes::complete::{tag, take},
        character,
        character::complete::multispace0,
        error::Error as NomError,
        multi::many1,
        Finish, IResult,
    };

    fn parse_hand(input: &str) -> IResult<&str, Hand> {
        let (input, cards) = take(5usize)(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, bet) = character::complete::u32(input)?;
        let (input, _) = multispace0(input)?;
        Ok((input, Hand::from(cards, bet).unwrap()))
    }

    pub fn parse_input(input: &str) -> Result<Vec<Hand>, NomError<&str>> {
        let (_input, hands) = many1(parse_hand)(input).finish()?;
        Ok(hands)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl Card {
    fn from(c: &char) -> Result<Self, &str> {
        match c {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err("Invalid card value"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bet: u32,
}
impl Hand {
    fn from(card_str: &str, bet: u32) -> Result<Self, &str> {
        let cards: Vec<_> = card_str.chars().map(|c| Card::from(&c).unwrap()).collect();
        let mut counts = HashMap::new();
        for card in cards.iter() {
            counts
                .entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut ordered_counts = counts.values().collect::<Vec<_>>();
        ordered_counts.sort();
        let hand_type;
        match ordered_counts.as_slice() {
            [5] => hand_type = Some(HandType::FiveKind),
            [1, 4] => hand_type = Some(HandType::FourKind),
            [2, 3] => hand_type = Some(HandType::FullHouse),
            [1, 1, 3] => hand_type = Some(HandType::ThreeKind),
            [1, 2, 2] => hand_type = Some(HandType::TwoPair),
            [1, 1, 1, 2] => hand_type = Some(HandType::OnePair),
            [1, 1, 1, 1, 1] => hand_type = Some(HandType::HighCard),
            _ => hand_type = None,
        }
        return Ok(Self {
            hand_type: hand_type.unwrap(),
            cards,
            bet,
        });
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = input_parser::parse_input(input).unwrap();
    hands.sort();
    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) as u32 * hand.bet)
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

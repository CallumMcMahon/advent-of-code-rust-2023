advent_of_code::solution!(2);

mod arp_parser {
    use std::str::FromStr;

    use super::{Game, Grab, Cube, Colour};
    use nom::{
        bytes::complete::tag, 
        character::complete::{u8, digit1, alpha1, newline}, 
        combinator::into, 
        error::Error as NomError,
        sequence::{preceded, tuple, delimited}, 
        Finish, IResult, multi::separated_list0,
    };

    fn parse_cube(input: &str) -> IResult<&str, Cube> {
        let (remaining, item) = tuple((
            tag(" "),
            digit1,
            tag(" "),
            alpha1,
        ))(input)?;
        let cube = Cube {
            count: item.1.parse().unwrap(),
            colour: Colour::from_str(item.3).unwrap(),
        };
        return Ok((remaining, cube))
    }

    fn parse_grab(input: &str) -> IResult<&str, Grab> {
        let (remainder, grab) = separated_list0(
            tag(","), parse_cube
        )(input)?;
        return Ok((remainder, grab.into()))
    }

    fn parse_game(input: &str) -> IResult<&str, Game> {
        let (remainder, game_number) = delimited(
            tag("Game "), digit1, tag(":")
        )(input)?;
        let (remainder, grabs) = separated_list0(
            tag(";"), parse_grab
        )(remainder)?;
        Ok((remainder, Game {number: game_number.parse().unwrap(), grabs: grabs}))
    }

    fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
        separated_list0(newline, parse_game)(input)
    }
    
    pub fn parse_data(input: &str) -> Result<Vec<Game>, NomError<&str>> {
        let parse_result = into(
            parse_games
        )(input);
        let (_, games) = parse_result.finish()?;
        Ok(games)
    }
}

use std::str::FromStr;

enum Colour {
        Red,
        Green,
        Blue,
    }

    impl FromStr for Colour {

        type Err = ();
        fn from_str(input: &str) -> Result<Colour, Self::Err> {
            match input {
                "red"  => Ok(Colour::Red),
                "green"  => Ok(Colour::Green),
                "blue"  => Ok(Colour::Blue),
                _      => Err(()),
            }
        }
    }

struct Cube {
    count: u32,
    colour: Colour,
}

struct Grab {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<Vec<Cube>> for Grab {
    fn from(cubes: Vec<Cube>) -> Self {
        let mut grab_item = Grab {red: 0, green: 0, blue: 0};
        for cube in cubes {
            match cube.colour {
                Colour::Red => grab_item.red += cube.count,
                Colour::Green => grab_item.green += cube.count,
                Colour::Blue => grab_item.blue += cube.count,
            }
        }
        return grab_item
    } 
}

pub struct Game {
    number: u32,
    grabs: Vec<Grab>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let max_grab = Grab {red: 12, green: 13, blue: 14};
    let games = arp_parser::parse_data(input).unwrap();
    let mut count = 0;
    for game in games {
        let mut impossible = false;
        for grab in game.grabs {
            if grab.red > max_grab.red || grab.green > max_grab.green || grab.blue > max_grab.blue {
                impossible = true;
            }
        }
        if impossible == false {
            count += game.number;
        }
    }
    Some(count)
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

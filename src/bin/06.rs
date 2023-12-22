advent_of_code::solution!(6);

mod input_parser {
    use super::Race;
    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, multispace1, newline},
        combinator::map_res,
        error::Error as NomError,
        multi::separated_list1,
        Finish, IResult,
    };

    fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
        let (input, _) = tag("Time:")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, times) = separated_list1(multispace1, map_res(digit1, str::parse))(input)?;
        let (input, _) = newline(input)?;

        let (input, _) = tag("Distance:")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, distances) = separated_list1(multispace1, map_res(digit1, str::parse))(input)?;

        let races: Vec<Race> = times
            .iter()
            .zip(distances.iter())
            .map(|(&time, &distance)| Race { time, distance })
            .collect();
        return Ok((input, races));
    }

    pub fn parse_input(input: &str) -> Result<Vec<Race>, NomError<&str>> {
        let (_input, races) = parse_races(input).finish()?;
        return Ok(races);
    }
}

pub struct Race {
    time: u64,
    distance: u64,
}
impl Race {
    fn margin(&self) -> u32 {
        let time = self.time as f64;
        let distance = self.distance as f64;
        let component_1 = time / 2.0;
        let discriminator = time * time - 4.0 * distance;
        let component_2 = discriminator.sqrt() / 2.0;
        let mut lower = component_1 - component_2;

        if lower.fract() == 0.0 {
            lower += 1.0
        } else {
            lower = lower.ceil()
        }
        let mut upper = component_1 + component_2;
        if upper.fract() == 0.0 {
            upper = upper - 1.0
        } else {
            upper = upper.floor()
        }
        return (upper - lower) as u32 + 1;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let races = input_parser::parse_input(input).unwrap();
    return Some(races.iter().map(|x| x.margin()).product());
}

pub fn part_two(input: &str) -> Option<u32> {
    let races = input_parser::parse_input(input).unwrap();
    let mut kerning_time = 0;
    for race in races.iter() {
        let time_digits = (race.time).ilog10() + 1;
        kerning_time *= 10_u64.pow(time_digits);
        kerning_time += race.time;
    }
    let mut kerning_distance = 0;
    for race in races.iter() {
        let distance_digits = (race.distance).ilog10() + 1;
        kerning_distance *= 10_u64.pow(distance_digits);
        kerning_distance += race.distance;
    }
    let kerning_race = Race {
        time: kerning_time,
        distance: kerning_distance,
    };
    return Some(kerning_race.margin());
}

// time limit t_0
// distance to beat d_min
// charge time v variable

// travel time t = t_0 - v
// race distance d = t * v
// d = (t_0 - v) * v = v*t_0 - v^2
// winning solutions
// v*t_0 - v^2 > d_min
// -v^2 + v*t_0 - d_min > 0

// A=-1, B=t_0, C=-d_min
// -t_0 +- sqrt(t_0^2 - 4*(-1)*(-d_min)) / (2*(-1))
// t_0/2 -+ sqrt(t_0^2 - 4*d_min)/2

// t_0=7, d=9
// (-7 +- sqrt(49 - 4*9)) / -2 = 3.5 +- sqrt(13)/2 = (1.7, 5.3)
// -> 2, 3, 4, 5

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}

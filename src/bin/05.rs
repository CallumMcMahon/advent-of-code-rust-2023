advent_of_code::solution!(5);

use std::ops::Range;

use rayon::prelude::*;

struct MapRange {
    range: Range<u64>,
    translation: i64,
}

struct CategoryTransformer {
    mappings: Vec<MapRange>,
}
impl CategoryTransformer {
    fn transform_item(&self, item: u64) -> u64 {
        for map in self.mappings.iter() {
            if map.range.contains(&item) {
                return (item as i64 + map.translation) as u64;
            }
        }
        return item;
    }
}

pub struct Mapping {
    seeds: Vec<u64>,
    category_transformer: Vec<CategoryTransformer>,
}
impl Mapping {
    fn seed_to_location(&self, seed: u64) -> u64 {
        let mut current_val = seed;
        for map in self.category_transformer.iter() {
            current_val = map.transform_item(current_val)
        }
        return current_val;
    }

    fn seed_ranges(&self) -> Vec<Range<u64>> {
        let mut result = Vec::new();
        for chunk in self.seeds.chunks(2) {
            result.push(Range {
                start: chunk[0],
                end: chunk[0] + chunk[1],
            })
        }
        return result;
    }
}

mod input_parser {
    use std::ops::Range;

    use super::{CategoryTransformer, MapRange, Mapping};
    use nom::{
        bytes::complete::{tag, take_until},
        character::complete::{digit1, multispace0, newline, space1},
        combinator::map_res,
        error::Error as NomError,
        multi::{many1, separated_list1},
        Finish, IResult,
    };

    fn space_separated_nums(input: &str) -> IResult<&str, Vec<u64>> {
        let (input, nums) = separated_list1(space1, map_res(digit1, str::parse))(input)?;
        let (input, _) = newline(input)?;
        return Ok((input, nums));
    }

    fn parse_map(input: &str) -> IResult<&str, CategoryTransformer> {
        let (input, _) = take_until("map:\n")(input)?;
        let (input, _) = tag("map:")(input)?;
        let (input, _) = newline(input)?;

        let (input, maps) = many1(space_separated_nums)(input)?;
        let (input, _) = multispace0(input)?;
        let mut map_ranges = Vec::new();
        for map in maps.iter() {
            map_ranges.push(MapRange {
                range: Range { start: map[1], end: map[1] + map[2] },
                translation: map[0] as i64 - map[1] as i64,
            })
        }
        return Ok((input, CategoryTransformer { mappings: map_ranges }));
    }

    fn parse_maps(input: &str) -> IResult<&str, Mapping> {
        let (input, _) = tag("seeds: ")(input)?;
        let (input, seeds) = space_separated_nums(input)?;
        let (input, maps) = many1(parse_map)(input)?;
        return Ok((
            input,
            Mapping {
                seeds,
                category_transformer: maps,
            },
        ));
    }

    pub fn parse_input(input: &str) -> Result<Mapping, NomError<&str>> {
        let (_, maps) = parse_maps(input).finish()?;
        Ok(maps)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let maps = input_parser::parse_input(input).unwrap();
    return maps
        .seeds
        .par_iter()
        .map(|seed| maps.seed_to_location(*seed))
        .min();
}

// 467769747 + 210166838 + 33216796 + 86969850 + 378609832 + 314009711 + 36868255 + 170490105 + 265455365 + 31190888 = 1,994,747,387 elements
pub fn part_two(input: &str) -> Option<u64> {
    let maps = input_parser::parse_input(input).unwrap();
    let mut min_per_range = Vec::new();
    for seed_range in maps.seed_ranges() {
        min_per_range.push(
            seed_range
                .into_par_iter()
                .map(|seed| maps.seed_to_location(seed))
                .min()?,
        )
    }
    return min_per_range.into_iter().min();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}

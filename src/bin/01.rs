use std::collections::HashMap;

struct Calibration {
    first: char,
    last: char,
}

impl Calibration {
    fn value(self) -> u32 {
        let combined = self.first.to_string() + &self.last.to_string();
        return combined.parse().unwrap();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let mut start = None;
        let mut end = None;
        for c in line.chars() {
            if c.is_ascii_digit() {
                if start == None {
                    start = Some(c);
                    end = Some(c);
                } else {
                    end = Some(c)
                }
            }
        }
        let boundaries = Calibration {
            first: start.unwrap(),
            last: end.unwrap(),
        };
        count += boundaries.value();
    }
    return Some(count);
}

#[derive(PartialEq)]
struct DigitAndPos {
    digit: char,
    position: usize,
}

pub fn part_two(input: &str) -> Option<u32> {
    let name_to_digit = HashMap::from([
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    let mut count = 0;
    for line in input.lines() {
        // digit
        let mut first = None;
        let mut last = None;
        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                last = Some(DigitAndPos {
                    digit: c,
                    position: i,
                });
                if first == None {
                    first = Some(DigitAndPos {
                        digit: c,
                        position: i,
                    });
                }
            }
        }
        // textual
        for (textual, digit) in name_to_digit.iter() {
            if let Some(index) = line.find(textual) {
                if let Some(start_val) = first.as_ref() {
                    if index < start_val.position {
                        first = Some(DigitAndPos {
                            digit: *digit,
                            position: index,
                        });
                    }
                } else {
                    first = Some(DigitAndPos {
                        digit: *digit,
                        position: index,
                    });
                }
            }
            if let Some(index) = line.rfind(textual) {
                if let Some(end_val) = last.as_ref() {
                    if index > end_val.position {
                        last = Some(DigitAndPos {
                            digit: *digit,
                            position: index,
                        })
                    }
                } else {
                    last = Some(DigitAndPos {
                        digit: *digit,
                        position: index,
                    })
                }
            }
        }
        let boundaries = Calibration {
            first: first.unwrap().digit,
            last: last.unwrap().digit,
        };
        count += boundaries.value();
    }
    return Some(count);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(142));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(281));
    }
}

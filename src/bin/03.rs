advent_of_code::solution!(3);

struct Coord {
    d1: usize,
    d2: usize,
}

fn surroundings(from: Coord, d1_limit: usize, d2_limit: usize) -> Vec<Coord> {
    let mut valid_surroundings = Vec::new();
    // three positions above `from`
    if from.d1 > 0 {
        valid_surroundings.push(Coord {
            d1: from.d1 - 1,
            d2: from.d2,
        });
        if from.d2 > 0 {
            valid_surroundings.push(Coord {
                d1: from.d1 - 1,
                d2: from.d2 - 1,
            })
        }
        if from.d2 + 1 < d2_limit - 1 {
            valid_surroundings.push(Coord {
                d1: from.d1 - 1,
                d2: from.d2 + 1,
            })
        }
    }
    // left and right of `from`
    if from.d2 > 0 {
        valid_surroundings.push(Coord {
            d1: from.d1,
            d2: from.d2 - 1,
        })
    }
    if from.d2 + 1 < d2_limit - 1 {
        valid_surroundings.push(Coord {
            d1: from.d1,
            d2: from.d2 + 1,
        })
    }
    // three positions below `from`
    if from.d1 + 1 < d1_limit - 1 {
        valid_surroundings.push(Coord {
            d1: from.d1 + 1,
            d2: from.d2,
        });
        if from.d2 > 0 {
            valid_surroundings.push(Coord {
                d1: from.d1 + 1,
                d2: from.d2 - 1,
            })
        }
        if from.d2 + 1 < d2_limit - 1 {
            valid_surroundings.push(Coord {
                d1: from.d1 + 1,
                d2: from.d2 + 1,
            })
        }
    }
    valid_surroundings
}

fn is_symbol(input: &char) -> bool {
    return input != &'.' && input.is_ascii_punctuation();
}

fn does_surrounding_have_symbol(input_array: &Vec<Vec<char>>, from: Coord) -> bool {
    let surround = surroundings(from, input_array.len(), input_array[0].len());
    for coord in surround {
        if is_symbol(&input_array[coord.d1][coord.d2]) {
            return true;
        }
    }
    return false;
}

pub fn part_one(input: &str) -> Option<u32> {
    let input_array: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count: u32 = 0;
    for (row_idx, row) in input_array.iter().enumerate() {
        let mut col_idx = 0;
        while col_idx < row.len() {
            if !row[col_idx].is_ascii_digit() {
                col_idx += 1;
                continue;
            }
            let mut near_symbol = does_surrounding_have_symbol(
                &input_array,
                Coord {
                    d1: row_idx,
                    d2: col_idx,
                },
            );
            let mut col_end = 0;
            for (col_offset, val) in row[col_idx..].iter().enumerate() {
                if !val.is_ascii_digit() {
                    break;
                }
                col_end = col_offset;
                near_symbol = near_symbol
                    || does_surrounding_have_symbol(
                        &input_array,
                        Coord {
                            d1: row_idx,
                            d2: col_idx + col_offset,
                        },
                    );
            }
            if near_symbol {
                let new_num = row[col_idx..(col_idx + col_end + 1)]
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                count += new_num;
            }
            col_idx += col_end + 1
        }
    }
    return Some(count);
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

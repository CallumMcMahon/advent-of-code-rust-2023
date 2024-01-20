advent_of_code::solution!(9);

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let sequences: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|character| character.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    return sequences;
}

fn get_differences(sequence: Vec<i32>) -> Vec<Vec<i32>> {
    let mut differences = vec![sequence];
    loop {
        let new_sequence: Vec<_> = differences
            .last()
            .unwrap()
            .windows(2)
            .map(|x| x[1] - x[0])
            .collect();
        if new_sequence.iter().all(|&x| x == 0) {
            break;
        }
        differences.push(new_sequence);
    }
    return differences;
}

pub fn part_one(input: &str) -> Option<u32> {
    let sequences = parse_input(input);
    let differences: Vec<Vec<Vec<i32>>> = sequences.into_iter().map(get_differences).collect();
    let total: i32 = differences
        .iter()
        .map(|x| x.iter().rev().fold(0, |acc, x| acc + x.last().unwrap()))
        .sum();
    return Some(total as u32);
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
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

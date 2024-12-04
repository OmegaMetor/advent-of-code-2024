advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut half_one: Vec<u32> = Vec::new();
    let mut half_two: Vec<u32> = Vec::new();

    input.lines().for_each(|line| {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|number| number.parse::<u32>().unwrap())
            .collect();
        half_one.push(numbers[0]);
        half_two.push(numbers[1]);
    });
    half_one.sort();
    half_two.sort();
    (half_one, half_two)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (half_one, half_two) = parse_input(input);

    let mut total_dist: u32 = 0;

    for (left, right) in half_one.iter().zip(half_two.iter()) {
        if left > right {
            total_dist += left - right;
        } else {
            total_dist += right - left;
        }
    }

    Some(total_dist)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (half_one, half_two) = parse_input(input);

    let mut total_similarity: u32 = 0;

    half_one.iter().for_each(|&number| {
        total_similarity +=
            number * (half_two.iter().filter(|&number2| *number2 == number)).count() as u32
    });

    Some(total_similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut total = 0;

    for c in re.captures_iter(input) {
        total += c.get(1).unwrap().as_str().parse::<u32>().unwrap()
            * c.get(2).unwrap().as_str().parse::<u32>().unwrap();
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(?<mul>mul)\((?<num1>[0-9]{1,3}),(?<num2>[0-9]{1,3})\)|((?<do>do)\(\)|((?<dont>don't)\(\)))").unwrap();
    let mut total: u32 = 0;
    let mut enabled: bool = true;

    for c in re.captures_iter(input) {
        if enabled && c.name("mul").is_some() {
            let num1 = c.name("num1").unwrap().as_str().parse::<u32>().unwrap();
            let num2 = c.name("num2").unwrap().as_str().parse::<u32>().unwrap();
            total += num1 * num2;
        }
        if c.name("do").is_some() {
            enabled = true;
        }
        if c.name("dont").is_some() {
            enabled = false;
        }
    }

    Some(total)
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

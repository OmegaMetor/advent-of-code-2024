advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut reports: Vec<Vec<u32>> = Vec::new();
    input.lines().for_each(|line| {
        reports.push(
            line.split_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        )
    });

    let mut safe_reports: u32 = 0;

    'reports: for mut report in reports {
        if report[0] > report[1] {
            // decreasing
            report.reverse(); // Make it increasing
        }

        for i in 1..report.len() {
            if report[i - 1] > report[i] {
                // If decreasing
                continue 'reports;
            }
            let diff: u32 = report[i] - report[i - 1];
            if diff < 1 || diff > 3 {
                continue 'reports;
            }
        }

        safe_reports += 1;
    }

    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut reports: Vec<Vec<u32>> = Vec::new();
    input.lines().for_each(|line| {
        reports.push(
            line.split_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        )
    });

    let mut safe_reports: u32 = 0;

    for mut report in reports {
        if report[0] > report[1] {
            // decreasing
            report.reverse(); // Make it increasing
        }
        let mut problems = 0;

        for i in 1..report.len() {
            let current_value = report[i];
            let previous_value = report[i - 1];
            if previous_value > current_value {
                // If decreasing
                problems += 1;
                continue;
            }
            let diff: i32 = current_value as i32 - previous_value as i32;
            if diff < 1 || diff > 3 {
                problems += 1;
                continue;
            }
        }
        if problems <= 1 {
            safe_reports += 1;
        }
    }
    Some(safe_reports)
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

advent_of_code::solution!(5);

struct PageRule {
    page_before: u32,
    page: u32,
}

impl PageRule {
    pub fn new(page_before: u32, page: u32) -> Self {
        Self { page_before, page }
    }

    pub fn validate(&self, update: &Vec<u32>) -> bool {
        let page_before_index = update.iter().position(|n| *n == self.page_before);
        let page_index = update.iter().position(|n| *n == self.page);

        match (page_before_index, page_index) {
            (Some(page_before), Some(page)) => page_before < page,
            _ => true,
        }
    }

    pub fn fix(&self, update: &mut Vec<u32>) {
        let page_before_index = update.iter().position(|n| *n == self.page_before);
        let page_index = update.iter().position(|n| *n == self.page);

        match (page_before_index, page_index) {
            (Some(page_before), Some(page)) => {
                if page_before > page {
                    update[page_before] = self.page;
                    update[page] = self.page_before
                }
            }
            _ => {}
        };
    }
}

fn parse_input(input: &str) -> (Vec<PageRule>, Vec<Vec<u32>>) {
    let input_sections = input.split("\n\n").collect::<Vec<&str>>();
    let page_rules = input_sections[0]
        .lines()
        .map(|line| {
            let numbers = line
                .split("|")
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            PageRule::new(numbers[0], numbers[1])
        })
        .collect::<Vec<PageRule>>();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    for update_line in input_sections[1].lines() {
        updates.push(
            update_line
                .split(",")
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    (page_rules, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (page_rules, updates) = parse_input(input);

    let mut total_middle: u32 = 0;

    'update: for update in updates {
        for page_rule in page_rules.as_slice() {
            if !page_rule.validate(&update) {
                continue 'update;
            }
        }
        total_middle += update[(update.len() - 1) / 2];
    }

    Some(total_middle)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (page_rules, updates) = parse_input(input);

    let mut total_middle: u32 = 0;

    for mut update in updates {
        let mut valid = true;
        for page_rule in page_rules.as_slice() {
            if !page_rule.validate(&update) {
                valid = false;
                break;
            }
        }
        if valid {
            continue;
        }

        // Fix it

        while !valid {
            page_rules
                .iter()
                .for_each(|page_rule| page_rule.fix(&mut update));
            valid = page_rules
                .iter()
                .all(|page_rule| page_rule.validate(&update));
        }

        total_middle += update[(update.len() - 1) / 2];
    }

    Some(total_middle)
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

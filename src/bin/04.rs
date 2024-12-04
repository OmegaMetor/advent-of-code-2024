advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {

    let mut xmas_count = 0;

    let lines: Vec<&str> = input.lines().collect();

    for (char_offset, line_offset) in (-1..2).flat_map(|x| (-1..2).map(move |y| (x, y))) {
        if char_offset == 0 && line_offset == 0 {continue}
        xmas_count += search_offset(&lines, char_offset, line_offset);
    }

    Some(xmas_count)
}

fn search_offset(lines: &Vec<&str>, char_offset: i32, line_offset: i32) -> u32 {
    let mut xmas_count = 0;



    let (beg_lines_skip, end_lines_skip) = {
        match line_offset {
            -1 => ("XMAS".len()-1, 0),
            0 =>  (0, 0),
            1 =>  (0, "XMAS".len()-1),
            _ =>  (0, 0)
        }
    };

    let (beg_char_skip, end_char_skip) = {
        match char_offset {
            -1 => ("XMAS".len()-1, 0),
            0 =>  (0, 0),
            1 =>  (0, "XMAS".len()-1),
            _ =>  (0, 0)
        }
    };

    for line_loop_index in beg_lines_skip..(lines.len()-end_lines_skip) {
        'index: for char_loop_index in beg_char_skip..lines[line_loop_index].len()-end_char_skip {
            let (mut char_index, mut line_index) = (char_loop_index as i32, line_loop_index as i32);
            for char in "XMAS".chars() {
                if lines[line_index as usize].chars().nth(char_index as usize).unwrap() != char { continue 'index};
                char_index += char_offset;
                line_index += line_offset;

            }
            xmas_count += 1;
        }
    }

    xmas_count
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut x_mas_count: u32 = 0;
    for line_index in 1..lines.len() - 1 {
        for char_index in 1..lines[line_index].len()-1 {
            if lines[line_index][char_index] != 'A' { continue; }
            let (up_left, up_right, down_left, down_right) = (
                lines[line_index - 1][char_index - 1], lines[line_index - 1][char_index + 1],
                lines[line_index + 1][char_index - 1], lines[line_index + 1][char_index + 1]
            );
            if up_left == 'A' || up_right == 'A' || down_left == 'A' || down_right == 'A' { continue; };
            if up_left == 'X' || up_right == 'X' || down_left == 'X' || down_right == 'X' { continue; };

            if up_left == down_right || up_right == down_left { continue; };

            x_mas_count += 1;
        }

    };

    Some(x_mas_count)
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

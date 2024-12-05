advent_of_code::solution!(2);

fn parse_line(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

fn parse_lines(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(parse_line).collect()
}

fn is_safe(report: &&Vec<u32>) -> bool {
    let should_increase = report[0] < report[1];
    for i in 0..(report.len() - 1) {
        if (report[i] < report[i + 1]) != should_increase {
            return false;
        }
        let diff = report[i].abs_diff(report[i + 1]);
        if diff > 3 || diff < 1 {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let safe_reports = parse_lines(input).iter().filter(is_safe).count();
    Some(safe_reports as u32)
}

fn is_safe_with_dampener(report: &&Vec<u32>) -> bool {
    for i in 0..report.len() {
        let mut clone: Vec<u32> = report.to_vec();
        clone.remove(i);
        if is_safe(&&clone) {
            return true;
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let safe_reports = parse_lines(input).iter().filter(is_safe_with_dampener).count();
    Some(safe_reports as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}

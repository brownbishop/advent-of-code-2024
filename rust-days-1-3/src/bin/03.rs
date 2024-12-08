advent_of_code::solution!(3);
use regex::Regex;

const MUL: &str = r"mul\((\d{1,3}),(\d{1,3})\)";
const DO: &str = r"do\(\)";
const DONT: &str = r"don't\(\)";

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(MUL).unwrap();
    let mut sum = 0;
    for captures in re.captures_iter(input) {
        let x = captures[1].parse::<u32>().unwrap();
        let y = captures[2].parse::<u32>().unwrap();
        sum += x * y;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(&format!("{DO}|{DONT}|{MUL}")).unwrap();
    let mut enabled = true;
    let mut sum: u32 = 0;
    for captures in re.find_iter(input) {
        match captures.as_str() {
            "do()" => {
                enabled = true
            }
            "don't()" => enabled = false,
            val if enabled => {
                if let Some(res) = part_one(val) {
                    sum += res;
                }
            },
            &_ => {}
        }
    }
    Some(sum)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut first: Vec<u32> = vec![];
    let mut second: Vec<u32> = vec![];
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        first.push(iter.next().unwrap().parse::<u32>().unwrap());
        second.push(iter.next().unwrap().parse::<u32>().unwrap());
    }
    (first, second)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lists = parse_input(input);
    lists.0.sort();
    lists.1.sort();

    let mut sum: u32 = 0;
    for i in 0..lists.0.len() {
        sum += if lists.0[i] > lists.1[i] {
            lists.0[i] - lists.1[i]
        } else {
            lists.1[i] - lists.0[i]
        };
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lists = parse_input(input);

    let sum = lists.0.iter().map(|i| {
        let count = lists.1.iter().filter(|x| **x == *i).count();
        i * count as u32
    }).sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        let val = result.unwrap();
        println!("{val}");
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

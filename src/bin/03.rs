use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;

    let regex = Regex::new(r"mul\(([0-9]*,[0-9]*)\)").unwrap();

    for s in regex.captures_iter(input) {
        let (num1, num2) = s.get(1)?.as_str().split(",").collect_tuple()?;
        result += num1.parse::<u32>().unwrap() * num2.parse::<u32>().unwrap();
    }

    Some(result)
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

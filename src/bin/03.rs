use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(3);

enum ResultOfMatch {
    Mul(u32, u32),
    Do,
    Dont,
}

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
    let mut result = 0;

    let regex = Regex::new(r"mul\(([0-9]*,[0-9]*)\)|do\(\)|don't\(\)").unwrap();

    let map = regex.captures_iter(input).filter_map(|s| {
        if let Some(regex_match) = s.get(1) {
            let (num1, num2) = regex_match.as_str().split(",").collect_tuple()?;
            Some(ResultOfMatch::Mul(
                num1.parse::<u32>().unwrap(),
                num2.parse::<u32>().unwrap(),
            ))
        } else if s.get(0)?.as_str() == "do()" {
            Some(ResultOfMatch::Do)
        } else if s.get(0)?.as_str() == "don't()" {
            Some(ResultOfMatch::Dont)
        } else {
            None
        }
    });

    let mut doing = true;

    for i in map {
        match i {
            ResultOfMatch::Mul(first, second) => {
                if doing {
                    result += first * second;
                }
            }
            ResultOfMatch::Do => doing = true,
            ResultOfMatch::Dont => doing = false,
        }
    }

    Some(result)
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

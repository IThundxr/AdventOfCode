advent_of_code::solution!(2);

pub fn is_safe(nums: &[isize]) -> bool {
    let first_difference = nums[1] - nums[0];

    for i in 1..nums.len() {
        let difference = nums[i] - nums[i - 1];

        if difference.abs() < 1
            || difference.abs() > 3
            || (difference.signum() != first_difference.signum())
        {
            return false;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<usize> {
    let result = input
        .lines()
        .filter(|&line| {
            let nums = line
                .split_ascii_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>();

            is_safe(&nums)
        })
        .count();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let result = input
        .lines()
        .filter(|&line| {
            let nums = line
                .split_ascii_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>();

            if !is_safe(&nums) {
                for i in 0..nums.len() {
                    let mut new_nums = nums.clone();
                    new_nums.remove(i);

                    if is_safe(&new_nums) {
                        return true;
                    }
                }
            } else {
                return true;
            }

            false
        })
        .count();

    Some(result)
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

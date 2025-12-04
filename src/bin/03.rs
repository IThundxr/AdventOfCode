use itertools::Itertools;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;
    for line in input.lines().filter(|l| !l.is_empty()) {
        let mut digits = line
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u64))
            .collect::<Vec<_>>();

        let mut first_index = 0;
        let (mut first_val, mut second_val) = (0, 0);
        for i in 0..(digits.len() - 1) {
            let digit = digits[i];

            if digit > first_val {
                first_index = i + 1;
                first_val = digit;
            }
        }

        digits.reverse();
        for i in 0..(digits.len() - first_index) {
            let digit = digits[i];

            if digit > second_val {
                second_val = digit;
            }
        }

        total += first_val * 10 + second_val;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;
    for line in input.lines().filter(|l| !l.is_empty()) {
        let digits = line
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u64))
            .collect::<Vec<_>>();

        let on_batteries = &mut digits.to_vec()[digits.len() - 12..];

        for i in (0..=digits.len() - 13).rev() {
            let mut battery = digits[i];

            for o in 0..on_batteries.len() {
                if battery >= on_batteries[o] {
                    std::mem::swap(&mut on_batteries[o], &mut battery);
                } else {
                    break;
                }
            }
        }

        let joined = on_batteries.iter().join("").parse::<u64>().unwrap();
        total += joined;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.split('\n');

    let mut current_num: i16 = 50;
    let mut zeros = 0;
    for line in lines.filter(|l| !l.is_empty()) {
        let (direction, turn_amount) = line.split_at(1);
        let turn_amount = turn_amount.parse::<i16>().ok()?;

        match direction {
            "L" => current_num = (current_num - turn_amount).rem_euclid(100),
            "R" => current_num = (current_num + turn_amount).rem_euclid(100),
            _ => return None, // Impossible to hit
        };

        if current_num == 0 {
            zeros += 1;
        }
    }

    Some(zeros)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.split('\n');

    let mut current_num: i16 = 50;
    let mut zeros = 0;
    for line in lines.filter(|l| !l.is_empty()) {
        let (direction, turn_amount) = line.split_at(1);
        let turn_amount = turn_amount.parse::<i16>().ok()?;

        for _ in 0..turn_amount {
            current_num = match direction {
                "L" => current_num - 1,
                "R" => current_num + 1,
                _ => return None, // Impossible to hit
            }
            .rem_euclid(100);

            if current_num == 0 {
                zeros += 1;
            }
        }
    }

    Some(zeros)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

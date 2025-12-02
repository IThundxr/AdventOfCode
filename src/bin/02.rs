advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;
    for range in input.replace("\n", "").split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let (start, end) = (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap());

        for i in start..=end {
            let str = i.to_string();
            if str.len() % 2 != 0 {
                continue;
            }

            let mid = str.len() / 2;
            let (left, right) = str.split_at(mid);
            if left == right {
                total += i;
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;
    for range in input.replace("\n", "").split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let (start, end) = (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap());

        for i in start..=end {
            let str = i.to_string();
            let len = str.len();

            for block_len in 1..=(len / 2) {
                if len % block_len != 0 {
                    continue;
                }

                let block = &str[..block_len];
                let mut is_valid = true;

                for i in (block_len..len).step_by(block_len) {
                    if &str[i..i + block_len] != block {
                        is_valid = false;
                        break;
                    }
                }

                if is_valid {
                    total += i;
                    break;
                }
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;

    let mut first_list_sorted: Vec<u32> = Vec::new();
    let mut second_list_sorted: Vec<u32> = Vec::new();

    for line in input.lines() {
        let (first_list, second_list) = line.split_once("   ")?;
        first_list_sorted.push(first_list.parse::<u32>().unwrap());
        second_list_sorted.push(second_list.parse::<u32>().unwrap());
    }

    first_list_sorted.sort();
    second_list_sorted.sort();

    for i in 0..first_list_sorted.len() {
        let first_number = first_list_sorted.get(i)?;
        let second_number = second_list_sorted.get(i)?;

        result += first_number.abs_diff(*second_number);
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;

    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();

    for line in input.lines() {
        let (first, second) = line.split_once("   ")?;

        let first_num = first.parse::<u32>().unwrap();
        let second_num = second.parse::<u32>().unwrap();

        first_list.push(first_num);
        second_list.push(second_num);
    }

    for first in first_list {
        let mut second_count: u32 = 0;

        for &second in &second_list {
            if first == second {
                second_count += 1;
            }
        }

        result += first * second_count;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

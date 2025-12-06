use advent_of_code::util::range::Range;
use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let (raw_ranges, raw_ids) = input.trim().split_once("\n\n").unwrap();

    let fresh_ranges = raw_ranges
        .split("\n")
        .map(Range::parse)
        .filter_map(Result::ok)
        .collect_vec();

    let fresh_foods = raw_ids
        .split("\n")
        .map(|id| id.parse::<usize>().unwrap())
        .filter(|&i| {
            for range in fresh_ranges.iter() {
                if range.contains(i) {
                    return true;
                }
            }

            false
        })
        .count();

    Some(fresh_foods)
}

pub fn part_two(input: &str) -> Option<usize> {
    let raw_ranges = input.trim().split_once("\n\n").unwrap().0;

    let mut ranges = raw_ranges
        .split("\n")
        .map(Range::parse)
        .filter_map(Result::ok)
        .collect_vec();
    ranges.sort();

    let mut merged = Vec::from([ranges[0]]);
    for r in &ranges[1..] {
        let other = merged.last().unwrap();

        let (start, end) = (r.start, r.end);
        let (start_other, end_other) = (other.start, other.end);

        if start > end_other {
            merged.push(*r);
        } else {
            *merged.last_mut().unwrap() = Range::new(start_other, end_other.max(end))
        }
    }

    Some(merged.iter().map(|&r| r.end - r.start + 1).sum())
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
        assert_eq!(result, Some(14));
    }
}

use advent_of_code::util::grid::Grid;
use std::collections::HashMap;

advent_of_code::solution!(7);

fn day_7(input: &str) -> Option<(u64, u64)> {
    let grid = Grid::parse(input);

    let (start_x, _start_y) = grid.find_first(b'S')?;

    let mut beams: HashMap<(i32, i32), u64> = HashMap::from([((start_x, 0_i32), 1)]);
    let mut split_count = 0;

    loop {
        let mut new_beams = HashMap::new();
        for (&(x, y), &v) in &beams {
            let ny = y + 1;

            if !grid.contains((x, ny)) {
                continue;
            }

            match grid[(x, ny)] {
                b'.' => {
                    *new_beams.entry((x, ny)).or_default() += v;
                }
                b'^' => {
                    for nx in [x - 1, x + 1] {
                        *new_beams.entry((nx, ny)).or_default() += v;
                    }
                    split_count += 1;
                }
                _ => continue,
            }
        }
        if new_beams.is_empty() {
            break;
        }
        beams = new_beams;
    }

    Some((split_count, beams.values().sum()))
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(day_7(input)?.0)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(day_7(input)?.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}

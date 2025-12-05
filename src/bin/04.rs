use advent_of_code::util::grid::Grid;

advent_of_code::solution!(4);

const ADJACENTS: [(i32, i32); 8] = [
    (-1, -1), // ↖
    (-1, 0),  // ↑
    (-1, 1),  // ↗
    (0, -1),  // ←
    (0, 1),   // →
    (1, -1),  // ↙
    (1, 0),   // ↓
    (1, 1),   // ↘
];

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::parse(input);

    let mut accessible_rolls = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid[(x, y)] == b'@' {
                let adjacent_rolls = get_adjacent_rolls(x, y, &grid);

                if adjacent_rolls < 4 {
                    accessible_rolls += 1;
                }
            }
        }
    }

    Some(accessible_rolls)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::parse(input);

    let mut last_removed_rolls = 1;
    let mut removed_rolls_total = 0;
    while last_removed_rolls >= 1 {
        let mut rolls_removed = 0;

        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid[(x, y)] == b'@' {
                    let adjacent_rolls = get_adjacent_rolls(x, y, &grid);

                    if adjacent_rolls < 4 {
                        rolls_removed += 1;
                        grid[(x, y)] = b'x';
                    }
                }
            }
        }

        removed_rolls_total += rolls_removed;
        last_removed_rolls = rolls_removed;
    }

    Some(removed_rolls_total)
}

fn get_adjacent_rolls(x: i32, y: i32, grid: &Grid<u8>) -> usize {
    ADJACENTS
        .iter()
        .map(|(x_m, y_m)| (x_m + x, y_m + y))
        .filter(|&next| grid.contains(next) && grid[next] == b'@')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}

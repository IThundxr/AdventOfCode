use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(6);

#[derive(Debug)]
struct Question<T> {
    numbers: Vec<T>,
    is_addition: bool,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut questions: Vec<Question<u16>> = Vec::new();
    for line in input.trim().lines() {
        for (index, number) in line.split_whitespace().map(|s| s.trim()).enumerate() {
            if let Some(question) = questions.get_mut(index) {
                let is_addition = number == "+";
                let is_multiplication = number == "*";
                if is_addition || is_multiplication {
                    question.is_addition = is_addition;
                } else {
                    question.numbers.push(number.parse().unwrap());
                }
            } else {
                questions.push(Question {
                    numbers: vec![number.parse::<u16>().unwrap()],
                    is_addition: true,
                });
            }
        }
    }

    let answer = questions
        .iter()
        .map(|q| {
            let init = if q.is_addition { 0 } else { 1 };
            q.numbers.iter().fold(init, |sum, &i| {
                let i = i as u64;
                if q.is_addition { sum + i } else { sum * i }
            })
        })
        .sum::<u64>();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let regex = Regex::new(r"([*+])(\s*)").unwrap();

    let lines = input.trim_end().lines().collect_vec();
    let numbers = &lines[..lines.len() - 1];
    let &operations = lines.last().unwrap();

    // (column_size, question)
    let mut questions: Vec<(usize, Question<Vec<Option<u16>>>)> = Vec::new();

    for matched in regex.captures_iter(operations) {
        let is_addition = matched.get(1).unwrap().as_str() == "+";
        let size = matched.get(2).unwrap().as_str().chars().count();

        questions.push((
            size,
            Question {
                // Y-axis
                numbers: Vec::new(),
                is_addition,
            },
        ))
    }

    for &numbers_line in numbers {
        let mut starting_index = 0;
        for (column_size, question) in questions.iter_mut() {
            let number = if column_size == &0 {
                &numbers_line[starting_index..]
            } else {
                &numbers_line[starting_index..starting_index + *column_size]
            };

            // X-axis
            let numbers = number
                .chars()
                .map(|c| c.to_digit(10).map(|d| d as u16))
                .collect_vec();

            question.numbers.push(numbers);

            starting_index += *column_size + 1
        }
    }

    let sum = questions
        .iter()
        .rev()
        .map(|(_, q)| {
            let height = q.numbers.len();
            let width = q.numbers.iter().map(|row| row.len()).max().unwrap_or(0);

            let init = if q.is_addition { 0 } else { 1 };
            (0..width)
                .rev() // columns right to left
                .map(|x| {
                    (0..height) // iterate rows top down
                        .filter_map(|y| q.numbers[y].get(x).and_then(|&d| d))
                        .fold(0u64, |acc, d| acc * 10 + d as u64)
                })
                .fold(init, |sum, i| if q.is_addition { sum + i } else { sum * i })
        })
        .sum::<u64>();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}

use std::time::SystemTime;

fn main() {
    let time_start = SystemTime::now();

    let use_example = false;

    let input: &str;

    if !use_example {
        input = include_str!("../input/day2.txt");
    } else {
        input = include_str!("../input/day2_example.txt");
    }

    let mut game_sum: i32 = 0;
    let mut game_power_thing: i32 = 0;

    // Game 1: 1 green, 2 red, 6 blue; 4 red, 1 green, 3 blue; 7 blue, 5 green; 6 blue, 2 red, 1 green
    for line in input.lines() {
        // game string = Game 123, other_input = 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let (game_string, other_input) = line.split_once(": ").unwrap();
        // game_number = 123
        let (_, game_number_string) = game_string.split_once(" ").unwrap();
        let game_number = game_number_string.parse::<i32>().unwrap();

        // 1 green, 2 red, 6 blue
        // 4 red, 1 green, 3 blue
        // 7 blue, 5 green
        // 6 blue, 2 red, 1 green
        let rolls = other_input.split("; ");

        let mut game_possible = true;

        let mut lowest_amount = Cubes { red: 0, green: 0, blue: 0 };
        let mut power_of_cubes = 0;

        // 1 green, 2 red, 6 blue
        for roll in rolls {
            let mut cubes = Cubes { red: 0, green: 0, blue: 0 };

            // 1 green
            // 2 red
            // 6 blue
            let parts = roll.split(", ");

            // 1 green
            for part in parts {
                let (number, color) = part.split_once(' ').unwrap();

                let result: i32 = match number.parse() {
                    Ok(val) => val,
                    Err(_) => continue,
                };

                match color {
                    "red" => cubes.red += result,
                    "green" => cubes.green += result,
                    "blue" => cubes.blue += result,
                    _ => (),
                }
            }

            lowest_amount.red = lowest_amount.red.max(cubes.red);
            lowest_amount.green = lowest_amount.green.max(cubes.green);
            lowest_amount.blue = lowest_amount.blue.max(cubes.blue);

            if !cubes.is_possible() {
                game_possible = false;
            }
        }

        power_of_cubes = lowest_amount.red * lowest_amount.green * lowest_amount.blue;

        game_power_thing += power_of_cubes;

        if game_possible {
            game_sum += game_number;
        }
    }

    let time_taken = SystemTime::now().duration_since(time_start).unwrap().as_micros();

    println!("Part 1: {} \nPart 2: {} \n\nTook a total of {}ms to complete", game_sum, game_power_thing, time_taken);
}

impl Cubes {
    fn is_possible(&self) -> bool {
        self.red <= CUBE_LIMITS.red
        && self.green <= CUBE_LIMITS.green
        && self.blue <= CUBE_LIMITS.blue
    }
}

const CUBE_LIMITS: Cubes = Cubes {
    red: 12,
    green: 13,
    blue: 14
};

struct Cubes {
    red: i32,
    green: i32,
    blue: i32
}

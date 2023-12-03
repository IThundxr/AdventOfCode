fn main() {
    let input = include_str!("../input/day2.txt");

    let mut game_sum: i32 = 0;

    // Game 1: 1 green, 2 red, 6 blue; 4 red, 1 green, 3 blue; 7 blue, 5 green; 6 blue, 2 red, 1 green
    for line in input.lines() {
        let mut cubes = Cubes { red: 0, green: 0, blue: 0 };

        // game string = Game 123, other_input = 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let (game_string, other_input) = line.split_once(": ").unwrap();
        // game_number = 123
        let (_, game_number) = game_string.split_once(" ").unwrap();

        // 1 green, 2 red, 6 blue
        // 4 red, 1 green, 3 blue
        // 7 blue, 5 green
        // 6 blue, 2 red, 1 green
        let rolls = other_input.split("; ");

        // 1 green, 2 red, 6 blue
        for roll in rolls {
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
        }
    }

    println!("{}", game_sum)
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

#[derive(Clone, Copy)]
struct Cubes {
    red: i32,
    green: i32,
    blue: i32
}

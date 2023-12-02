use crate::utils;

type Cubes = Vec<Cube>;

struct Game {
    id: u32,
    sets_of_cubes: Vec<Cubes>,
}

enum Cube {
    Blue(u8),
    Green(u8),
    Red(u8),
}

impl Cube {
    fn parse(input: String) -> Self {
        let (count_str, color) = input.split_once(" ").unwrap();

        return match color {
            "blue" => Cube::Blue(count_str.parse::<u8>().unwrap()),
            "green" => Cube::Green(count_str.parse::<u8>().unwrap()),
            "red" => Cube::Red(count_str.parse::<u8>().unwrap()),
            _ => panic!("Invalid color"),
        };
    }
}

impl Game {
    fn parse(input: String) -> Game {
        let (id_str, set_of_cubes_str) = input.split_once(": ").unwrap();

        return Game {
            id: id_str.replace("Game ", "").parse::<u32>().unwrap(),
            sets_of_cubes: set_of_cubes_str
                .split("; ")
                .map(|cubes_str| {
                    cubes_str
                        .split(", ")
                        .map(|cube_str| Cube::parse(cube_str.to_string()))
                        .collect::<Cubes>()
                })
                .collect(),
        };
    }

    fn valid(&self) -> bool {
        return self
            .sets_of_cubes
            .iter()
            .filter(|cubes| {
                cubes.iter().any(|cube| match cube {
                    Cube::Red(i) => *i > 12,
                    Cube::Green(i) => *i > 13,
                    Cube::Blue(i) => *i > 14,
                })
            })
            .collect::<Vec<&Cubes>>()
            .is_empty();
    }

    fn minimum_set(&self) -> Cubes {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for cubes in self.sets_of_cubes.iter() {
            for cube in cubes.iter() {
                match cube {
                    Cube::Red(i) => min_red = min_red.max(*i),
                    Cube::Green(i) => min_green = min_green.max(*i),
                    Cube::Blue(i) => min_blue = min_blue.max(*i),
                }
            }
        }

        return vec![
            Cube::Red(min_red),
            Cube::Green(min_green),
            Cube::Blue(min_blue),
        ];
    }
}

pub fn solve() {
    let input = utils::read_input("inputs/day_02.txt");

    println!("Day 02, Puzzle 01: {}", puzzle_01(&input));
    println!("Day 02, Puzzle 02: {}", puzzle_02(&input));
}

fn puzzle_01(input: &String) -> u32 {
    return input
        .lines()
        .map(|line| Game::parse(line.to_string()))
        .filter(Game::valid)
        .map(|game| game.id)
        .sum::<u32>();
}

fn puzzle_02(input: &String) -> u32 {
    return input
        .lines()
        .map(|line| Game::parse(line.to_string()))
        .map(|game| {
            game.minimum_set()
                .iter()
                .fold(1 as u32, |acc, cube| match cube {
                    Cube::Red(i) => acc * (*i as u32),
                    Cube::Green(i) => acc * (*i as u32),
                    Cube::Blue(i) => acc * (*i as u32),
                }) as u32
        })
        .sum::<u32>();
}

mod tests {
    #[test]
    fn test_puzzle_01() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        let result = super::puzzle_01(&input.to_string());

        assert_eq!(result, 8);
    }

    #[test]
    fn test_puzzle_02() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        let result = super::puzzle_02(&input.to_string());

        assert_eq!(result, 2286)
    }
}

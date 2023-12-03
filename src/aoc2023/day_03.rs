use std::collections::HashMap;

use crate::utils::read_input;

const DR: [i8; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];
const DC: [i8; 8] = [0, 1, 1, 1, 0, -1, -1, -1];

struct EngineSchematic(Vec<Vec<char>>);

impl EngineSchematic {
    fn parse(input: &str) -> Self {
        return EngineSchematic(
            input
                .lines()
                .map(|line| line.chars().collect())
                .collect::<Vec<Vec<char>>>(),
        );
    }

    fn row_len(&self) -> usize {
        return self.0.len();
    }

    fn col_len(&self) -> usize {
        return self.0[0].len();
    }

    fn point(&self, r: usize, c: usize) -> Point {
        return Point::new(r, c, self.0[r][c]);
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    value: char,
}

impl Point {
    fn new(y: usize, x: usize, value: char) -> Self {
        return Point { x, y, value };
    }

    fn is_digit(&self) -> bool {
        return self.value.is_digit(10);
    }

    fn collect_numbers(
        &self,
        engine: &EngineSchematic,
        visited: &mut Vec<Vec<bool>>,
    ) -> HashMap<Point, usize> {
        let mut results = HashMap::new();

        for k in 0..8 {
            let i = self.y as isize + DR[k] as isize;
            let j = self.x as isize + DC[k] as isize;

            if i < 0 || i >= engine.row_len() as isize || j < 0 || j >= engine.col_len() as isize {
                continue;
            }
            if visited[i as usize][j as usize] {
                continue;
            }

            let point = engine.point(i as usize, j as usize);
            if point.is_digit() {
                let mut number_str = String::from(point.value);
                visited[i as usize][j as usize] = true;

                let mut _j = j;
                loop {
                    _j -= 1;

                    if _j < 0 || _j >= engine.col_len() as isize {
                        break;
                    }
                    if visited[i as usize][_j as usize] {
                        break;
                    }

                    let point = engine.point(i as usize, _j as usize);
                    if !point.is_digit() {
                        break;
                    }

                    number_str.insert(0, point.value);
                    visited[i as usize][_j as usize] = true;
                }
                let mut _j = j;
                loop {
                    _j += 1;

                    if _j < 0 || _j >= engine.col_len() as isize {
                        break;
                    }
                    if visited[i as usize][_j as usize] {
                        break;
                    }

                    let point = engine.point(i as usize, _j as usize);
                    if !point.is_digit() {
                        break;
                    }

                    number_str.push(point.value);
                    visited[i as usize][_j as usize] = true;
                }

                results.insert(point, number_str.parse::<usize>().unwrap());
            }
        }

        return results;
    }
}

pub fn solve() {
    let input = read_input("inputs/day_03.txt");

    println!("Day 03, Puzzle 01: {}", puzzle_01(&input));
    println!("Day 03, Puzzle 02: {}", puzzle_02(&input));
}

fn puzzle_01(input: &str) -> usize {
    let engine = EngineSchematic::parse(input);
    let mut visited = vec![vec![false; engine.col_len()]; engine.row_len()];
    let mut sum = 0;

    for r in 0..engine.row_len() {
        for c in 0..engine.col_len() {
            let point = engine.point(r, c);

            if point.value == '.' || point.is_digit() {
                continue;
            }

            sum += point
                .collect_numbers(&engine, &mut visited)
                .values()
                .sum::<usize>();
        }
    }

    return sum;
}

fn puzzle_02(input: &str) -> usize {
    let engine = EngineSchematic::parse(input);
    let mut visited = vec![vec![false; engine.col_len()]; engine.row_len()];
    let mut sum = 0;

    for r in 0..engine.row_len() {
        for c in 0..engine.col_len() {
            let point = engine.point(r, c);
            if point.value != '*' {
                continue;
            }

            let numbers = point.collect_numbers(&engine, &mut visited);

            if numbers.len() != 2 {
                continue;
            }

            let mut iter = numbers.iter();
            let p_1 = iter.next().unwrap();
            let p_2 = iter.next().unwrap();
            if (p_1.0.y as isize - p_2.0.y as isize).abs() == 2
                || (p_1.0.x as isize - p_2.0.x as isize).abs() == 2
            {
                sum += p_1.1 * p_2.1;
            }
        }
    }

    return sum;
}

mod tests {
    #[test]
    fn test_puzzle_01() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let result = super::puzzle_01(&input.to_string());

        assert_eq!(result, 4361);
    }

    #[test]
    fn test_puzzle_02() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let result = super::puzzle_02(&input.to_string());

        assert_eq!(result, 467835);
    }
}

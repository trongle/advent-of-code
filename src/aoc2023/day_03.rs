use std::collections::HashMap;

use crate::utils::read_input;

const DR: [i8; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];
const DC: [i8; 8] = [0, 1, 1, 1, 0, -1, -1, -1];

pub fn solve() {
    let input = read_input("inputs/day_03.txt");

    println!("Day 03, Puzzle 01: {}", puzzle_01(&input));
    println!("Day 03, Puzzle 02: {}", puzzle_02(&input));
}

fn puzzle_01(input: &str) -> usize {
    let grids = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut visited = vec![vec![false; grids[0].len()]; grids.len()];
    let mut sum = 0;

    for r in 0..grids.len() {
        for c in 0..grids[0].len() {
            if grids[r][c] == '.' || grids[r][c].is_digit(10) {
                continue;
            }

            // collect all numbers around it
            for k in 0..8 {
                let i = r as isize + DR[k] as isize;
                let j = c as isize + DC[k] as isize;

                if i < 0 || i >= grids.len() as isize || j < 0 || j >= grids[0].len() as isize {
                    continue;
                }
                if visited[i as usize][j as usize] {
                    continue;
                }

                if grids[i as usize][j as usize].is_digit(10) {
                    let mut number_str = String::from(grids[i as usize][j as usize]);
                    visited[i as usize][j as usize] = true;

                    let mut _j = j;
                    loop {
                        _j -= 1;
                        if _j < 0
                            || _j >= grids[0].len() as isize
                            || !grids[i as usize][_j as usize].is_digit(10)
                        {
                            break;
                        }
                        if visited[i as usize][_j as usize] {
                            break;
                        }

                        number_str.insert(0, grids[i as usize][_j as usize]);
                        visited[i as usize][_j as usize] = true;
                    }
                    let mut _j = j;
                    loop {
                        _j += 1;
                        if _j < 0
                            || _j >= grids[0].len() as isize
                            || !grids[i as usize][_j as usize].is_digit(10)
                        {
                            break;
                        }
                        if visited[i as usize][_j as usize] {
                            break;
                        }

                        number_str.push(grids[i as usize][_j as usize]);
                        visited[i as usize][_j as usize] = true;
                    }

                    sum += number_str.parse::<usize>().unwrap();
                }
            }
        }
    }

    return sum;
}

fn puzzle_02(input: &str) -> usize {
    let grids = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut visited = vec![vec![false; grids[0].len()]; grids.len()];
    let mut sum = 0;

    for r in 0..grids.len() {
        for c in 0..grids[0].len() {
            if grids[r][c] != '*' {
                continue;
            }

            // collect all numbers around it
            let mut numbers: HashMap<String, usize> = HashMap::new();
            for k in 0..8 {
                let i = r as isize + DR[k] as isize;
                let j = c as isize + DC[k] as isize;

                if i < 0 || i >= grids.len() as isize || j < 0 || j >= grids[0].len() as isize {
                    continue;
                }
                if visited[i as usize][j as usize] {
                    continue;
                }

                if grids[i as usize][j as usize].is_digit(10) {
                    let mut number_str = String::from(grids[i as usize][j as usize]);
                    visited[i as usize][j as usize] = true;

                    let mut _j = j;
                    loop {
                        _j -= 1;
                        if _j < 0
                            || _j >= grids[0].len() as isize
                            || !grids[i as usize][_j as usize].is_digit(10)
                        {
                            break;
                        }
                        if visited[i as usize][_j as usize] {
                            break;
                        }

                        number_str.insert(0, grids[i as usize][_j as usize]);
                        visited[i as usize][_j as usize] = true;
                    }
                    let mut _j = j;
                    loop {
                        _j += 1;
                        if _j < 0
                            || _j >= grids[0].len() as isize
                            || !grids[i as usize][_j as usize].is_digit(10)
                        {
                            break;
                        }
                        if visited[i as usize][_j as usize] {
                            break;
                        }

                        number_str.push(grids[i as usize][_j as usize]);
                        visited[i as usize][_j as usize] = true;
                    }

                    numbers.insert(format!("{i}|{j}"), number_str.parse::<usize>().unwrap());
                }
            }

            if numbers.len() != 2 {
                continue;
            }

            let mut iter = numbers.iter();
            let p_1 = iter.next().unwrap();
            let p_2 = iter.next().unwrap();
            let (r_1, c_1) = p_1.0.split_once("|").unwrap();
            let (r_2, c_2) = p_2.0.split_once("|").unwrap();
            if (r_1.parse::<isize>().unwrap() - r_2.parse::<isize>().unwrap()).abs() == 2
                || (c_1.parse::<isize>().unwrap() - c_2.parse::<isize>().unwrap()).abs() == 2
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

//
// - loop throught the grid
// - check if the current cell is a symbol (dot is not a symbol):
//  - if it is a symbol, collect all numbers around it
// arr = []
// i = 0, y = 0: 4 skip
// i = 1, y = 0: 6 skip
// i = 2, y = 0: 7 skip
// i = 3, y = 0: . skip
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo
// ooooooooooo

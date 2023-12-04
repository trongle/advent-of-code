use std::collections::VecDeque;

use crate::utils;

pub fn solve() {
    let input = utils::read_input("inputs/day_04.txt");

    println!("Day 04, Puzzle 01: {}", puzzle_01(&input));
    println!("Day 04, Puzzle 02: {}", puzzle_02(&input));
}

fn puzzle_01(input: &String) -> usize {
    let mut result = 0;

    for line in input.lines() {
        let mut list_of_numbers = line.split(":").nth(1).unwrap().split("|").take(2);
        let (winning_nums, my_nums) = (
            list_of_numbers.next().unwrap(),
            list_of_numbers.next().unwrap(),
        );

        let winning_nums = winning_nums
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut points = 0;
        for num in my_nums.split_whitespace() {
            if winning_nums.contains(&num.parse::<usize>().unwrap()) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        result += points;
    }

    return result;
}

fn puzzle_02(input: &String) -> usize {
    #[derive(Clone)]
    struct Card {
        number: u8,
        matching_nums: u8,
    }

    impl Card {
        fn parse(input: &str) -> Self {
            let mut parts = input.split(":");
            let number = parts
                .next()
                .unwrap()
                .replace("Card ", "")
                .trim()
                .parse::<u8>()
                .unwrap();

            let mut list_of_numbers = parts.next().unwrap().split("|").take(2);
            let (winning_nums, my_nums) = (
                list_of_numbers
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
                list_of_numbers
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap()),
            );

            return Self {
                number,
                matching_nums: my_nums.fold(0, |acc, num| {
                    if winning_nums.contains(&num) {
                        acc + 1
                    } else {
                        acc
                    }
                }),
            };
        }
    }

    let mut result = 0;
    let mut queue: VecDeque<Card> = VecDeque::new();
    let mut cards: Vec<Card> = Vec::new();

    // collect all cards into a queue first.
    for line in input.lines() {
        let card = Card::parse(line);

        cards.push(card.clone());
        queue.push_back(card);

        result += 1;
    }

    while !queue.is_empty() {
        let card = queue.pop_front().unwrap();

        for i in 0..card.matching_nums {
            queue.push_back(cards[(card.number as usize) + (i as usize)].clone());
        }

        result += card.matching_nums as usize;
    }

    return result;
}

mod tests {
    #[test]
    fn test_puzzle_01() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        assert_eq!(super::puzzle_01(&input.to_string()), 13);
    }

    #[test]
    fn test_puzzle_02() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        assert_eq!(super::puzzle_02(&input.to_string()), 30);
    }
}

// total = 6
// queue = [1,2,3,4,5,6]
// 1. queue = [2,3,4,5,6,2,3,4,5,6], total = 11
// 2. queue = [3,4,5,6,3,4], total = 13
// 3. queue = [4,5,6,3,4,4,5] total = 15
// 4. queue = [5,6,3,4,4,5, 5] total = 16
// 5. queue = [6,3,4,4,5,5] total = 16
// 6. queue = [3,4,4,5,5] total = 16
// 7. queue = [4,4,5,5,4,5] total = 18
// 8. queue = [4,5,5,4,5,5] total = 19
// 9. queue = [5,5,4,5,5,5] total = 20
// 10. queue = [5,4,5,5,5] total = 21
// 11. queue = [4,5,5,5] total = 22
// 11. queue = [5,5,5,5] total = 23
// 11. queue = [5,5,5,5] total = 23

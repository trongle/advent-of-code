use std::fs;

fn solve(lines: impl Iterator<Item = String>) -> u32 {
    return lines
        .map(|line| {
            let mut first_digit: Option<u8> = None;
            let mut last_digit: Option<u8> = None;

            // Iterate over each character in the line.
            // If the character is a digit, store it in
            // first_digit if it is empty, otherwise store
            // it in last_digit. So after the loop,
            // first_digit will contain the first digit
            // in the line and last_digit will contain
            // the last digit in the line.
            for x in line.chars() {
                if let Ok(digit) = x.to_string().parse::<u8>() {
                    if first_digit.is_none() {
                        first_digit = Some(digit);
                    } else {
                        last_digit = Some(digit);
                    }
                }
            }
            // Make sure we have a last digit
            // in case only one digit was found
            // in the line.
            if last_digit.is_none() {
                last_digit = first_digit;
            }

            format!("{}{}", first_digit.unwrap(), last_digit.unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>();
}

pub fn puzzle_01() {
    let result = solve(
        fs::read_to_string("inputs/day_01.txt")
            .expect("File day_01.txt not found!")
            .lines()
            .map(|line| line.to_string()),
    );

    println!("Day 01, Puzzle 01: {}", result);
}

pub fn puzzle_02() {
    let result = solve(
        fs::read_to_string("inputs/day_01.txt")
            .expect("File day_01.txt not found!")
            .lines()
            .map(|line| {
                // Replace the words with numbers. Notice
                // that there are overlapping words, so
                // we need to replace the words with the
                // first character and the last chatacter
                // of the word. For example:
                //
                // "2eightwo" => "2e8t2o"
                let new_line = line
                    .replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "f4r")
                    .replace("five", "f5e")
                    .replace("six", "s6x")
                    .replace("seven", "s7n")
                    .replace("eight", "e8t")
                    .replace("nine", "n9e");

                new_line
            }),
    );

    println!("Day 01, Puzzle 02: {}", result);
}

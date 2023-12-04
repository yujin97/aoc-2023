use std::collections::HashMap;
use std::fs;

fn main() {
    let document = fs::read_to_string("src/input/d1.txt").expect("Failed to read file");
    let mut calibration_values = Vec::new();
    let mut length_digit_map = HashMap::new();

    length_digit_map.insert(3, vec![("one", 1), ("two", 2), ("six", 6), ("ten", 10)]);
    length_digit_map.insert(4, vec![("four", 4), ("five", 5), ("nine", 9)]);
    length_digit_map.insert(5, vec![("three", 3), ("seven", 7), ("eight", 8)]);

    for line in document.lines() {
        let mut digits = Vec::new();
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap();
                digits.push(digit);
            } else {
                // do a 3..=5 loop
                for range in 2..=4 {
                    let end = i + range;
                    if end > line.len() - 1 {
                        break;
                    }
                    let guess = &line[i..=(i + range)];
                    let word_length = range + 1;
                    let possible_entries = length_digit_map.get(&word_length).unwrap();
                    for (word, length) in possible_entries {
                        if *word == guess {
                            digits.push(*length);
                            break;
                        }
                    }
                }
            }
        }
        let first = (digits
            .first()
            .expect("Failed to unwrap first value")
            .to_owned())
            * 10;
        let last = digits.last().expect("Failed to last value").to_owned();
        let calibration_value = first + last;
        calibration_values.push(calibration_value);
    }

    let sum = calibration_values
        .into_iter()
        .reduce(|acc, elem| acc + elem)
        .unwrap();

    println!("The sum of the calibration values is {sum}");
}

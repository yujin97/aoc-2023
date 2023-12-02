use std::fs;

fn main() {
    let document = fs::read_to_string("src/input/d1.txt").expect("Failed to read file");
    let mut calibration_values = Vec::new();

    for line in document.lines() {
        let mut digits = Vec::new();
        for digit in line.chars() {
            if digit.is_digit(10) {
                let digit = digit.to_digit(10).unwrap();
                digits.push(digit);
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

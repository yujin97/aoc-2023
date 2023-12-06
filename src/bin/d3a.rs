use std::collections::HashMap;
use std::fs;

struct Number {
    start: usize,
    end: usize,
    value: usize,
}

impl Number {
    fn parse(start: usize, end: usize, value: &str) -> Self {
        let value: usize = value.parse().unwrap();
        Number { start, end, value }
    }

    fn is_part_number(
        &self,
        previous_line: Option<&HashMap<usize, bool>>,
        next_line: Option<&HashMap<usize, bool>>,
        current_line: &HashMap<usize, bool>,
    ) -> bool {
        let is_first = self.start == 0;
        let is_last = self.end == 139;

        let mut current_line_check_point = Vec::new();
        let mut previous_line_check_point = Vec::new();
        let mut next_line_check_point = Vec::new();

        if !is_first {
            current_line_check_point.push(self.start - 1);
        }
        // based on the input file
        if !is_last {
            current_line_check_point.push(self.end + 1);
        }

        let check_start = if is_first { self.start } else { self.start - 1 };
        let check_end = if is_last { self.end } else { self.end + 1 };

        if previous_line.is_some() {
            for point in check_start..=check_end {
                previous_line_check_point.push(point);
            }
        }

        if next_line.is_some() {
            for point in check_start..=check_end {
                next_line_check_point.push(point);
            }
        }

        for point in current_line_check_point {
            if current_line.get(&point).is_some() {
                return true;
            }
        }

        if let Some(previous_line) = previous_line {
            for point in previous_line_check_point {
                if previous_line.get(&point).is_some() {
                    return true;
                }
            }
        }

        if let Some(next_line) = next_line {
            for point in next_line_check_point {
                if next_line.get(&point).is_some() {
                    return true;
                }
            }
        }

        false
    }
}

fn is_a_symbol(c: char) -> bool {
    if c.is_digit(10) {
        return false;
    }
    match c {
        '.' => false,
        _ => true,
    }
}

fn main() {
    let schematic = fs::read_to_string("src/input/d3.txt").expect("Failed to read file");

    let mut ans = 0;
    let rows: Vec<_> = schematic.lines().collect();
    let number_of_rows = rows.len();

    let mut numbers_by_rows = Vec::new();
    let mut symbols_by_rows = Vec::new();
    for row in rows {
        let mut numbers = Vec::new();
        let mut symbols = HashMap::new();
        let mut number_start = None;
        let mut number_end = None;
        for (i, c) in row.chars().enumerate() {
            if is_a_symbol(c) {
                symbols.insert(i, true);
            }
            if c.is_digit(10) {
                if number_start.is_none() {
                    number_start = Some(i);
                } else {
                    number_end = Some(i);
                }
                continue;
            }
            // it means it is a period
            if number_start.is_some() {
                let start = number_start.unwrap();
                let end = number_end.or_else(|| Some(start)).unwrap();
                let number = Number::parse(start, end, &row[start..=end]);
                numbers.push(number);
                number_start = None;
                number_end = None;
            }
        }
        if number_start.is_some() {
            let start = number_start.unwrap();
            let end = number_end.or_else(|| Some(start)).unwrap();
            let number = Number::parse(start, end, &row[start..=end]);
            numbers.push(number);
        }

        numbers_by_rows.push(numbers);
        symbols_by_rows.push(symbols);
    }

    for (i, numbers) in numbers_by_rows.iter().enumerate() {
        for number in numbers {
            let is_first = i == 0;
            let is_last = i == number_of_rows - 1;
            let previous_line = if is_first {
                None
            } else {
                Some(&symbols_by_rows[i - 1])
            };
            let next_line = if is_last {
                None
            } else {
                Some(&symbols_by_rows[i + 1])
            };
            if number.is_part_number(previous_line, next_line, &symbols_by_rows[i]) {
                ans += number.value;
            }
        }
    }
    println!("The answer is {ans}");
}

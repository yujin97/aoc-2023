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

fn find_gear_ratio(
    gear_x: usize,
    gear_y: usize,
    number_map: &Vec<Vec<(i32, usize)>>,
) -> Option<usize> {
    // assume width == height
    let max_index = number_map.len() - 1;
    let search_start_x = if gear_x > 0 { gear_x - 1 } else { gear_x };
    let search_end_x = if gear_x < max_index {
        gear_x + 1
    } else {
        gear_x
    };
    let search_start_y = if gear_y > 0 { gear_y - 1 } else { gear_y };
    let search_end_y = if gear_y < max_index {
        gear_y + 1
    } else {
        gear_y
    };

    let mut adjacent_part_number = HashMap::new();
    for y in search_start_y..=search_end_y {
        for x in search_start_x..=search_end_x {
            let (id, value) = number_map[y][x];
            if id > 0 {
                adjacent_part_number.insert(id, value);
            }
        }
    }

    if adjacent_part_number.len() == 2 {
        let ratio = adjacent_part_number
            .iter()
            .map(|(_, value)| *value)
            .reduce(|acc, e| acc * e)
            .unwrap();

        return Some(ratio);
    }

    None
}

fn main() {
    let schematic = fs::read_to_string("src/input/d3.txt").expect("Failed to read file");

    let mut ans = 0;
    let mut number_map = vec![vec![(-1, 0); 140]; 140];
    // -1 => symbol, 0 => gear (*), 1..=n => part name ID
    let mut part_id_count = 1;
    let rows: Vec<_> = schematic.lines().collect();
    let number_of_rows = rows.len();

    let mut numbers_by_rows = Vec::new();
    let mut symbols_by_rows = Vec::new();
    for (row_number, row) in rows.iter().enumerate() {
        let mut numbers = Vec::new();
        let mut symbols = HashMap::new();
        let mut number_start = None;
        let mut number_end = None;
        for (i, c) in row.chars().enumerate() {
            if is_a_symbol(c) {
                symbols.insert(i, true);
                if c == '*' {
                    number_map[row_number][i] = (0i32, 0);
                }
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
                for position in number.start..=number.end {
                    number_map[i][position] = (part_id_count, number.value);
                }
                part_id_count += 1;
            }
        }
    }

    for (y, rows) in number_map.iter().enumerate() {
        for (x, number) in rows.iter().enumerate() {
            if number.0 == 0 {
                let ratio = find_gear_ratio(x, y, &number_map);
                if let Some(ratio) = ratio {
                    ans += ratio;
                }
            }
        }
    }

    println!("The answer is {ans}");
}

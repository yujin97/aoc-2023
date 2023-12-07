use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<usize>,
    owned_numbers: Vec<usize>,
}

impl Card {
    fn parse(source: &str) -> Self {
        let source: String = source.split(':').skip(1).collect();
        let mut source_iter = source.trim().split('|');
        let winning_numbers: Vec<_> = source_iter
            .next()
            .unwrap()
            .split(' ')
            .filter(|number| *number != "")
            .map(|number| number.trim().parse::<usize>().unwrap())
            .collect();
        let owned_numbers: Vec<_> = source_iter
            .next()
            .unwrap()
            .split(' ')
            .filter(|number| *number != "")
            .map(|number| number.parse::<usize>().unwrap())
            .collect();
        Self {
            winning_numbers,
            owned_numbers,
        }
    }

    fn calculate_points(&self) -> usize {
        let mut counter = HashMap::new();

        for winning_number in &self.winning_numbers {
            counter.insert(winning_number, 0);
        }

        for owned_number in &self.owned_numbers {
            if counter.contains_key(&owned_number) {
                counter.insert(owned_number, 1);
            }
        }

        let matched_numbers: Vec<_> = counter.into_iter().filter(|number| number.1 > 0).collect();
        let number_of_matches: u32 = matched_numbers.len().try_into().unwrap();

        if number_of_matches > 0 {
            2usize.pow(number_of_matches - 1)
        } else {
            0
        }
    }
}

fn main() {
    let cards = fs::read_to_string("src/input/d4.txt").expect("Failed to read file");
    let mut ans = 0;

    for card in cards.lines() {
        let card = Card::parse(card);
        let points = card.calculate_points();
        ans += points;
    }

    println!("The answer is {ans}");
}

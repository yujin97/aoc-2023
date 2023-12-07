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

    fn calculate_new_card(&self) -> usize {
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
        let number_of_matches = matched_numbers.len();

        number_of_matches
    }
}

fn main() {
    let cards = fs::read_to_string("src/input/d4.txt").expect("Failed to read file");
    let mut original_cards = Vec::new();

    for card in cards.lines() {
        let card = Card::parse(card);
        original_cards.push(card);
    }

    let number_of_cards = original_cards.len();
    let mut calculated_values_of_card = vec![1; number_of_cards];

    let last_card_id = number_of_cards - 1;

    for idx in (0..=number_of_cards - 2).rev() {
        let spawn = original_cards[idx].calculate_new_card();
        if spawn > 0 {
            let first_spawn = idx + 1;
            let last_spawn = if idx + spawn > last_card_id {
                last_card_id
            } else {
                idx + spawn
            };
            for spawn_idx in first_spawn..=last_spawn {
                calculated_values_of_card[idx] += calculated_values_of_card[spawn_idx];
            }
        }
    }

    let ans: i32 = calculated_values_of_card.iter().sum();

    println!("The answer is {ans}");
}

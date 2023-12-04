use std::collections::HashMap;
use std::fs;

fn main() {
    let games = fs::read_to_string("src/input/d2.txt").expect("Failed to read file");

    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let mut ans = 0;

    for (game_number, game) in games.lines().enumerate() {
        let game_number = game_number + 1;
        let mut record = HashMap::new();
        record.insert("blue", 0);
        record.insert("green", 0);
        record.insert("red", 0);
        let grabs = game.split(':').skip(1).collect::<String>();
        for grab in grabs.split(';') {
            let kinds: Vec<_> = grab
                .split(',')
                .map(|kind| {
                    let kind = kind.trim().split(' ').collect::<Vec<_>>();
                    let number = kind[0].parse::<u32>().unwrap();
                    (number, kind[1])
                })
                .collect();

            for kind in kinds {
                if kind.0 > *record.get(kind.1).unwrap() {
                    record.insert(kind.1, kind.0);
                }
            }
        }

        let red_number = record.get("red").unwrap();
        let green_number = record.get("green").unwrap();
        let blue_number = record.get("blue").unwrap();

        let is_possible =
            &red_limit >= red_number && &green_limit >= green_number && &blue_limit >= blue_number;

        if is_possible {
            ans += game_number;
        }
    }

    println!("The answer is: {ans}");
}

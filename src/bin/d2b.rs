use std::collections::HashMap;
use std::fs;

fn main() {
    let games = fs::read_to_string("src/input/d2.txt").expect("Failed to read file");

    let mut ans = 0;

    for game in games.lines() {
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

        let power = red_number * green_number * blue_number;

        ans += power;
    }

    println!("The answer is: {ans}");
}

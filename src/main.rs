use itertools::Itertools;
use rustyline::Editor;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    // let one_suit: Vec<char> = "AKQJT987654321".chars().collect();
    // let honors: Vec<char> = "AKQJT".chars().collect();
    let pointed_honors: Vec<char> = "AKQJ".chars().collect();
    // let deck: Vec<char> = one_suit.iter().cycle().take(one_suit.len() * 4).map(|&x| x).collect();
    let mut combinations:Vec<String> = Vec::new();
    for n in 1..pointed_honors.len() {
        combinations.extend(
            pointed_honors.clone().into_iter()
            .combinations(n)
            .map(|c| c.iter().collect::<String>())
            );
    }
    
    println!("{:?}", combinations);

    let mut rng = thread_rng();

    let mut total_asked = 0;
    let mut correct = 0;
    let mut wrong = 0;
    let now = Instant::now();
    loop {
        combinations.shuffle(&mut rng);
        for c in combinations {
            println!("{}", c);
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    // rl.add_history_entry(line.as_str());
                    match line.parse::<u8>() {
                        Ok(a) => {
                            if a == hcp(c) {
                                println!("Yay!");
                                correct += 1;
                            } else {
                                println!("nope");
                                wrong += 1;
                            }
                        },
                        Err(_) => {
                            println!("not even a number so... No");
                        }
                    }
                }, 
                Err(err) => {
                    println!("Error: {:?}", err);
                    println!("number done: {}, Number correct: {}, Number wrong: {}, Time: {}",
                             total_asked,
                             correct,
                             wrong,
                             now.elapsed().as_secs());
                    break
                }
            }
            total_asked += 1;
        }
    }
}

fn hcp<T: AsRef<str>>(cards: T) -> u8 {
    cards.as_ref().chars().map(|c| {
        match c {
            'A' => 4,
            'K' => 3,
            'Q' => 2,
            'J' => 1,
            _   => 0
        }
    })
    .sum()
}



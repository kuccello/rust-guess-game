use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::fs;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct LeaderboardRecord {
    name: String,
    guesses: u8
}

#[derive(Serialize, Deserialize, Debug)]
struct Leaderboard {
    first: LeaderboardRecord,
    second: LeaderboardRecord,
    third: LeaderboardRecord
}

impl Leaderboard {
    fn calculate_slot(&self, val: u8) -> u8 {
        match val.cmp(&self.third.guesses) {
            Ordering::Less => match val.cmp(&self.second.guesses) {
                Ordering::Less => match val.cmp(&self.first.guesses) {
                    Ordering::Less => 1,
                    Ordering::Equal => 1,
                    Ordering::Greater => 2
                },
                Ordering::Equal => 2,
                Ordering::Greater => 3
            },
            Ordering::Greater => 0,
            Ordering::Equal => 3
        }
    }

    fn write(&self) -> Leaderboard {
        let serialized = serde_json::to_string(&self).expect("Could not convert to string");
        fs::write("./leaderboard.json", serialized).expect("Issue writing leaderboard file");
        let data = fs::read_to_string("./leaderboard.json").unwrap();
        serde_json::from_str(&data).expect("Could not convert from string")
    }

    fn update(&self, name: String, guesses: u8) {
        let slot = &self.calculate_slot(guesses);
        match slot {
            1 => (Leaderboard {
                first: LeaderboardRecord {name, guesses},
                second: LeaderboardRecord {name: self.first.name.to_string(), guesses: self.first.guesses},
                third: LeaderboardRecord {name: self.second.name.to_string(), guesses: self.second.guesses}
            }).write(),
            2 => (Leaderboard {
                first: LeaderboardRecord {name: self.first.name.to_string(), guesses: self.first.guesses},
                second: LeaderboardRecord {name, guesses},
                third: LeaderboardRecord {name: self.second.name.to_string(), guesses: self.second.guesses}
            }).write(),
            3 => (Leaderboard {
                first: LeaderboardRecord {name: self.first.name.to_string(), guesses: self.first.guesses},
                second: LeaderboardRecord {name: self.second.name.to_string(), guesses: self.second.guesses},
                third: LeaderboardRecord {name, guesses}
            }).write(),
            _ => {
                println!("Was slot value {:?}", slot);
                self.write()
            }
        };
    }
}

fn load_leaderboard() -> Leaderboard {
    let leaderboard = fs::read_to_string("./leaderboard.json");
    match leaderboard {
        Ok(data) => serde_json::from_str(&data).expect("Could not convert from string"),
        Err(e) => {
            println!("Failed to find leaderboard file! {}", e);
            let leaderboard = Leaderboard {
                first: LeaderboardRecord {name:"NONE".to_string(), guesses:255},
                second: LeaderboardRecord {name:"NONE".to_string(), guesses:255},
                third: LeaderboardRecord {name:"NONE".to_string(), guesses:255}
            };
            leaderboard.write()
        }
    }
}

fn main() {
    let leaderboard = load_leaderboard();

    println!("==============[ Leader Board ]=========================");
    println!("\t First Place: \t{:?}, with {} guesses", leaderboard.first.name, leaderboard.first.guesses);
    println!("\tSecond Place: \t{:?}, with {} guesses", leaderboard.second.name, leaderboard.second.guesses);
    println!("\t Third Place: \t{:?}, with {} guesses", leaderboard.third.name, leaderboard.third.guesses);
    println!("=======================================================");
    println!("Guess the number!");

    let mut tries: u8 = 0;

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("What is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name)
        .expect("Failed to read line");
    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        tries = tries + 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won!");
                println!("You used {} tries.", tries);
                println!("Your slot is {}", leaderboard.calculate_slot(tries));
                leaderboard.update(name.trim().to_string(), tries);
                break;
            }
        }
    }
}

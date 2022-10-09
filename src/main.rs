mod enums;
mod helpers;

use colored::Colorize;
use rand::{self, Rng};
use std::io::Write;
use std::process::exit;

use crate::helpers::get_choice::choice;
use crate::helpers::get_input::input;
use crate::helpers::get_winner::winner;

fn main() {
    loop {
        let header: colored::ColoredString = "
    (1): Rock
    (2): Paper
    (3): Scissor
    Any other to exit
    "
        .black();
        let user_choice = loop {
            println!("{}", header);
            let s = input("Please choose");
            match choice(&s) {
                Some(choice) => break choice,
                None => {println!("No choice made, exiting"); exit(0)},
            };
        };
        let x = rand::thread_rng().gen_range(1..4);
        let computer_choice = choice(x.to_string().as_str()).unwrap();
        for v in "Loading...".as_bytes().iter() {
            std::thread::sleep(std::time::Duration::from_millis(50));
            print!("{}", *v as char);
            std::io::stdout().flush().unwrap();
        }
        let s1 = "\nYou choose:".cyan();
        let s2 = "I choose:".blue();
        println!("{} {:?}", s1, user_choice.clone());
        println!("{} {:?}", s2, computer_choice.clone());
        match winner(user_choice, computer_choice) {
            Some(true) => println!("{}", "You win !".green()),
            Some(false) => println!("{}", "You lose !".red()),
            None => println!("{}", "Draw !".yellow()),
        };
    }
}


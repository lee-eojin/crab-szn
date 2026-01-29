use crate::constants::{MSG_ANSWER, MSG_GUESS};

pub fn print_answer(answer: &Vec<u8>) {
    println!("{}{:?}", MSG_ANSWER, answer);
}

pub fn print_guess(guess: &Vec<u8>) {
    println!("{}{:?}", MSG_GUESS, guess);
}

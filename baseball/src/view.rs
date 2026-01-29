use crate::constants::{MSG_ANSWER, MSG_GUESS, MSG_STRIKE, MSG_BALL, MSG_OUT, MSG_WIN, DIGIT_COUNT};
use crate::game::Score;

pub fn print_answer(answer: &Vec<u8>) {
    println!("{}{:?}", MSG_ANSWER, answer);
}

pub fn print_guess(guess: &Vec<u8>) {
    println!("{}{:?}", MSG_GUESS, guess);
}

pub fn print_score(score: &Score) {
    if score.strike == 0 && score.ball == 0 {
        println!("{}", MSG_OUT);
    } else {
        let mut result = String::new();
        if score.strike > 0 {
            result.push_str(&format!("{}{} ", score.strike, MSG_STRIKE));
        }
        if score.ball > 0 {
            result.push_str(&format!("{}{}", score.ball, MSG_BALL));
        }
        println!("{}", result.trim());
    }
}

pub fn print_win(attempts: u32) {
    println!("{} ({}번 만에 맞춤)", MSG_WIN, attempts);
}

use rand::seq::SliceRandom;
use std::io::{self, Write};

use crate::constants::{DIGIT_COUNT, MIN_NUM, MAX_NUM, MSG_INPUT_PROMPT};

pub struct Game {
    answer: Vec<u8>,
    attempts: u32,
}

pub struct Score {
    pub strike: u8,
    pub ball: u8,
}

impl Game {
    pub fn new() -> Self {
        let mut nums: Vec<u8> = (MIN_NUM..=MAX_NUM).collect();
        nums.shuffle(&mut rand::thread_rng());

        Game {
            answer: nums[0..DIGIT_COUNT].to_vec(),
            attempts: 0,
        }
    }

    pub fn get_input(&self) -> Vec<u8> {
        print!("{}", MSG_INPUT_PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input
            .trim()
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect()
    }

    pub fn answer(&self) -> &Vec<u8> {
        &self.answer
    }

    pub fn judge(&mut self, guess: &Vec<u8>) -> Score {
        self.attempts += 1;

        let mut strike = 0;
        let mut ball = 0;

        for (i, g) in guess.iter().enumerate() {
            if self.answer[i] == *g {
                strike += 1;
            } else if self.answer.contains(g) {
                ball += 1;
            }
        }

        Score { strike, ball }
    }

    pub fn attempts(&self) -> u32 {
        self.attempts
    }
}

impl Score {
    pub fn is_win(&self) -> bool {
        self.strike == DIGIT_COUNT as u8
    }
}

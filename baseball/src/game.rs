use rand::seq::SliceRandom;
use std::io::{self, Write};

pub struct Game {
    answer: Vec<u8>,
    attempts: u32,
}

impl Game {
    pub fn new() -> Self {
        let mut nums: Vec<u8> = (1..=9).collect();
        nums.shuffle(&mut rand::thread_rng());

        Game {
            answer: nums[0..3].to_vec(),
            attempts: 0,
        }
    }

    pub fn get_input(&self) -> Vec<u8> {
        print!("숫자를 입력하세요: ");
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
}

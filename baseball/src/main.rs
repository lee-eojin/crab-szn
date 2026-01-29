use rand::seq::SliceRandom;

fn generate_number() -> Vec<u8> {
    let mut nums: Vec<u8> = (1..=9).collect();
    nums.shuffle(&mut rand::thread_rng());
    nums[0..3].to_vec()
}

fn main() {
    let answer = generate_number();
    println!("정답: {:?}", answer);
}

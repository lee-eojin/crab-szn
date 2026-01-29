mod game;

use game::Game;

fn main() {
    let game = Game::new();
    println!("정답: {:?}", game.answer());

    let guess = game.get_input();
    println!("입력: {:?}", guess);
}

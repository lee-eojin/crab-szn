mod constants;
mod game;
mod view;

use game::Game;

fn main() {
    let game = Game::new();
    view::print_answer(game.answer());

    let guess = game.get_input();
    view::print_guess(&guess);
}

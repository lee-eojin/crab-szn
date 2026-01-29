mod constants;
mod game;
mod view;

use game::Game;

fn main() {
    let mut game = Game::new();

    loop {
        let guess = game.get_input();
        let score = game.judge(&guess);

        view::print_score(&score);

        if score.is_win() {
            view::print_win(game.attempts());
            break;
        }
    }
}

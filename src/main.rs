use solver_2048::game;

fn main() {
    let mut game = game::Game::new();
    println!("{game}");

    game.board.shift_down();

    println!("{game}");
}

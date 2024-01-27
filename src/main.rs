use std::time;

use game::Game;
use renderer::Renderer;

mod game;
mod renderer;

fn main() {
    let mut game = Game::new_random(150, 50, true);
    let renderer = Renderer::new('\u{25A0}', '\u{25A1}', game.board());

    std::process::Command::new("clear").spawn().unwrap();

    loop {
        renderer.render(game.board());

        game.step();

        std::thread::sleep(time::Duration::from_millis(50));
        renderer.cursor_up();
    }
}

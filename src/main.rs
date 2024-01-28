use std::time;

use clap::Arg;
use clap::Parser;
use game::Game;
use renderer::Renderer;

mod game;
mod renderer;

#[derive(Parser)]
#[command(author, version, about, long_about = None, disable_help_flag = true)]
struct Arguments {
    #[arg(short, long, default_value_t = 30)]
    width: usize,

    #[arg(short, long, default_value_t = 10)]
    height: usize,

    #[arg(long, action=clap::ArgAction::SetTrue)]
    no_torus: bool,

    #[arg(short, long, default_value_t = 100)]
    duration: u64,
}

fn main() {
    let args = Arguments::parse();

    let mut game = Game::new_random(args.width, args.height, !args.no_torus);
    let renderer = Renderer::new('\u{25A0}', '\u{25A1}', game.board());

    std::process::Command::new("clear").spawn().unwrap();

    loop {
        renderer.render(game.board());
        
        game.step();

        std::thread::sleep(time::Duration::from_millis(args.duration));
        renderer.clear();
    }
}

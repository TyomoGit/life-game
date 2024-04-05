use std::time;
use getch::Getch;

use game::Game;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use renderer::Renderer;

mod game;
mod renderer;

// #[derive(Parser)]
// #[clap(disable_help_flag = true)]
// #[command(author, version, about, long_about = None)]
// struct Arguments {
//     #[arg(short, long, default_value_t = 30)]
//     width: usize,

//     #[arg(short, long, default_value_t = 10)]
//     height: usize,

//     #[arg(long, action=clap::ArgAction::SetTrue)]
//     no_torus: bool,

//     #[arg(short, long, default_value_t = 100)]
//     duration: u64,

//     #[arg(long, action=clap::ArgAction::Help, )]
//     help: Option<bool>,
// }

fn main() {
    // let args = Arguments::parse();


    // let renderer = Renderer::new('\u{25A0}', '\u{25A1}', game.board());

    let mut queue: Vec<Game> = Vec::new();

    for _ in 0..10usize.pow(3) {
        queue.push(Game::new_random(40, 40, true));
    }

    queue.par_iter_mut()
        .for_each(|game| {
            game.step_until_dead();
        });

    let max_game = queue.iter()
        .max_by(|a, b| a.epochs().cmp(&b.epochs()))
        .unwrap();

    // let min_game = queue.iter()
    //     .min_by(|a, b| a.epochs().cmp(&b.epochs()))
    //     .unwrap();

    let renderer = Renderer::new("\u{25A0} ", "  ", max_game.board());
    renderer.render(max_game.board());
    renderer.render(max_game.init_board.board());
    println!("epochs: {}", max_game.epochs());

    // Getch::new()
    //     .getch()
    //     .expect("Failed to get input");

    // let mut game = Game::new(min_game.init_board.clone(), true);
    // for _ in 0..min_game.epochs() {
    //     renderer.render(game.board());
    //     println!("{} / {}", game.epochs(), max_game.epochs());

    //     game.step();
    //     std::thread::sleep(time::Duration::from_millis(100));
    //     renderer.clear();
    // }

    Getch::new()
        .getch()
        .expect("Failed to get input");

    let mut game = Game::new(max_game.init_board.clone(), true);
    loop {

        renderer.render(game.board());
        println!("{} / {}", game.epochs(), max_game.epochs());

        game.step();
        std::thread::sleep(time::Duration::from_millis(100));
        renderer.clear();
    }
}

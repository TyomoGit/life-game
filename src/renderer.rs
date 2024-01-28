const HORIZONTAL: char = '\u{2500}';
const VERTICAL: char = '\u{2502}';
const TOP_RIGHT: char = '\u{2510}';
const TOP_LEFT: char = '\u{250c}';
const BOTTOM_RIGHT: char = '\u{2518}';
const BOTTOM_LEFT: char = '\u{2514}';

pub struct Renderer {
    live_icon: char,
    dead_icon: char,

    screen_height: usize,
}

impl Renderer {
    pub fn new(live_icon: char, dead_icon: char, board: &[Vec<bool>]) -> Self {
        Self {
            live_icon,
            dead_icon,
            screen_height: board.len() + 2,
        }
    }

    pub fn render(&self, board: &[Vec<bool>]) {
        let width = board.first().unwrap().len();
        let height = board.len();

        for y in 0..height + 2 {
            for x in 0..width + 2 {
                let cell = match (y, x) {
                    (0, 0) => TOP_LEFT,
                    (0, x) if x == width + 1 => TOP_RIGHT,
                    (y, 0) if y == height + 1 => BOTTOM_LEFT,
                    (y, x) if x == width + 1 && y == height + 1 => BOTTOM_RIGHT,

                    (0, _) => HORIZONTAL,
                    (y, _) if y == height + 1 => HORIZONTAL,
                    (_, 0) => VERTICAL,
                    (_, x) if x == width + 1 => VERTICAL,

                    _ => {
                        if board[y - 1][x - 1] {
                            self.live_icon
                        } else {
                            self.dead_icon
                        }
                    }
                };

                print!("{} ", cell);
            }
            println!();
        }
    }

    pub fn clear(&self) {
        print!("{}[2J", 27 as char);
    }
}

use rand::{random, thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Board {
    board: Vec<Vec<bool>>,
}

impl Board {
    pub fn new(board: Vec<Vec<bool>>) -> Self {
        Self { board: board }
    }

    pub fn height(&self) -> usize {
        self.board.len()
    }

    pub fn width(&self) -> usize {
        self.board.first().unwrap().len()
    }

    pub fn board(&self) -> &Vec<Vec<bool>> {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut Vec<Vec<bool>> {
        &mut self.board
    }

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.board.get(y)?.get(x).cloned()
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.board[y][x] = value;
    }
}

pub struct Game {
    board: Board,
    buffer: Board,
    is_torus: bool,
    width: usize,
    height: usize,
}

impl Game {
    pub fn new(width: usize, height: usize, is_torus: bool) -> Self {
        let board = Board::new(vec![vec![false; width]; height]);
        Self {
            board: board.clone(),
            buffer: board,
            is_torus,
            width,
            height,
        }
    }

    pub fn new_random(width: usize, height: usize, is_torus: bool) -> Self {
        let mut board = Board::new(vec![Vec::with_capacity(width); height]);
        let mut rng = thread_rng();

        for row in board.board_mut() {
            for _ in 0..width {
                row.push(rng.gen_bool(0.5));
            }
        }

        Self {
            board: board.clone(),
            buffer: board,
            is_torus,
            width,
            height,
        }
    }

    pub fn height(&self) -> usize {
        self.board.height()
    }

    pub fn width(&self) -> usize {
        self.board.width()
    }

    pub fn board(&self) -> &Vec<Vec<bool>> {
        self.board.board()
    }

    pub fn step(&mut self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let neighbor_count = self.count_neighbors(x, y);

                if self.board.get(x, y).unwrap() {
                    if !(neighbor_count == 2 || neighbor_count == 3) {
                        self.buffer.set(x, y, false);
                    }
                } else if neighbor_count == 3 {
                    self.buffer.set(x, y, true);
                }
            }
        }
        self.board = self.buffer.clone();
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        const DIRECTIONS: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut count: usize = 0;

        for direction in DIRECTIONS {
            let x = x as i32 + direction.0;
            let y = y as i32 + direction.1;

            if self.check_within_range(x, y) {
                if self.board.get(x as usize, y as usize).unwrap() {
                    count += 1;
                }
            } else if self.is_torus {
                let x = x as usize % self.width();
                let y = y as usize % self.height();

                if self.board.get(x, y).unwrap() {
                    count += 1;
                }
            }
        }

        count
    }

    fn check_within_range(&self, x: i32, y: i32) -> bool {
        0 <= x && (x as usize) < self.width() && 0 <= y && (y as usize) < self.height()
    }
}


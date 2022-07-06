use crate::board::{Board, Owner, Move, Direction};
use rand::Rng;
use rand::rngs::ThreadRng;

pub trait Player {
    fn make_move(&mut self, board: &Board, owner: Owner) -> Move;
}

pub struct RandomPlayer {
    rng: ThreadRng
}

impl RandomPlayer {
    pub fn new() -> RandomPlayer {
        RandomPlayer {
            rng: ThreadRng::default()
        }
    }
}

impl Player for RandomPlayer {
    fn make_move(&mut self, board: &Board, owner: Owner) -> Move {
        loop {
            let index_x = self.rng.gen_range(0..board.size_x);
            let index_y = self.rng.gen_range(0..board.size_y);
            let random_direction: usize = self.rng.gen_range(0..4);
            let potential_move = Move {
                x: index_x,
                y: index_y,
                owner: owner,
                direction: Direction::index(random_direction)
            };
            if board.check_if_free(&potential_move) {
                return potential_move;
            }
        }
    }
}
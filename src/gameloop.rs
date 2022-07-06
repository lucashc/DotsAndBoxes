use crate::players::Player;
use crate::board::{Board, Owner};

pub fn run_game(player1: &mut dyn Player, player2: &mut dyn Player, board: &mut Board){
    let mut owner = Owner::Player1;
    let mut i = 0;
    while board.filled < board.size_x*board.size_y {
        let player_move = if i == 0 {
            player1.make_move(board, owner)
        } else {
            player2.make_move(board, owner)
        };
        board.make_move(player_move);
        println!("{}", board);
        owner.tick();
        i = (i + 1) % 2;
    }
}
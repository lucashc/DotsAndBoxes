use crate::players::Player;
use crate::board::{Board, Owner};

pub enum Winner {
    Player1,
    Player2,
    Tie
}

pub fn run_game(player1: &mut dyn Player, player2: &mut dyn Player, board: &mut Board) -> Winner {
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
    let counts = board.count_owners();
    if counts[0] == counts[1] {
        return Winner::Tie;
    } else if counts[0] > counts[1] {
        return Winner::Player1;
    } else {
        return Winner::Player2;
    }
}
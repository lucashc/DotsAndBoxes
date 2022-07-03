mod board;
use board::{Board, Player, Direction};


fn main() {
    let mut board = Board::new(10, 10);
    board.make_move(Player::Player1, 0, 0, Direction::North);
    board.make_move(Player::Player1, 0, 0, Direction::West);
    board.make_move(Player::Player1, 0, 0, Direction::East);
    board.make_move(Player::Player2, 0, 0, Direction::South);
    board.make_move(Player::Player1, 4, 4, Direction::South);
    println!("{:?}", board[4][4]);
    println!("{:?}", board[5][4]);
    board.check_consistency();
    println!("My Board:\n{}", board);
}

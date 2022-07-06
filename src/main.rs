mod board;
mod players;
mod gameloop;
use players::RandomPlayer;
use board::Board;
use gameloop::run_game;


fn main() {
    let mut board = Board::new(10, 10);
    let mut random1 = RandomPlayer::new();
    let mut random2 = RandomPlayer::new();
    run_game(&mut random1, &mut random2, &mut board);
    board.check_consistency();
    println!("My Board:\n{}", board);
}

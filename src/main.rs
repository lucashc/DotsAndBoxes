mod board;
mod players;
mod gameloop;
use players::RandomPlayer;
use board::Board;
use gameloop::run_game;


fn main() {
    let mut board = Board::new(5, 5);
    let mut random1 = RandomPlayer::new();
    let mut random2 = RandomPlayer::new();
    run_game(&mut random1, &mut random2, &mut board);
    board.check_consistency();
    println!("Counts: {}, {}", board.count_owners()[0], board.count_owners()[1]);
}

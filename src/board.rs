use std::fmt;
use std::ops::{Index, IndexMut, Neg};
use self::Direction::*;

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Player {
    Empty,
    Player1,
    Player2
}

impl Default for Player {
    fn default() -> Self { Player::Empty }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    North, 
    South,
    East,
    West,
}

impl Direction {
    fn iterator() -> impl Iterator<Item = Direction> {
        [North, South, East, West].iter().copied()
    }
}

impl Neg for Direction {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            North => Direction::South,
            South => Direction::North,
            East => Direction::West,
            West => Direction::East
        }
    }
}


#[derive(Clone)]
pub struct Board {
    size_x: usize,
    size_y: usize,
    // First index is row, second is column
    // So first by y, then by x
    board: Vec<Vec<Tile>>
}

#[derive(Clone, Default, Debug)]
pub struct Tile {
    owner: Player,
    north: bool,
    south: bool,
    west: bool,
    east: bool
}

impl Index<Direction> for Tile {
    type Output = bool;
    fn index(&self, index: Direction) -> &Self::Output{
        match index {
            North => &self.north,
            South => &self.south,
            West => &self.west,
            East => &self.east
        }
    }
}

impl IndexMut<Direction> for Tile {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        match index {
            North => &mut self.north,
            South => &mut self.south,
            West => &mut self.west,
            East => &mut self.east
        }
    }
}


impl Tile {
    fn resolve(&mut self, player: Player) -> bool{
        if self.owner == Player::Empty {
            if self.north && self.south && self.west && self.east {
                self.owner = player;
                return true;
            } else {
                return false;
            }
        } else {
            return true;
        }
    }
}

impl Board {
    pub fn new(size_x: usize, size_y: usize) -> Board {
        Board {
            size_x: size_x,
            size_y: size_y,
            board: vec![vec![Tile::default(); size_x]; size_y]
        }
    }

    pub fn adjacent_line(&self, index_x: usize, index_y: usize, direction: Direction) -> Option<bool> {
        match direction {
            North => {
                if index_y > 0 {
                    Some(self[((index_y as isize)-1) as usize][index_x][-direction])
                } else {
                    None
                }
            },
            South => {
                if index_y+1 < self.size_y {
                    Some(self[index_y+1][index_x][-direction])
                } else {
                    None
                }
            },
            East => {
                if index_x+1 < self.size_x {
                    Some(self[index_y][index_x+1][-direction])
                } else {
                    None
                }
            },
            West => {
                if index_x > 0 {
                    Some(self[index_y][((index_x as isize)-1) as usize][-direction])
                } else {
                    None
                }
            }
        }
    }

    pub fn make_move(&mut self, player: Player, index_x: usize, index_y: usize, direction: Direction) {
        let lines_to_be_drawn = match direction {
            Direction::North => {
                if index_y > 0 {
                    [(Some((index_x, index_y)), Direction::North),
                    (Some((index_x, index_y-1)), Direction::South)]
                } else {
                    [(Some((index_x, index_y)), Direction::North),
                    (None, Direction::South)]
                }
            },
            Direction::South => {
                if index_y < self.size_y-1 {
                    [(Some((index_x, index_y)), Direction::South),
                    (Some((index_x, index_y+1)), Direction::North)]
                } else {
                    [(Some((index_x, index_y)), Direction::South),
                    (None, Direction::North)]
                }
            },
            Direction::East => {
                if index_x < self.size_x-1 {
                    [(Some((index_x, index_y)), Direction::East),
                    (Some((index_x+1, index_y)), Direction::West)]
                } else {
                    [(Some((index_x, index_y)), Direction::East),
                    (None, Direction::West)]
                }
            },
            Direction::West => {
                if index_y > 0 {
                    [(Some((index_x, index_y)), Direction::West),
                    (Some((index_x, index_y-1)), Direction::East)]
                } else {
                    [(Some((index_x, index_y)), Direction::West),
                    (None, Direction::East)]
                }
            }
        };
        for i in 0..2 {
            let line = &lines_to_be_drawn[i];
            match line.0 {
                Some((index_x, index_y)) => {
                    self[index_y][index_x][line.1] = true;
                    self[index_y][index_x].resolve(player);
                }
                _ => {}
            };
        }
    }

    pub fn check_consistency(&self) {
        for i in 0..self.size_y {
            for j in 0..self.size_x {
                for direction in Direction::iterator() {
                    if self[i][j][direction] {
                        if let Some(drawn) = self.adjacent_line(j, i, direction) {
                            if !drawn {
                                panic!("No Draw: {}, {}, {:?} missing", i, j, direction);
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Index<usize> for Board {
    type Output = Vec<Tile>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.board[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.board[index]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Board[{}, {}, \n", self.size_x, self.size_y)?;
        let mut top = vec![false; self.size_x];
        let mut bottom = vec![false; self.size_x];

        fn print_line(f: &mut fmt::Formatter, line: &Vec<bool>) -> fmt::Result{
            for j in 0..line.len() {
                if line[j] {
                    write!(f, " ―")?;
                }else {
                    write!(f, "  ")?;
                }
            }
            Ok(())
        }

        for i in 0..self.size_y {
            for j in 0..self.size_x {
                top[j] = self[i][j].north;
                bottom[j] = self[i][j].south;
            }
            print_line(f, &top)?;
            write!(f, "\n")?;
            for j in 0..self.size_x {
                if self[i][j].west {
                    write!(f, "|")?;
                } else {
                    write!(f, " ")?;
                }
                match self[i][j].owner {
                    Player::Empty => write!(f, "○")?,
                    Player::Player1 => write!(f, "█")?,
                    Player::Player2 => write!(f, "▓")?
                }
                if j == self.size_x-1 {
                    if self[i][j].east {
                        write!(f, "-")?;
                    } else {
                        write!(f, " ")?;
                    }
                }
            }
            write!(f, "\n")?;
            if i == self.size_y - 1 {
                print_line(f, &bottom)?;
                write!(f, "\n")?;
            }
        }
        write!(f, "]")
    }
}
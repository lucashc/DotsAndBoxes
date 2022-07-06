use std::fmt;
use std::ops::{Index, IndexMut, Neg};
use self::Direction::*;

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Owner {
    Empty,
    Player1,
    Player2
}

impl Default for Owner {
    fn default() -> Self { Owner::Empty }
}

impl Owner {
    pub fn tick(&mut self) {
        match self {
            Owner::Player1 => *self = Owner::Player2,
            Owner::Player2 => *self = Owner::Player1,
            Owner::Empty => *self = Owner::Empty
        };
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    North, 
    South,
    East,
    West,
}

impl Direction {
    pub fn iterator() -> impl Iterator<Item = Direction> {
        [North, South, East, West].iter().copied()
    }
    pub fn index(i: usize) -> Direction {
        match i {
            0 => North,
            1 => South,
            2 => East,
            3 => West,
            _ => North
        }
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

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Move {
    pub x: usize,
    pub y: usize, 
    pub direction: Direction,
    pub owner: Owner
}


#[derive(Clone)]
pub struct Board {
    pub size_x: usize,
    pub size_y: usize,
    // First index is row, second is column
    // So first by y, then by x
    pub board: Vec<Vec<Tile>>,

    pub filled: usize
}

#[derive(Clone, Default, Debug)]
pub struct Tile {
    pub owner: Owner,
    pub north: bool,
    pub south: bool,
    pub west: bool,
    pub east: bool
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
    fn resolve(&mut self, player: Owner) -> bool{
        if self.owner == Owner::Empty {
            if self.north && self.south && self.west && self.east {
                self.owner = player;
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

impl Board {
    pub fn new(size_x: usize, size_y: usize) -> Board {
        Board {
            size_x: size_x,
            size_y: size_y,
            board: vec![vec![Tile::default(); size_x]; size_y],
            filled: 0
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

    pub fn make_move(&mut self, m: Move) {
        let index_x = m.x;
        let index_y = m.y;
        let lines_to_be_drawn = match m.direction {
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
                    if self[index_y][index_x].resolve(m.owner) {
                        self.filled += 1;
                    };
                }
                _ => {}
            };
        }
    }

    pub fn check_if_free(&self, potential_move: &Move) -> bool {
        !self[potential_move.y][potential_move.x][potential_move.direction]
    }

    pub fn count_owners(&self) -> [u64; 2] {
        let mut player1 = 0u64;
        let mut player2 = 0u64;
        for i in 0..self.size_y {
            for j in 0..self.size_x {
                match self[i][j].owner {
                    Owner::Player1 => player1 += 1,
                    Owner::Player2 => player2 += 1,
                    Owner::Empty => continue
                };
            }
        }
        return [player1, player2];
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
                    Owner::Empty => write!(f, "○")?,
                    Owner::Player1 => write!(f, "█")?,
                    Owner::Player2 => write!(f, "▓")?
                }
                if j == self.size_x-1 {
                    if self[i][j].east {
                        write!(f, "|")?;
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
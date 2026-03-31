use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Row {
    One,
    Two,
    Three,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Col {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    pub row: Row,
    pub col: Col,
}

#[derive(Debug)]
pub struct ParseCoordinateError;

impl fmt::Display for ParseCoordinateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid coordinate. Use format like A1, B2, C3 (column A-C, row 1-3)"
        )
    }
}

impl FromStr for Coordinate {
    type Err = ParseCoordinateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_uppercase();
        if s.len() != 2 {
            return Err(ParseCoordinateError);
        }
        let mut chars = s.chars();
        let col_char = chars.next().ok_or(ParseCoordinateError)?;
        let row_char = chars.next().ok_or(ParseCoordinateError)?;

        let col = match col_char {
            'A' => Col::A,
            'B' => Col::B,
            'C' => Col::C,
            _ => return Err(ParseCoordinateError),
        };

        let row = match row_char {
            '1' => Row::One,
            '2' => Row::Two,
            '3' => Row::Three,
            _ => return Err(ParseCoordinateError),
        };

        Ok(Coordinate { row, col })
    }
}

#[derive(Debug)]
pub enum MoveError {
    AlreadyOccupied,
    AlreadyWinner,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveError::AlreadyOccupied => write!(f, "That field is already occupied"),
            MoveError::AlreadyWinner => write!(f, "The game is already over"),
        }
    }
}

// Board layout: index = row * 3 + col
// Row::One=0, Row::Two=1, Row::Three=2
// Col::A=0, Col::B=1, Col::C=2
fn coord_index(coord: Coordinate) -> usize {
    let row = match coord.row {
        Row::One => 0,
        Row::Two => 1,
        Row::Three => 2,
    };
    let col = match coord.col {
        Col::A => 0,
        Col::B => 1,
        Col::C => 2,
    };
    row * 3 + col
}

pub struct Game {
    board: [Option<Player>; 9],
    current_player: Player,
    winner: Option<Player>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: [None; 9],
            current_player: Player::X,
            winner: None,
        }
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn check_winner(&self) -> Option<Player> {
        const LINES: [[usize; 3]; 8] = [
            [0, 1, 2], // row 1
            [3, 4, 5], // row 2
            [6, 7, 8], // row 3
            [0, 3, 6], // col A
            [1, 4, 7], // col B
            [2, 5, 8], // col C
            [0, 4, 8], // diagonal
            [2, 4, 6], // anti-diagonal
        ];

        for line in &LINES {
            let [a, b, c] = *line;
            if let (Some(p1), Some(p2), Some(p3)) = (self.board[a], self.board[b], self.board[c])
                && p1 == p2
                && p2 == p3
            {
                return Some(p1);
            }
        }
        None
    }

    pub fn make_move(&mut self, coord: Coordinate) -> Result<Option<Player>, MoveError> {
        if self.winner.is_some() {
            return Err(MoveError::AlreadyWinner);
        }

        let idx = coord_index(coord);
        if self.board[idx].is_some() {
            return Err(MoveError::AlreadyOccupied);
        }

        self.board[idx] = Some(self.current_player);

        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        self.winner = self.check_winner();
        Ok(self.winner)
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "    A   B   C")?;
        writeln!(f, "  +---+---+---+")?;
        for (row_idx, row_label) in [(0, '1'), (1, '2'), (2, '3')] {
            write!(f, "{} |", row_label)?;
            for col_idx in 0..3 {
                let cell = match self.board[row_idx * 3 + col_idx] {
                    Some(Player::X) => " X ",
                    Some(Player::O) => " O ",
                    None => "   ",
                };
                write!(f, "{}|", cell)?;
            }
            writeln!(f)?;
            writeln!(f, "  +---+---+---+")?;
        }
        Ok(())
    }
}

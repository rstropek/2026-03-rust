use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    InProgress,
    Won(Player),
    Draw,
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
pub enum ParseCoordinateError {
    WrongLength,
    UnknownColumn(char),
    UnknownRow(char),
}

impl fmt::Display for ParseCoordinateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseCoordinateError::WrongLength => {
                write!(f, "Invalid coordinate length. Use format like A1, B2, C3")
            }
            ParseCoordinateError::UnknownColumn(c) => {
                write!(f, "Invalid column '{c}'. Must be A, B, or C")
            }
            ParseCoordinateError::UnknownRow(r) => {
                write!(f, "Invalid row '{r}'. Must be 1, 2, or 3")
            }
        }
    }
}

impl FromStr for Coordinate {
    type Err = ParseCoordinateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_uppercase();
        if s.len() != 2 {
            return Err(ParseCoordinateError::WrongLength);
        }
        let mut chars = s.chars();
        let col_char = chars.next().unwrap();
        let row_char = chars.next().unwrap();

        let col = match col_char {
            'A' => Col::A,
            'B' => Col::B,
            'C' => Col::C,
            _ => return Err(ParseCoordinateError::UnknownColumn(col_char)),
        };

        let row = match row_char {
            '1' => Row::One,
            '2' => Row::Two,
            '3' => Row::Three,
            _ => return Err(ParseCoordinateError::UnknownRow(row_char)),
        };

        Ok(Coordinate { row, col })
    }
}

#[derive(Debug)]
pub enum MoveError {
    AlreadyOccupied,
    GameOver,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveError::AlreadyOccupied => write!(f, "That field is already occupied"),
            MoveError::GameOver => write!(f, "The game is already over"),
        }
    }
}

// Board layout: index = row * 3 + col
// Row::One=0, Row::Two=1, Row::Three=2
// Col::A=0, Col::B=1, Col::C=2
impl From<Coordinate> for usize {
    fn from(coord: Coordinate) -> Self {
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
}

pub struct Game {
    board: [Option<Player>; 9],
    current_player: Player,
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: [None; 9],
            current_player: Player::X,
            state: GameState::InProgress,
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

    pub fn make_move(&mut self, coord: Coordinate) -> Result<GameState, MoveError> {
        if self.state != GameState::InProgress {
            return Err(MoveError::GameOver);
        }

        let idx = usize::from(coord); // We get from the From trait
        //let idx: usize = coord.into();      // We get from the Into trait
        if self.board[idx].is_some() {
            return Err(MoveError::AlreadyOccupied);
        }

        self.board[idx] = Some(self.current_player);

        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        self.state = if let Some(winner) = self.check_winner() {
            GameState::Won(winner)
        } else if self.board.iter().all(|c| c.is_some()) {
            GameState::Draw
        } else {
            GameState::InProgress
        };

        Ok(self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn coord(s: &str) -> Coordinate {
        s.parse().unwrap()
    }

    // --- From<Coordinate> for usize ---

    #[test]
    fn coord_to_index_all_cells() {
        assert_eq!(usize::from(coord("A1")), 0);
        assert_eq!(usize::from(coord("B1")), 1);
        assert_eq!(usize::from(coord("C1")), 2);
        assert_eq!(usize::from(coord("A2")), 3);
        assert_eq!(usize::from(coord("B2")), 4);
        assert_eq!(usize::from(coord("C2")), 5);
        assert_eq!(usize::from(coord("A3")), 6);
        assert_eq!(usize::from(coord("B3")), 7);
        assert_eq!(usize::from(coord("C3")), 8);
    }

    // --- Coordinate parsing ---

    #[test]
    fn parse_coordinate_valid() {
        let c = coord("B2");
        assert_eq!(c.row, Row::Two);
        assert_eq!(c.col, Col::B);
    }

    #[test]
    fn parse_coordinate_lowercase() {
        let c = coord("c3");
        assert_eq!(c.row, Row::Three);
        assert_eq!(c.col, Col::C);
    }

    #[test]
    fn parse_coordinate_wrong_length() {
        assert!(matches!(
            "A".parse::<Coordinate>(),
            Err(ParseCoordinateError::WrongLength)
        ));
        assert!(matches!(
            "A12".parse::<Coordinate>(),
            Err(ParseCoordinateError::WrongLength)
        ));
    }

    #[test]
    fn parse_coordinate_unknown_column() {
        assert!(matches!(
            "D1".parse::<Coordinate>(),
            Err(ParseCoordinateError::UnknownColumn('D'))
        ));
    }

    #[test]
    fn parse_coordinate_unknown_row() {
        assert!(matches!(
            "A4".parse::<Coordinate>(),
            Err(ParseCoordinateError::UnknownRow('4'))
        ));
    }

    // --- Game::new ---

    #[test]
    fn new_game_starts_with_player_x() {
        let game = Game::new();
        assert_eq!(game.current_player(), Player::X);
    }

    #[test]
    fn new_game_has_no_winner() {
        let game = Game::new();
        assert_eq!(game.check_winner(), None);
    }

    // --- make_move ---

    #[test]
    fn players_alternate() {
        let mut game = Game::new();
        game.make_move(coord("A1")).unwrap();
        assert_eq!(game.current_player(), Player::O);
        game.make_move(coord("B1")).unwrap();
        assert_eq!(game.current_player(), Player::X);
    }

    #[test]
    fn move_on_occupied_cell_returns_error() {
        let mut game = Game::new();
        game.make_move(coord("A1")).unwrap();
        assert!(matches!(
            game.make_move(coord("A1")),
            Err(MoveError::AlreadyOccupied)
        ));
    }

    #[test]
    fn move_after_winner_returns_error() {
        let mut game = Game::new();
        // X wins the first row
        game.make_move(coord("A1")).unwrap();
        game.make_move(coord("A2")).unwrap();
        game.make_move(coord("B1")).unwrap();
        game.make_move(coord("B2")).unwrap();
        game.make_move(coord("C1")).unwrap(); // X wins
        assert!(matches!(
            game.make_move(coord("C2")),
            Err(MoveError::GameOver)
        ));
    }

    // --- check_winner ---

    #[test]
    fn winner_row() {
        let mut game = Game::new();
        // X: A1 B1 C1  (row 1)
        // O: A2 B2
        game.make_move(coord("A1")).unwrap();
        game.make_move(coord("A2")).unwrap();
        game.make_move(coord("B1")).unwrap();
        game.make_move(coord("B2")).unwrap();
        let result = game.make_move(coord("C1")).unwrap();
        assert_eq!(result, GameState::Won(Player::X));
    }

    #[test]
    fn winner_column() {
        let mut game = Game::new();
        // X: A1 A2 A3  (col A)
        // O: B1 B2
        game.make_move(coord("A1")).unwrap();
        game.make_move(coord("B1")).unwrap();
        game.make_move(coord("A2")).unwrap();
        game.make_move(coord("B2")).unwrap();
        let result = game.make_move(coord("A3")).unwrap();
        assert_eq!(result, GameState::Won(Player::X));
    }

    #[test]
    fn winner_diagonal() {
        let mut game = Game::new();
        // X: A1 B2 C3
        // O: B1 C1
        game.make_move(coord("A1")).unwrap();
        game.make_move(coord("B1")).unwrap();
        game.make_move(coord("B2")).unwrap();
        game.make_move(coord("C1")).unwrap();
        let result = game.make_move(coord("C3")).unwrap();
        assert_eq!(result, GameState::Won(Player::X));
    }

    #[test]
    fn winner_anti_diagonal() {
        let mut game = Game::new();
        // X: C1 B2 A3
        // O: A1 B1
        game.make_move(coord("C1")).unwrap();
        game.make_move(coord("A1")).unwrap();
        game.make_move(coord("B2")).unwrap();
        game.make_move(coord("B1")).unwrap();
        let result = game.make_move(coord("A3")).unwrap();
        assert_eq!(result, GameState::Won(Player::X));
    }

    #[test]
    fn in_progress_returns_in_progress() {
        let mut game = Game::new();
        game.make_move(coord("A1")).unwrap();
        game.make_move(coord("B1")).unwrap();
        let result = game.make_move(coord("C1")).unwrap();
        assert_eq!(result, GameState::InProgress);
    }

    #[test]
    fn draw_when_board_full_no_winner() {
        let mut game = Game::new();
        // X O X
        // X O X
        // O X O  → no winner
        game.make_move(coord("A1")).unwrap(); // X
        game.make_move(coord("B1")).unwrap(); // O
        game.make_move(coord("C1")).unwrap(); // X
        game.make_move(coord("B2")).unwrap(); // O
        game.make_move(coord("A2")).unwrap(); // X
        game.make_move(coord("A3")).unwrap(); // O
        game.make_move(coord("C2")).unwrap(); // X
        game.make_move(coord("C3")).unwrap(); // O
        let result = game.make_move(coord("B3")).unwrap(); // X
        assert_eq!(result, GameState::Draw);
    }

    #[test]
    fn move_after_draw_returns_error() {
        let mut game = Game::new();
        game.make_move(coord("A1")).unwrap();
        game.make_move(coord("B1")).unwrap();
        game.make_move(coord("C1")).unwrap();
        game.make_move(coord("B2")).unwrap();
        game.make_move(coord("A2")).unwrap();
        game.make_move(coord("A3")).unwrap();
        game.make_move(coord("C2")).unwrap();
        game.make_move(coord("C3")).unwrap();
        game.make_move(coord("B3")).unwrap(); // draw
        assert!(matches!(
            game.make_move(coord("A1")),
            Err(MoveError::GameOver)
        ));
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

mod tictactoe;

use std::io::{self, BufRead, Write};
use tictactoe::{Game, GameState};

fn main() {
    let mut game = Game::new();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        println!("{game}");
        print!(
            "Player {}, enter your move (e.g. A1): ",
            game.current_player()
        );
        io::stdout().flush().unwrap();

        let Some(Ok(input)) = lines.next() else {
            break;
        };

        let coord = match input.trim().parse() {
            Ok(c) => c,
            Err(e) => {
                println!("Error: {e}");
                continue;
            }
        };

        match game.make_move(coord) {
            Ok(GameState::Won(winner)) => {
                println!("{game}");
                println!("Player {winner} wins!");
                break;
            }
            Ok(GameState::Draw) => {
                println!("{game}");
                println!("It's a draw!");
                break;
            }
            Ok(GameState::InProgress) => {}
            Err(e) => println!("Error: {e}"),
        }
    }
}

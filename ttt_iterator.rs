struct TicTacToeBoard<T> {
    board: [[T; 3]; 3],
}

struct TicTacToeBoardIter<'a, T> {
    cells: &'a [[T; 3]; 3],
    current_index: usize, // 0..=8
}

impl<T: Copy> Iterator for TicTacToeBoardIter<'_, T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_index {
            0..=8 => {
                let row = self.current_index / 3;
                let col = self.current_index % 3;
                self.current_index += 1;
                Some(self.cells[row][col])
            }
            _ => None,
        }
    }
}

impl<'a, T: Copy> IntoIterator for &'a TicTacToeBoard<T> {
    type Item = T;
    type IntoIter = TicTacToeBoardIter<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        TicTacToeBoardIter {
            cells: &self.board,
            current_index: 0,
        }
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    for number in numbers.iter().skip(2).take(3).filter(|&n| n % 2 == 0) {
        println!("{}", number);
    }

    let board = TicTacToeBoard {
        board: [
            [Some('X'), Some('O'), Some('X')],
            [Some('O'), Some('X'), None],
            [Some('X'), Some('O'), Some('X')],
        ],
    };
    for cell in &board {
        match cell {
            Some(player) => print!("{} ", player),
            None => print!(". "),
        }
    }
}

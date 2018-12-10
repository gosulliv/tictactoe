use std::fmt::{Display, Error, Formatter};
use std::io::{BufRead,Write};

#[derive(PartialEq,Eq,Debug, Clone, Copy)]
enum Symbol {
    X,
    O,
}

use self::Symbol::{O, X};

enum GameState {
    Win(Symbol),
    InProgress,
    Draw,
}

struct TicTacToe {
    /// index by row then column
    board: [[Option<Symbol>; 3]; 3],
    whose_turn: Symbol,
}


impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            board: [[None; 3]; 3],
            whose_turn: X,
        }
    }

    //pub fn reset(&mut self) {
        //self.board = [[None; 3]; 3];
        //self.whose_turn = X;
    //}

    pub fn go(&mut self, x: usize, y: usize) -> Result<GameState, &'static str> {
        if  x > 2 || y > 2 {
            return Err("Index out of range. Must be in from 0 to 2")
        }

        match self.board[x][y] {
            None => self.board[x][y] = Some(self.whose_turn),
            Some(_) => return Err("Can't move in an occupied space"), // can't go there!
        }

        self.whose_turn = match self.whose_turn {
            X => O,
            O => X,
        };

        Ok(self.current_state())
    }

    pub fn current_state(&self) -> GameState {
        let mut moves_remaining = false;
        let board = self.board;

        // rows
        for row in &board {
            if row[0] == row[1] && row[0] == row[2] {
                return GameState::Win(row[0]);
            }
        }

        let columns = (0..2).iter()
            .map ({ |i|  [board.row[0][i], board.row[1][i], board.row[2][i]] });
        for column in &columns {
            if let Some(symbol) = column.next() {
                if column.all(symbol) {
                return GameState::Win(symbol);
                }
            }
        }

        // diagonals
        let middle = board[1][1];
        if middle == board[0][0] && middle == board[2][2] ||
            middle == board[2][0] && middle = board[0][2] {
                return middle;
            }

        if board.iter().iter().flatten().all(|x| x.is_some()) {
            GameState::Draw
        } else {
            GameState::InProgress
        }
    }
}

impl Display for TicTacToe {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for row in &self.board {
            let line = row.iter().map(|elt| match elt {
                None => ' ',
                Some(Symbol::X) => 'X',
                Some(Symbol::O) => 'O',
            });
            writeln!(f, "|{}|", &line.collect::<String>())?;
        }
        Ok(())
    }
}

fn main() {
    let mut board = TicTacToe::new();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    loop {
        write!(stdout, "{}\nYour move > ", &board)
            .unwrap();
        stdout.flush()
            .unwrap();
        let mut input_text = String::new();

        stdin.read_line(&mut input_text).unwrap();

        let input_text = input_text.trim();
        let mut numbers = input_text.split(',').map(|s| s.parse().unwrap());

        let x = numbers.next().unwrap();
        let y = numbers.next().unwrap();

        board.go(x, y)
            .unwrap_or_else(|msg| {
            writeln!(stdout, "Move failed: {}", msg)
                .unwrap();
        });
        

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    #[test]
    fn display() {
        use super::Symbol::*;

        let display_testcase = |board: &TicTacToe, expected: &str| {
            let mut s: String = String::new();
            write!(&mut s, "{}", &board).unwrap();
            assert_eq!(&s, expected);
        };

        let mut board = TicTacToe::new();
        display_testcase(&board, "|   |\n|   |\n|   |\n");

        board.board[0][0] = Some(X);
        display_testcase(&board, "|X  |\n|   |\n|   |\n");

        board.board[0][0] = Some(O);
        display_testcase(&board, "|O  |\n|   |\n|   |\n");

        board.board[1][1] = Some(X);
        display_testcase(&board, "|O  |\n| X |\n|   |\n");

        board.board[2][0] = Some(O);
        display_testcase(&board, "|O  |\n| X |\n|O  |\n");

        board.board[1][0] = Some(X);
        display_testcase(&board, "|O  |\n|XX |\n|O  |\n");

        board.board[2][2] = Some(O);
        display_testcase(&board, "|O  |\n|XX |\n|O O|\n");
    }

    #[test]
    fn moves() {
        let mut board = TicTacToe::new();

        board.go(0,0).unwrap();
        board.go(1,1).unwrap();
        board.go(0,1).unwrap();

    }

    #[test]
    fn range_result_panic() {
        let mut board = TicTacToe::new();

        assert!(board.go(3,0).is_err());
        assert!(board.go(0,3).is_err());
    }

}

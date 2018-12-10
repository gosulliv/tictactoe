use std::fmt::{Display, Error, Formatter};
use std::io::{BufRead, Write};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Symbol {
    X,
    O,
}
impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let c = match self {
            Symbol::X => 'X',
            Symbol::O => 'O',
        };
        write!(f, "{}", &c)
    }
}

use self::Symbol::{O, X};

#[derive(Debug,PartialEq,Eq)]
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
        if x > 2 || y > 2 {
            return Err("Index out of range. Must be in from 0 to 2");
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
        let board = self.board;
        let these_win = |a: Option<Symbol>, b: Option<Symbol>, c: Option<Symbol>| {
            if a == b && a == c {
                a.map(|x| GameState::Win(x))
            } else {
                None
            }
        };

        let rows = self.board.iter();
        let columns = (0..=2)
            .into_iter()
            .map({ |i| [board[0][i], board[1][i], board[2][i]] });

        None.or_else(|| {
            rows.flat_map(|row| these_win(row[0], row[1], row[2]))
                .next()
        })
        .or_else(|| {
            columns
                .flat_map(|column| these_win(column[0], column[1], column[2]))
                .next()
        })
        .or_else(|| these_win(board[0][0], board[1][1], board[2][2]))
        .or_else(|| these_win(board[2][0], board[1][1], board[0][2]))
        .or_else(|| {
            if board
                .iter()
                .map(|x| x.iter())
                .flatten()
                .all(|x| x.is_some())
            {
                Some(GameState::Draw)
            } else {
                Some(GameState::InProgress)
            }
        })
        .unwrap()
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
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    loop {
        let mut board = TicTacToe::new();

        loop {
            write!(stdout, "{}\n{} to move > ", &board, &board.whose_turn).unwrap();
            stdout.flush().unwrap();

            let mut input_text = String::new();
            stdin.read_line(&mut input_text).unwrap();

            let input_text = input_text.trim();
            let mut numbers = input_text.split(',').map(|s| s.parse().unwrap());

            let x = numbers.next().unwrap();
            let y = numbers.next().unwrap();

            match board.go(x, y) {
                Ok(GameState::Win(x)) => {writeln!(stdout, "{} wins!", x).unwrap(); break
                },
                Ok(GameState::Draw) => {
                    writeln!(stdout, "Draw game!").unwrap(); break
                },
                Err(msg) => writeln!(stdout, "Move failed: {}", msg).unwrap(),
                Ok(GameState::InProgress) => (),
            };
        }
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

        board.go(0, 0).unwrap();
        board.go(1, 1).unwrap();
        board.go(0, 1).unwrap();
    }

    #[test]
    fn range_result_panic() {
        let mut board = TicTacToe::new();

        assert!(board.go(3, 0).is_err());
        assert!(board.go(0, 3).is_err());
    }

    #[test]
    fn o_wins() {
        let mut board = TicTacToe::new();
        assert_eq!(GameState::InProgress, board.go(1,1).unwrap());
        assert_eq!(GameState::InProgress,board.go(1,2).unwrap());
        assert_eq!(GameState::InProgress,board.go(2,0).unwrap());
        assert_eq!(GameState::InProgress,board.go(0,2).unwrap());
        assert_eq!(GameState::InProgress,board.go(0,0).unwrap());
        assert_eq!(GameState::Win(O), board.go(2,2) .unwrap());
    }
}

use serde::Serialize;

use crate::tic_tac_toe::errors::TicTacToeError;

#[derive(Clone, Copy, PartialEq, Serialize, Debug)]
pub enum Letters {
    X,
    O,
}

#[derive(PartialEq, Clone)]
pub enum Winner {
    Option(Option<Letters>),
    None,
}

#[derive(Clone)]
pub struct Game {
    pub cells: Vec<Vec<Option<Letters>>>,
    pub next_letter: Letters,
    pub winner: Winner,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            cells: vec![
                vec![None, None, None],
                vec![None, None, None],
                vec![None, None, None],
            ],
            next_letter: Letters::X,
            winner: Winner::None,
        }
    }

    pub fn make_move(&mut self, x: usize, y: usize) -> Result<Winner, TicTacToeError> {
        if x > 2 || y > 2 {
            return Err(TicTacToeError::FieldOutOfBounds);
        }

        if self.winner != Winner::None {
            return Err(TicTacToeError::GameFinished);
        }

        if self.cells[x][y].is_none() {
            self.cells[x][y] = Some(self.next_letter);
            self.switch_letter();
        } else {
            return Err(TicTacToeError::FieldOccupied);
        }

        let winner = self.get_winner();
        if winner != Winner::None {
            self.winner = winner.clone();
        }

        Ok(winner)
    }

    fn get_winner(&self) -> Winner {
        for i in 0..3 {
            if self.cells[i][0].is_some()
                && self.cells[i][0] == self.cells[i][1]
                && self.cells[i][0] == self.cells[i][2]
            {
                return Winner::Option(self.cells[i][0]);
            }

            if self.cells[0][i].is_some()
                && self.cells[0][i] == self.cells[1][i]
                && self.cells[0][i] == self.cells[2][i]
            {
                return Winner::Option(self.cells[0][i]);
            }
        }

        if self.cells[0][0].is_some()
            && self.cells[0][0] == self.cells[1][1]
            && self.cells[0][0] == self.cells[2][2]
        {
            return Winner::Option(self.cells[0][0]);
        }

        if self.cells[2][0].is_some()
            && self.cells[2][0] == self.cells[1][1]
            && self.cells[2][0] == self.cells[0][2]
        {
            return Winner::Option(self.cells[2][0]);
        }

        let mut occupied = 0;
        for i in 0..3 {
            for j in 0..3 {
                if self.cells[i][j].is_some() {
                    occupied += 1;
                }
            }
        }

        if occupied == 9 {
            return Winner::Option(None);
        }

        Winner::None
    }

    fn switch_letter(&mut self) {
        if self.next_letter == Letters::X {
            self.next_letter = Letters::O;
        } else {
            self.next_letter = Letters::X;
        }
    }
}

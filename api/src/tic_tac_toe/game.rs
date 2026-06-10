use crate::tic_tac_toe::errors::TicTacToeError;

pub struct Game {
    pub board: Vec<Vec<Option<char>>>,
    pub next_letter: char,
    pub winner: Option<char>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: vec![
                vec![None, None, None],
                vec![None, None, None],
                vec![None, None, None],
            ],
            next_letter: 'X',
            winner: None,
        }
    }

    pub fn make_move(&mut self, x: usize, y: usize) -> Result<Option<char>, TicTacToeError> {
        if x > 2 || y > 2 {
            return Err(TicTacToeError::FieldOutOfBounds);
        }

        if self.winner.is_some() {
            return Err(TicTacToeError::GameFinished);
        }

        if self.board[x][y].is_none() {
            self.board[x][y] = Some(self.next_letter);
            self.switch_letter();
        } else {
            return Err(TicTacToeError::FieldOccupied);
        }

        let winner = self.get_winner();
        if winner.is_some() {
            self.winner = winner;
        }

        Ok(winner)
    }

    fn get_winner(&self) -> Option<char> {
        for i in 0..3 {
            if self.board[i][0].is_some()
                && self.board[i][0] == self.board[i][1]
                && self.board[i][0] == self.board[i][2]
            {
                return self.board[i][0];
            }

            if self.board[0][i].is_some()
                && self.board[0][i] == self.board[1][i]
                && self.board[0][i] == self.board[2][i]
            {
                return self.board[0][i];
            }
        }

        if self.board[0][0].is_some()
            && self.board[0][0] == self.board[1][1]
            && self.board[0][0] == self.board[2][2]
        {
            return self.board[0][0];
        }

        if self.board[2][0].is_some()
            && self.board[2][0] == self.board[1][1]
            && self.board[2][0] == self.board[0][2]
        {
            return self.board[2][0];
        }

        None
    }

    fn switch_letter(&mut self) {
        if self.next_letter == 'X' {
            self.next_letter = 'O';
        } else {
            self.next_letter = 'X';
        }
    }
}

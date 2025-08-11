use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Mancala {
    pub board: [u8; 14],
    pub current_player: usize,
    pub game_over: bool,
    pub winner: Option<usize>,
}

impl Mancala {
    pub fn new() -> Mancala {
        let mut board = [4; 14];
        board[6] = 0;
        board[13] = 0;
        Mancala {
            board,
            current_player: 0,
            game_over: false,
            winner: None,
        }
    }

    pub fn make_move(&mut self, pit_index: usize) {
        if self.game_over || self.board[pit_index] == 0 {
            return;
        }

        let mut stones = self.board[pit_index];
        self.board[pit_index] = 0;
        let mut current_index = pit_index;

        while stones > 0 {
            current_index = (current_index + 1) % 14;
            if (self.current_player == 0 && current_index == 13) || (self.current_player == 1 && current_index == 6) {
                continue;
            }
            self.board[current_index] += 1;
            stones -= 1;
        }

        let last_stone_index = current_index;

        if self.current_player == 0 && last_stone_index == 6 {
            self.check_game_over();
            return;
        } else if self.current_player == 1 && last_stone_index == 13 {
            self.check_game_over();
            return;
        }

        if self.board[last_stone_index] == 1 && (0..6).contains(&last_stone_index) && self.current_player == 0 {
            let opposite_index = 12 - last_stone_index;
            if self.board[opposite_index] > 0 {
                self.board[6] += self.board[opposite_index] + 1;
                self.board[last_stone_index] = 0;
                self.board[opposite_index] = 0;
            }
        } else if self.board[last_stone_index] == 1 && (7..13).contains(&last_stone_index) && self.current_player == 1 {
            let opposite_index = 12 - last_stone_index;
            if self.board[opposite_index] > 0 {
                self.board[13] += self.board[opposite_index] + 1;
                self.board[last_stone_index] = 0;
                self.board[opposite_index] = 0;
            }
        }

        self.current_player = 1 - self.current_player;
        self.check_game_over();
    }

    fn check_game_over(&mut self) {
        let player1_pits: &[u8] = &self.board[0..6];
        let player2_pits: &[u8] = &self.board[7..13];

        if player1_pits.iter().all(|&x| x == 0) || player2_pits.iter().all(|&x| x == 0) {
            self.game_over = true;
            let player1_score = self.board[6];
            let player2_score = self.board[13];

            if player1_score > player2_score {
                self.winner = Some(0);
            } else if player2_score > player1_score {
                self.winner = Some(1);
            } else {
                self.winner = None; // Draw
            }
        }
    }
}

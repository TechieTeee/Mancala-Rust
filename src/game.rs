pub const PLAYER1_STORE: usize = 6;
pub const PLAYER2_STORE: usize = 13;
pub const PLAYER1_PITS: std::ops::Range<usize> = 0..6;
pub const PLAYER2_PITS: std::ops::Range<usize> = 7..13;
const BOARD_SIZE: usize = 14;
const INITIAL_STONES: u8 = 4;

#[derive(Clone, PartialEq, Debug)]
pub struct Mancala {
    pub board: [u8; BOARD_SIZE],
    pub current_player: usize,
    pub game_over: bool,
    pub winner: Option<usize>,
}

impl Mancala {
    pub fn new() -> Mancala {
        let mut board = [INITIAL_STONES; BOARD_SIZE];
        board[PLAYER1_STORE] = 0;
        board[PLAYER2_STORE] = 0;
        Mancala {
            board,
            current_player: 0,
            game_over: false,
            winner: None,
        }
    }

    pub fn is_valid_move(&self, pit_index: usize) -> bool {
        if self.game_over || self.board[pit_index] == 0 {
            return false;
        }
        match self.current_player {
            0 => PLAYER1_PITS.contains(&pit_index),
            1 => PLAYER2_PITS.contains(&pit_index),
            _ => false,
        }
    }

    pub fn make_move(&mut self, pit_index: usize) -> bool {
        if !self.is_valid_move(pit_index) {
            return false;
        }

        let opponent_store = if self.current_player == 0 { PLAYER2_STORE } else { PLAYER1_STORE };
        let own_store = if self.current_player == 0 { PLAYER1_STORE } else { PLAYER2_STORE };

        let mut stones = self.board[pit_index];
        self.board[pit_index] = 0;
        let mut current_index = pit_index;

        while stones > 0 {
            current_index = (current_index + 1) % BOARD_SIZE;
            if current_index == opponent_store {
                continue;
            }
            self.board[current_index] += 1;
            stones -= 1;
        }

        let last_stone_index = current_index;

        // Extra turn: last stone lands in own store
        if last_stone_index == own_store {
            self.check_game_over();
            return true;
        }

        // Capture: last stone lands in empty pit on own side
        if self.board[last_stone_index] == 1 {
            let own_pits = if self.current_player == 0 { PLAYER1_PITS } else { PLAYER2_PITS };
            if own_pits.contains(&last_stone_index) {
                let opposite_index = 12 - last_stone_index;
                if self.board[opposite_index] > 0 {
                    self.board[own_store] += self.board[opposite_index] + 1;
                    self.board[last_stone_index] = 0;
                    self.board[opposite_index] = 0;
                }
            }
        }

        self.current_player = 1 - self.current_player;
        self.check_game_over();
        true
    }

    fn check_game_over(&mut self) {
        let player1_empty = self.board[PLAYER1_PITS].iter().all(|&x| x == 0);
        let player2_empty = self.board[PLAYER2_PITS].iter().all(|&x| x == 0);

        if player1_empty || player2_empty {
            self.game_over = true;

            // Sweep remaining stones into each player's store
            for i in PLAYER1_PITS {
                self.board[PLAYER1_STORE] += self.board[i];
                self.board[i] = 0;
            }
            for i in PLAYER2_PITS {
                self.board[PLAYER2_STORE] += self.board[i];
                self.board[i] = 0;
            }

            let player1_score = self.board[PLAYER1_STORE];
            let player2_score = self.board[PLAYER2_STORE];

            if player1_score > player2_score {
                self.winner = Some(0);
            } else if player2_score > player1_score {
                self.winner = Some(1);
            } else {
                self.winner = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_has_correct_initial_state() {
        let game = Mancala::new();
        for i in PLAYER1_PITS {
            assert_eq!(game.board[i], 4);
        }
        for i in PLAYER2_PITS {
            assert_eq!(game.board[i], 4);
        }
        assert_eq!(game.board[PLAYER1_STORE], 0);
        assert_eq!(game.board[PLAYER2_STORE], 0);
        assert_eq!(game.current_player, 0);
        assert!(!game.game_over);
        assert_eq!(game.winner, None);
    }

    #[test]
    fn basic_move_distributes_stones() {
        let mut game = Mancala::new();
        game.make_move(0);
        assert_eq!(game.board[0], 0);
        assert_eq!(game.board[1], 5);
        assert_eq!(game.board[2], 5);
        assert_eq!(game.board[3], 5);
        assert_eq!(game.board[4], 5);
    }

    #[test]
    fn extra_turn_when_last_stone_lands_in_own_store() {
        let mut game = Mancala::new();
        // Pit 2 has 4 stones, lands in indices 3,4,5,6(store)
        game.make_move(2);
        assert_eq!(game.board[PLAYER1_STORE], 1);
        // Player 0 should get another turn
        assert_eq!(game.current_player, 0);
    }

    #[test]
    fn turn_switches_after_normal_move() {
        let mut game = Mancala::new();
        game.make_move(0); // stones go to 1,2,3,4 - no store landing
        assert_eq!(game.current_player, 1);
    }

    #[test]
    fn skips_opponent_store() {
        let mut game = Mancala::new();
        game.board = [0; 14];
        game.board[5] = 8; // 8 stones from pit 5
        // Add stones to player 1 pits so game doesn't end
        game.board[0] = 2;
        game.board[1] = 2;
        game.current_player = 0;

        game.make_move(5);
        // Stones go: 6(store),7,8,9,10,11,12, skip 13, 0
        assert_eq!(game.board[PLAYER2_STORE], 0, "opponent store must be skipped");
        assert_eq!(game.board[PLAYER1_STORE], 1);
        assert_eq!(game.board[12], 1);
    }

    #[test]
    fn capture_on_own_empty_pit() {
        let mut game = Mancala::new();
        game.board = [0; 14];
        game.board[0] = 1; // one stone in pit 0
        game.board[11] = 5; // opposite pit (12 - 1 = 11) has stones
        // Wait, pit 0's opposite is 12 - 0 = 12
        // Let's set up: pit 0 has 1 stone, move lands in pit 1 (empty), opposite is 12 - 1 = 11
        game.board[0] = 1;
        game.board[1] = 0; // landing pit is empty
        game.board[11] = 5; // opposite of pit 1
        game.current_player = 0;

        game.make_move(0);
        // Last stone lands in pit 1 (was empty) -> capture
        // Store gets: 5 (from opposite) + 1 (the stone itself) = 6
        assert_eq!(game.board[PLAYER1_STORE], 6);
        assert_eq!(game.board[1], 0);
        assert_eq!(game.board[11], 0);
    }

    #[test]
    fn no_capture_on_opponent_side() {
        let mut game = Mancala::new();
        game.board = [0; 14];
        game.board[5] = 3; // 3 stones from pit 5 -> lands in 6(store), 7, 8
        game.board[4] = 3; // opposite of 8 is 12-8=4, has stones
        game.current_player = 0;

        game.make_move(5);
        // Last stone lands in pit 8 (opponent's side) - no capture
        assert_eq!(game.board[PLAYER1_STORE], 1); // only the store stone
        assert_eq!(game.board[8], 1); // stone stays
        assert_eq!(game.board[4], 3); // opposite untouched
    }

    #[test]
    fn no_capture_when_opposite_is_empty() {
        let mut game = Mancala::new();
        game.board = [0; 14];
        game.board[0] = 1;
        game.board[1] = 0; // landing pit empty
        game.board[11] = 0; // opposite also empty
        // Keep player 1 side alive so game doesn't end immediately
        game.board[3] = 2;
        // Keep player 2 side alive
        game.board[7] = 2;
        game.current_player = 0;

        game.make_move(0);
        // Lands in pit 1, opposite is empty - no capture
        assert_eq!(game.board[1], 1); // stone stays
        assert_eq!(game.board[PLAYER1_STORE], 0);
    }

    #[test]
    fn game_over_sweeps_remaining_stones() {
        let mut game = Mancala::new();
        game.board = [0; 14];
        game.board[0] = 1; // only one stone left on player 1's side
        game.board[7] = 3;
        game.board[8] = 5;
        game.current_player = 0;

        game.make_move(0);
        // Stone goes to pit 1, but now all of player 1's pits... wait, pit 1 now has 1
        // Let me set this up so the move empties player 1's side

        // Reset: player 1 has 1 stone in pit 5, move lands in store
        game = Mancala::new();
        game.board = [0; 14];
        game.board[4] = 2; // lands at 5, 6(store) - extra turn
        game.current_player = 0;
        // Actually let's make it simpler
        game.board = [0; 14];
        game.board[5] = 1; // 1 stone, lands in store (index 6)
        game.board[7] = 3;
        game.board[8] = 5;
        game.current_player = 0;

        game.make_move(5);
        // Last stone in own store -> extra turn, but player 1 side is now empty
        // check_game_over triggers, sweeps player 2's stones
        assert!(game.game_over);
        assert_eq!(game.board[PLAYER1_STORE], 1);
        assert_eq!(game.board[PLAYER2_STORE], 8); // 3 + 5 swept
        assert_eq!(game.board[7], 0); // swept
        assert_eq!(game.board[8], 0); // swept
        assert_eq!(game.winner, Some(1)); // player 2 wins 8 vs 1
    }

    #[test]
    fn draw_game() {
        let mut game = Mancala::new();
        game.board = [0; 14];
        game.board[PLAYER1_STORE] = 24;
        game.board[PLAYER2_STORE] = 23;
        game.board[5] = 1; // last stone, lands in store
        game.current_player = 0;

        game.make_move(5);
        assert!(game.game_over);
        // 24 + 1 = 25 vs 23... not a draw. Let me fix.
        // We need equal scores after sweep.
        game = Mancala::new();
        game.board = [0; 14];
        game.board[PLAYER1_STORE] = 23;
        game.board[PLAYER2_STORE] = 23;
        game.board[5] = 1;
        game.board[12] = 1;
        game.current_player = 0;

        game.make_move(5);
        // P1 store = 23+1=24, P2 side has pit 12 with 1 stone, swept -> P2 store = 23+1=24
        assert!(game.game_over);
        assert_eq!(game.board[PLAYER1_STORE], 24);
        assert_eq!(game.board[PLAYER2_STORE], 24);
        assert_eq!(game.winner, None);
    }

    #[test]
    fn invalid_move_returns_false() {
        let mut game = Mancala::new();
        // Player 0 can't move player 1's pits
        assert!(!game.make_move(7));
        assert!(!game.make_move(12));
        // Can't move from stores
        assert!(!game.make_move(6));
        assert!(!game.make_move(13));
    }

    #[test]
    fn cannot_move_empty_pit() {
        let mut game = Mancala::new();
        game.board[0] = 0;
        assert!(!game.make_move(0));
        assert_eq!(game.current_player, 0); // turn unchanged
    }

    #[test]
    fn cannot_move_after_game_over() {
        let mut game = Mancala::new();
        game.game_over = true;
        assert!(!game.make_move(0));
    }

    #[test]
    fn player2_capture() {
        let mut game = Mancala::new();
        game.board = [0; 14];
        game.board[7] = 1;
        game.board[8] = 0; // landing pit empty
        game.board[4] = 6; // opposite of 8 is 12-8=4
        game.current_player = 1;

        game.make_move(7);
        // Lands in pit 8, captures opposite (pit 4) = 6 + 1 = 7
        assert_eq!(game.board[PLAYER2_STORE], 7);
        assert_eq!(game.board[8], 0);
        assert_eq!(game.board[4], 0);
    }

    #[test]
    fn player2_extra_turn() {
        let mut game = Mancala::new();
        game.board = [0; 14];
        game.board[7] = 6;
        // Keep player 1 side alive so game doesn't end from sweep
        game.board[0] = 2;
        game.current_player = 1;

        game.make_move(7);
        // Stones go: 8,9,10,11,12,13(store) -> extra turn
        assert_eq!(game.board[PLAYER2_STORE], 1);
        assert_eq!(game.current_player, 1);
    }

    #[test]
    fn player2_skips_player1_store() {
        let mut game = Mancala::new();
        game.board = [0; 14];
        game.board[12] = 3;
        // Keep both sides alive
        game.board[9] = 2;
        game.board[0] = 2;
        game.current_player = 1;

        game.make_move(12);
        // Stones: 13(store), skip 6, 0, 1
        assert_eq!(game.board[PLAYER1_STORE], 0, "player 1 store must be skipped");
        assert_eq!(game.board[PLAYER2_STORE], 1);
        assert_eq!(game.board[0], 3); // 2 existing + 1 new
        assert_eq!(game.board[1], 1);
    }

    #[test]
    fn total_stones_conserved() {
        let mut game = Mancala::new();
        let initial_total: u8 = game.board.iter().sum();
        assert_eq!(initial_total, 48);

        game.make_move(0);
        game.make_move(7);
        game.make_move(1);
        game.make_move(8);

        let total: u8 = game.board.iter().sum();
        assert_eq!(total, 48, "total stones must be conserved");
    }
}

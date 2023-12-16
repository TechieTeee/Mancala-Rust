use std::io;
use std::io::Write;

struct Mancala {
    board: [u8; 14],
    current_player: usize,
}

impl Mancala {
    fn new() -> Mancala {
        Mancala {
            board: [4; 14],
            current_player: 0,
        }
    }

    fn display_board(&self) {
        println!("Player 2 (top)");
        println!("-----------------------");
        for i in (7..14).rev() {
            print!("| {:2} ", self.board[i]);
        }
        println!("|");
        println!("|---------------------|");
        for i in 0..7 {
            print!("| {:2} ", self.board[i]);
        }
        println!("|");
        println!("-----------------------");
        println!("Player 1 (bottom)");
    }

    fn make_move(&mut self, pit_index: usize) {
        let stones = self.board[pit_index];
        self.board[pit_index] = 0;
        let mut current_index = pit_index + 1;

        for _ in 0..stones {
            current_index %= 14;
            self.board[current_index] += 1;
            current_index += 1;
        }

        self.handle_capture(current_index - 1);
        self.switch_player();
    }

    fn handle_capture(&mut self, last_index: usize) {
        let opposite_index = 12 - last_index;

        if (0..7).contains(&last_index) && self.board[last_index] == 1 && self.board[opposite_index] > 0 {
            self.board[7] += self.board[opposite_index] + 1;
            self.board[last_index] = 0;
            self.board[opposite_index] = 0;
        }
    }

    fn switch_player(&mut self) {
        self.current_player = 1 - self.current_player;
    }

    fn game_over(&self) -> bool {
        self.board[0..7].iter().all(|&x| x == 0) || self.board[7..14].iter().all(|&x| x == 0)
    }

    fn get_valid_move(&self) -> usize {
        loop {
            print!("Enter your move (1-6): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let pit_index: usize = match input.trim().parse() {
                Ok(num) if (1..=6).contains(&num) => num - 1,
                _ => {
                    println!("Invalid input. Please enter a number between 1 and 6.");
                    continue;
                }
            };

            if self.board[pit_index + self.current_player * 7] == 0 {
                println!("Invalid move. Pit is empty. Try again.");
                continue;
            }

            return pit_index + self.current_player * 7;
        }
    }
}

fn main() {
    let mut game = Mancala::new();

    while !game.game_over() {
        game.display_board();
        let pit_index = game.get_valid_move();
        game.make_move(pit_index);
    }

    game.display_board();
    println!("Game over!");
}

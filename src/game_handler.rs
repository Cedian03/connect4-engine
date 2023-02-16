use std::fmt;

use crate::{Move, Position, Solver, State};

use rand::{self, seq::SliceRandom};

pub struct GameHandler {
    pub position: Position,
    pub state: State,
    pub turn: u8,
    solver: Solver,
    moves: Vec<Move>,
}

impl GameHandler {
    pub fn new() -> Self {
        let mut solver = Solver::new();
        solver.load_book(".book");

        Self {
            position: Position::new(),
            state: State::InProgress,
            turn: 1,
            solver,
            moves: Vec::new(),
        }
    }

    pub fn play(&mut self, col: i32) -> Option<Move> {
        if self.position.can_play(col) {
            let m = Move::new(&mut self.position, &mut self.solver, col);

            if m.is_mate {
                self.state = State::Won(self.position.disk_to_play());
            } else if m.is_draw {
                self.state = State::Draw;
            }

            self.position.play(col);
            self.moves.push(m);
            self.turn += 1;

            Some(m)
        } else {
            None
        }
    }

    pub fn optimal_col(&mut self) -> i32 {
        *self
            .solver
            .optimal_cols(&mut self.position)
            .choose(&mut rand::thread_rng())
            .unwrap()
    }

    pub fn transcribe(&self) -> String {
        let mut string = String::new();
        for (i, moves) in self.moves.chunks(2).enumerate() {
            string.push_str(&format!("{})", i + 1));

            let mut winning_disk = None;

            for m in moves {
                string.push_str(&format!(" {:<4}", format!("{}", m)));
                if m.is_mate {
                    winning_disk = Some(m.disk)
                }
            }

            if let Some(disk) = winning_disk {
                string.push_str(&format!(" {:?} wins!", disk))
            }

            string.push('\n');
        }

        string
    }
}

impl fmt::Display for GameHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let board = format!("{}", self.position);
        let transcript = self.transcribe();

        let board = board.lines();
        let mut transcript = transcript.lines().rev();

        for row in board {
            write!(f, "{} |", row)?;
            if let Some(x) = transcript.next() {
                write!(f, " {}", x)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

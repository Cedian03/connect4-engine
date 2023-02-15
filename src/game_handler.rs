use crate::{Disk, Move, Position, Solver, State};

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
        *self.solver.optimal_cols(&mut self.position).choose(&mut rand::thread_rng()).unwrap()
    }

    pub fn transcribe(&self) -> String {
        let mut s = String::new();
        for m in self.moves.iter() {
            s.push_str(&format!("{} ", m))
        }
        s
    }
}

use super::*;

impl Cube {
    pub fn do_move_sequence(&mut self, move_sequence: &Moves) {
        for m in move_sequence {
            self.do_move(*m);
        }
    }
}

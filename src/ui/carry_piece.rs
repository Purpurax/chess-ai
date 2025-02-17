use crate::core::{piece::Piece, position::Position};

pub struct CarryPiece {
    grabbed_position: Option<Position>,
    grabbed_piece: Option<Piece>,
}

impl CarryPiece {
    pub fn new() -> CarryPiece {
        CarryPiece {
            grabbed_position: Option::None,
            grabbed_piece: Option::None,
        }
    }

    pub fn set(&mut self, position: &Position, piece: &Piece) {
        self.grabbed_position = Some(position.clone());
        self.grabbed_piece = Some(piece.clone());
    }

    pub fn clear(&mut self) {
        self.grabbed_position = None;
        self.grabbed_piece = None;
    }

    pub fn position(&self) -> &Option<Position> {
        &self.grabbed_position
    }

    pub fn piece(&self) -> &Option<Piece> {
        &self.grabbed_piece
    }
}

use crate::core::{position::Position, piece::Piece};

pub struct CarryPiece {
    grabbed_position: Option<Position>,
    grabbed_piece: Option<Piece>
}

impl CarryPiece {
    pub fn new() -> CarryPiece {
        CarryPiece {
            grabbed_position: Option::None,
            grabbed_piece: Option::None
        }
    }

    pub fn set(&mut self, new_position: Position, new_piece: Piece) {
        self.grabbed_position = Some(new_position);
        self.grabbed_piece = Some(new_piece);
    }

    pub fn clear(&mut self) {
        self.grabbed_position = None;
        self.grabbed_piece = None;
    }

    pub fn is_empty(&self) -> bool {
        self.grabbed_position.is_none()
    }

    pub fn has_grabbed(&self) -> bool {
        !self.is_empty()
    }

    pub fn position(&self) -> Position {
        self.grabbed_position.clone().unwrap()
    }

    pub fn piece(&self) -> Piece {
        self.grabbed_piece.clone().unwrap()
    }
}
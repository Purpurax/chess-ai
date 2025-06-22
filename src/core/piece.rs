// Piece uses 4 bits with the first bit _ being the color
//      0b_000 = empty
//      0b_001 = pawn
//      0b_010 = knight
//      0b_011 = bishop
//      0b_100 = rook
//      0b_101 = queen
//      0b_110 = king
//      0b_111 = unclear yet
//
// OR MAYBE
//
// Pieces use 8 bits with:
//      0b0_______ = black
//      0b1_______ = white
//      0b_1______ = piece has been moved
//      0b__1_____ = pawn
//      0b___1____ = knight
//      0b____1___ = bishop
//      0b_____1__ = rook
//      0b______1_ = queen
//      0b_______1 = king

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    Empty,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, PartialEq)]
pub struct Piece {
    color: bool, // 0b0 black, 0b1 white
    piece_type: PieceType,
}

impl Piece {
    pub fn binary_to_piece(binary: u64) -> Piece {
        let color: bool = binary & 0b10000000 == 0b10000000;

        let piece_type: PieceType = if (binary & 0b00100000) == 0b00100000 {
            PieceType::Pawn
        } else if (binary & 0b00010000) == 0b00010000 {
            PieceType::Knight
        } else if (binary & 0b00001000) == 0b00001000 {
            PieceType::Bishop
        } else if (binary & 0b00000100) == 0b00000100 {
            PieceType::Rook
        } else if (binary & 0b00000010) == 0b00000010 {
            PieceType::Queen
        } else if (binary & 0b00000001) == 0b00000001 {
            PieceType::King
        } else {
            PieceType::Empty
        };

        Piece { color, piece_type }
    }

    pub fn get_color(&self) -> bool {
        self.color
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn to_char(&self) -> char {
        match <(bool, PieceType)>::from(self) {
            (_, PieceType::Empty) => ' ',
            (false, PieceType::Pawn) => '♟',
            (false, PieceType::Knight) => '♞',
            (false, PieceType::Bishop) => '♝',
            (false, PieceType::Rook) => '♜',
            (false, PieceType::Queen) => '♛',
            (false, PieceType::King) => '♚',
            (true, PieceType::Pawn) => '♙',
            (true, PieceType::Knight) => '♘',
            (true, PieceType::Bishop) => '♗',
            (true, PieceType::Rook) => '♖',
            (true, PieceType::Queen) => '♕',
            (true, PieceType::King) => '♔',
        }
    }
}

impl From<&Piece> for (bool, PieceType) {
    fn from(piece: &Piece) -> (bool, PieceType) {
        let Piece { color, piece_type } = piece;
        (*color, *piece_type)
    }
}

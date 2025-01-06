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
// Pieces use 7 bits with:
//      0b0______ = empty
//      0b1______ = has piece
//      0b_0_____ = black
//      0b_1_____ = white
//      0b__1____ = horizontal and vertical movement
//      0b___1___ = diagonal movement
//      0b____1__ = one step movement
//      0b_____1_ = hop movement (in L shapes)
//      0b______1 = pawn movement

#[derive(Clone, Copy)]
pub enum PieceType {
    Empty,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Piece {
    color: bool, // 0b0 black, 0b1 white
    piece_type: PieceType
}

impl Piece {
    pub fn binary_to_piece(binary: u64) -> Piece {
        let color: bool = binary & 0b0100000 == 0b0100000;
    
        let piece_type: PieceType = if (binary & 0b1000000) == 0b0 {
            PieceType::Empty
        } else if (binary & 0b0011100) == 0b0011100 {
            PieceType::Queen
        } else if (binary & 0b0010000) == 0b0010000 {
            PieceType::Rook
        } else if (binary & 0b0001000) == 0b0001000 {
            PieceType::Bishop
        } else if (binary & 0b0000100) == 0b0000100 {
            PieceType::King
        } else if (binary & 0b0000010) == 0b0000010 {
            PieceType::Knight
        } else {
            PieceType::Pawn
        };
    
        Piece { color, piece_type }
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

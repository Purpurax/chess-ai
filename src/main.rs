// use primitive_types::U256 as u256;

fn main() {
    let board: Board = Board::new();

    println!("{}", board.to_string());
}

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

struct Board {
    layer_empty: u64,
    layer_color: u64,
    layer_straight: u64,
    layer_diagonal: u64,
    layer_one_step: u64,
    layer_hop: u64,
    layer_pawn: u64
}

impl Board {
    #[allow(dead_code)]
    fn zero() -> Board {
        Board {
            layer_empty: 0b0,
            layer_color: 0b0,
            layer_straight: 0b0,
            layer_diagonal: 0b0,
            layer_one_step: 0b0,
            layer_hop: 0b0,
            layer_pawn: 0b0
        }
    }

    pub fn new() -> Board {
        Board {
            layer_empty:    0b1111111111111111000000000000000000000000000000001111111111111111,
            layer_color:    0b0000000000000000000000000000000000000000000000001111111111111111,
            layer_straight: 0b1000100100000000000000000000000000000000000000000000000010001001,
            layer_diagonal: 0b0010110000000000000000000000000000000000000000000000000000101100,
            layer_one_step: 0b0001100000000000000000000000000000000000000000000000000000011000,
            layer_hop:      0b0100001000000000000000000000000000000000000000000000000001000010,
            layer_pawn:     0b0000000011111111000000000000000000000000000000001111111100000000
        }
    }

    fn iterator<'a>(&'a self) -> impl Iterator<Item = u64> + 'a {
        (0..64).map(|i| {
            let mask: u64 = 0b1 << i;

            let mut piece: u64 = ((self.layer_empty & mask) >> i) << 6;
            piece |= ((self.layer_color & mask) >> i) << 5;
            piece |= ((self.layer_straight & mask) >> i) << 4;
            piece |= ((self.layer_diagonal & mask) >> i) << 3;
            piece |= ((self.layer_one_step & mask) >> i) << 2;
            piece |= ((self.layer_hop & mask) >> i) << 1;

            piece | ((self.layer_pawn & mask) >> i)
        })
    }

    pub fn to_string(&self) -> String {
        let mut column: u8 = 0;
        
        let delimiter: &str = " | ";
        let border: &str = " +---+---+---+---+---+---+---+---+ ";

        let mut result_string: String = String::from(border);
        result_string.push('\n');

        self.iterator().for_each(|binary_piece| {
            let piece: Piece = binary_to_piece(binary_piece);

            result_string.push_str(delimiter);
            result_string.push(piece.to_char());

            column += 1;
            if column == 8 {
                result_string.push_str(delimiter);
                result_string.push('\n');
                column = 0;
            }
        });
        result_string.push_str(border);

        result_string
    }

}

pub struct Piece {
    color: bool, // 0b0 black, 0b1 white
    piece_type: PieceType
}

impl Piece {
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

fn binary_to_piece(binary: u64) -> Piece {
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

use crate::core::{piece::Piece, position::Position};

use super::move_validator;

#[derive(Clone)]
pub struct Board {
    pub layer_color: u64, // 0 is black
    pub layer_pawn: u64,
    pub layer_knight: u64,
    pub layer_bishop: u64,
    pub layer_rook: u64,
    pub layer_king: u64,
    pub layer_queen: u64,
}

impl Board {
    #[allow(dead_code)]
    fn zero() -> Board {
        Board {
            layer_color: 0b0, // 0 is black
            layer_pawn: 0b0,
            layer_knight: 0b0,
            layer_bishop: 0b0,
            layer_rook: 0b0,
            layer_queen: 0b0,
            layer_king: 0b0,
        }
    }

    pub fn new() -> Board {
        Board {
            // layer_color:     0b0000000000000000000000000000000000000000000000001111111111111111,
            // layer_pawn:      0b0000000011111111000000000000000000000000000000001111111100000000,
            // layer_knight:    0b0100001000000000000000000000000000000000000000000000000001000010,
            // layer_bishop:    0b0010010000000000000000000000000000000000000000000000000000100100,
            // layer_rook:      0b1000000100000000000000000000000000000000000000000000000010000001,
            // layer_queen:     0b0000100000000000000000000000000000000000000000000000000000001000,
            // layer_king:      0b0001000000000000000000000000000000000000000000000000000000010000
            layer_color:     0b0000000000000000000000000000000000000000000000001111111111111111,
            layer_pawn:      0b0000000011111111000000000000000000000000000000001111111100000000,
            layer_knight:    0b0100001000000000000000000000000000000000000000000000000001000010,
            layer_bishop:    0b0010010000000000000000000000000000000000000000000000000000100100,
            layer_rook:      0b1000000100000000000000000000000000000000000000000000000010000001,
            layer_queen:     0b0000100000000000000000000000000000000000000000000000000000001000,
            layer_king:      0b0001000000000000000000000000000000000000000000000000000000010000
        }
    }

    pub fn import(layers: [u64; 7]) -> Board {
        Board {
            layer_color: layers[0],
            layer_pawn: layers[1],
            layer_knight: layers[2],
            layer_bishop: layers[3],
            layer_rook: layers[4],
            layer_queen: layers[5],
            layer_king: layers[6]
        }
    }

    pub fn export(&self) -> [u64; 7] {
        [
            self.layer_color,
            self.layer_pawn,
            self.layer_knight,
            self.layer_bishop,
            self.layer_rook,
            self.layer_queen,
            self.layer_king
        ]
    }

    fn get_piece_binary_at(&self, position: &Position) -> u64 {
        let shift_amount: u8 = position.row * 8 + position.column;
        let mask: u64 = 0b1 << shift_amount;

        let mut piece: u64 = ((self.layer_color  & mask) >> shift_amount) << 6;
        piece |= ((self.layer_pawn   & mask) >> shift_amount) << 5;
        piece |= ((self.layer_knight & mask) >> shift_amount) << 4;
        piece |= ((self.layer_bishop & mask) >> shift_amount) << 3;
        piece |= ((self.layer_rook   & mask) >> shift_amount) << 2;
        piece |= ((self.layer_queen  & mask) >> shift_amount) << 1;
        piece | ( self.layer_king   & mask) >> shift_amount
    }

    fn set_position_binary(&mut self, position: &Position, binary: u64) {
        let shift_amount: u8 = position.row * 8 + position.column;
        let mask: u64 = !(0b1 << shift_amount);

        self.layer_color  = (self.layer_color  & mask) | (((binary & 0b1000000) >> 6) << shift_amount);
        self.layer_pawn   = (self.layer_pawn   & mask) | (((binary & 0b0100000) >> 5) << shift_amount);
        self.layer_knight = (self.layer_knight & mask) | (((binary & 0b0010000) >> 4) << shift_amount);
        self.layer_bishop = (self.layer_bishop & mask) | (((binary & 0b0001000) >> 3) << shift_amount);
        self.layer_rook   = (self.layer_rook   & mask) | (((binary & 0b0000100) >> 2) << shift_amount);
        self.layer_queen  = (self.layer_queen  & mask) | (((binary & 0b0000010) >> 1) << shift_amount);
        self.layer_king   = (self.layer_king   & mask) | ((binary & 0b0000001) << shift_amount);
    }

    pub fn get_piece_at(&self, position: &Position) -> Piece {
        let binary_piece: u64 = self.get_piece_binary_at(position);
        Piece::binary_to_piece(binary_piece)
    }

    pub fn get_layer_value_at(layer: u64, position: &Position) -> bool {
        let shift_amount: u8 = position.row * 8 + position.column;
        let mask: u64 = 0b1 << shift_amount;

        (layer & mask) == mask
    }

    pub fn get_empty_layer(&self) -> u64 {
        !(self.layer_pawn   | self.layer_knight
        | self.layer_bishop | self.layer_rook
        | self.layer_queen  | self.layer_king)
    }

    pub fn iterator<'a>(&'a self) -> impl Iterator<Item = u64> + 'a {
        (0..64).map(|i| {
            let mask: u64 = 0b1 << i;

            let mut piece: u64 = ((self.layer_color & mask) >> i) << 6;
            piece |= ((self.layer_pawn & mask) >> i) << 5;
            piece |= ((self.layer_knight & mask) >> i) << 4;
            piece |= ((self.layer_bishop & mask) >> i) << 3;
            piece |= ((self.layer_rook & mask) >> i) << 2;
            piece |= ((self.layer_queen & mask) >> i) << 1;

            piece | ((self.layer_king & mask) >> i)
        })
    }

    pub fn iterator_pieces<'a>(&'a self) -> impl Iterator<Item = Piece> + 'a {
        self.iterator().map(|binary_piece| {
            Piece::binary_to_piece(binary_piece)
        })
    }

    pub fn iterator_positions_and_pieces<'a>(&'a self) -> impl Iterator<Item = (Position, Piece)> + 'a {
        self.iterator_pieces().enumerate().map(|(i, piece)| {
            let row: u8 = i.div_euclid(8) as u8;
            let column: u8 = i.rem_euclid(8) as u8;
            
            (Position::new(row, column), piece)
        })
    }

    pub fn to_string(&self) -> String {
        let mut column: u8 = 0;
        
        let delimiter: &str = " | ";
        let border: &str = " +---+---+---+---+---+---+---+---+ ";

        let mut result_string: String = String::from(border);
        result_string.push('\n');

        self.iterator_pieces().for_each(|piece| {
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

    pub fn is_move_valid(&self, player_turn: bool, from: &Position, to: &Position) -> bool {
        return move_validator::is_move_valid(&self, player_turn, from, to, true);
    }

    pub fn move_from_to(&mut self, from: &Position, to: &Position) {
        let binary_piece: u64 = self.get_piece_binary_at(&from);

        self.set_position_binary(from, 0b0);
        self.set_position_binary(to, binary_piece);
    }
}
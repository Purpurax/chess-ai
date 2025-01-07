use crate::core::{piece::Piece, position::Position};

pub struct Board {
    layer_empty: u64, // 0 is empty
    layer_color: u64, // 0 is black
    layer_straight: u64,
    layer_diagonal: u64,
    layer_one_step: u64,
    layer_hop: u64,
    layer_pawn: u64,
    layer_has_moved: u64
}

impl Board {
    #[allow(dead_code)]
    fn zero() -> Board {
        Board {
            layer_empty:     0b0,
            layer_color:     0b0,
            layer_straight:  0b0,
            layer_diagonal:  0b0,
            layer_one_step:  0b0,
            layer_hop:       0b0,
            layer_pawn:      0b0,
            layer_has_moved: 0b0
        }
    }

    pub fn new() -> Board {
        Board {
            layer_empty:     0b1111111111111111000000000000000000000000000000001111111111111111,
            layer_color:     0b0000000000000000000000000000000000000000000000001111111111111111,
            layer_straight:  0b1000100100000000000000000000000000000000000000000000000010001001,
            layer_diagonal:  0b0010110000000000000000000000000000000000000000000000000000101100,
            layer_one_step:  0b0001100000000000000000000000000000000000000000000000000000011000,
            layer_hop:       0b0100001000000000000000000000000000000000000000000000000001000010,
            layer_pawn:      0b0000000011111111000000000000000000000000000000001111111100000000,
            layer_has_moved: 0b0000000000000000000000000000000000000000000000000000000000000000
        }
    }

    fn set_position_binary(&mut self, position: &Position, binary: u64) {
        let shift_amount: u8 = position.row * 8 + position.column;
        let mask: u64 = !(0b1 << shift_amount);

        self.layer_empty = (self.layer_empty & mask) | (((binary & 0b1000000) >> 6) << shift_amount);
        self.layer_color = (self.layer_color & mask) | (((binary & 0b100000) >> 5) << shift_amount);
        self.layer_straight = (self.layer_straight & mask) | (((binary & 0b10000) >> 4) << shift_amount);
        self.layer_diagonal = (self.layer_diagonal & mask) | (((binary & 0b1000) >> 3) << shift_amount);
        self.layer_one_step = (self.layer_one_step & mask) | (((binary & 0b100) >> 2) << shift_amount);
        self.layer_hop = (self.layer_hop & mask) | (((binary & 0b10) >> 1) << shift_amount);
        self.layer_pawn = (self.layer_pawn & mask) | ((binary & 0b1) << shift_amount);
    }

    fn get_piece_binary_at(&self, position: &Position) -> u64 {
        let shift_amount: u8 = position.row * 8 + position.column;
        let mask: u64 = 0b1 << shift_amount;

        let mut piece: u64 = ((self.layer_empty & mask) >> shift_amount) << 6;
        piece |= ((self.layer_color & mask) >> shift_amount) << 5;
        piece |= ((self.layer_straight & mask) >> shift_amount) << 4;
        piece |= ((self.layer_diagonal & mask) >> shift_amount) << 3;
        piece |= ((self.layer_one_step & mask) >> shift_amount) << 2;
        piece |= ((self.layer_hop & mask) >> shift_amount) << 1;
        piece | (self.layer_pawn & mask) >> shift_amount
    }

    pub fn get_piece_at(&self, position: &Position) -> Piece {
        let binary_piece: u64 = self.get_piece_binary_at(position);
        Piece::binary_to_piece(binary_piece)
    }

    fn get_layer_value_at(layer: &u64, position: &Position) -> bool {
        let shift_amount: u8 = position.row * 8 + position.column;
        let mask: u64 = 0b1 << shift_amount;

        (layer & mask) == mask
    }

    pub fn iterator<'a>(&'a self) -> impl Iterator<Item = u64> + 'a {
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

    pub fn iterator_pieces<'a>(&'a self) -> impl Iterator<Item = Piece> + 'a {
        self.iterator().map(|binary_piece| {
            Piece::binary_to_piece(binary_piece)
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

    fn is_path_clear(&self, from: Position, to: Position) -> bool {
        // straight path
        if from.row == to.row || from.column == to.column {
            // let difference: u8 = from - to;

            // if difference <= 1 {
            //     return true
            // }
            let mut position_check: Position = from.clone();
            position_check.move_towards(&to);

            while position_check != to {
                if Self::get_layer_value_at(&self.layer_empty, &position_check) {
                    return false
                }

                position_check.move_towards(&to);
            }

            return true
        }

        return false
    }

    pub fn is_move_valid(&self, player_turn: bool, from: Position, to: Position) -> bool {
        // position validity check
        if from.row > 7 || from.column > 7 || to.row > 7 || to.column > 7 || from == to {
            return false
        }
        
        // color check
        if Self::get_layer_value_at(&self.layer_color, &from) != player_turn
        || (Self::get_layer_value_at(&self.layer_empty, &to)
            && Self::get_layer_value_at(&self.layer_color, &to) == player_turn) {
            return false
        }

        // piece specific move check
        // straight move
        if Self::get_layer_value_at(&self.layer_straight, &from) && (from.row == to.row || from.column == to.column) {
            return self.is_path_clear(from, to)
        }
        // diagonal move
        if Self::get_layer_value_at(&self.layer_diagonal, &from) && (from.row - to.row == from.column - to.column) {
            return self.is_path_clear(from, to)
        }
        // one step move
        if Self::get_layer_value_at(&self.layer_one_step, &from) && (&from - &to == 1) {
            return true
        }
        // hop move
        if Self::get_layer_value_at(&self.layer_hop, &from)
        && (from.row.abs_diff(to.row) == 2 && from.column.abs_diff(to.column) == 1)
        && (from.row.abs_diff(to.row) == 1 && from.column.abs_diff(to.column) == 2) {
            return true
        }
        // pawn move
        // TODO: en pasant rule
        if Self::get_layer_value_at(&self.layer_pawn, &from) {
            // let moving_back: bool = ;
            // let attack_move: bool = ;


            // empty in front
            if Self::get_layer_value_at(&self.layer_empty, &to) {
                if !Self::get_layer_value_at(&self.layer_has_moved, &from) {

                } else {

                }
            } else {
                if !Self::get_layer_value_at(&self.layer_has_moved, &from) {
                    if player_turn && (from.row + 1 == to.row || from.row + 2 == to.row) && from.column == to.column
                    && !player_turn && (from.row == to.row + 1 || from.row == to.row + 2) && from.column == to.column {
                        return true
                    }
                } else {
                    if player_turn && from.row + 1 == to.row && from.column == to.column
                    && !player_turn && from.row == to.row + 1 && from.column == to.column {
                        return true
                    }
                }
            }
            return true
        }

        return false
    }

    pub fn move_from_to(&mut self, from: &Position, to: &Position) {
        let binary_piece: u64 = self.get_piece_binary_at(&from);

        self.set_position_binary(from, 0b0);
        self.set_position_binary(to, binary_piece);
    }
}
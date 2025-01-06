use crate::piece::Piece;

pub struct Board {
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
            let piece: Piece = Piece::binary_to_piece(binary_piece);

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
use crate::core::{board::Board, move_generator::{get_all_possible_moves, has_possible_moves}, piece::{Piece, PieceType}, position::Position};


pub fn is_move_valid(
    board: &Board,
    player_turn: bool,
    from: &Position,
    to: &Position
) -> bool {
    if !is_position_on_board(from) || !is_position_on_board(to) || from == to {
        return false
    }

    let from_piece: Piece = board.get_piece_at(from);
    let to_piece: Piece = board.get_piece_at(to);

    if !are_pieces_valid(&from_piece, &to_piece, player_turn) {
        return false
    }

    if !is_path_clear(board, from, to) && from_piece.piece_type() != PieceType::Knight {
        return false
    }

    if !match from_piece.piece_type() {
        PieceType::Empty => false,
        PieceType::Pawn => is_valid_pawn_move(&to_piece, player_turn, from, to),
        PieceType::Knight => is_valid_knight_move(from, to),
        PieceType::Bishop => is_valid_bishop_move(from, to),
        PieceType::Rook => is_valid_rook_move(from, to),
        PieceType::Queen => is_valid_queen_move(from, to),
        PieceType::King => is_valid_king_move(board.layer_not_moved, from, to),
    } {
        return false
    }

    let mut applied_board: Board = board.clone();
    applied_board.move_from_to(from, to);

    !is_check(&applied_board, !player_turn)
}

pub fn is_check(board: &Board, player_turn: bool) -> bool {
    let king_layer: u64 = if player_turn {
        (!board.layer_color) & board.layer_king
    } else {
        board.layer_color & board.layer_king
    };
    if king_layer == 0b0 {
        return true
    }
    let king_index: u32 = king_layer.ilog2();

    is_checked_by_pawn(
        king_index,
        player_turn,
        board.layer_color,
        board.layer_pawn
    ) || is_checked_by_knight(
        king_index,
        if player_turn {
            board.layer_color & board.layer_knight
        } else {
            (!board.layer_color) & board.layer_knight
        }
    ) || is_checked_in_diagonal_line(
        king_layer,
        king_index,
        if player_turn {
            board.layer_color & (board.layer_bishop | board.layer_queen)
        } else {
            (!board.layer_color) & (board.layer_bishop | board.layer_queen)
        },
        board.layer_pawn
            | board.layer_knight
            | board.layer_bishop
            | board.layer_rook
            | board.layer_queen
            | board.layer_king
    ) || is_checked_in_straight_line(
        king_layer,
        king_index,
        if player_turn {
            board.layer_color & (board.layer_rook | board.layer_queen)
        } else {
            (!board.layer_color) & (board.layer_rook | board.layer_queen)
        },
        board.layer_pawn
            | board.layer_knight
            | board.layer_bishop
            | board.layer_rook
            | board.layer_queen
            | board.layer_king
    )
}

fn is_checked_by_pawn(
    king_index: u32,
    player_turn: bool,
    layer_color: u64,
    layer_pawn: u64
) -> bool {
    let mask: u64 = if player_turn {
        match king_index {
            0 => 0b0000000000000000000000000000000000000000000000000000001000000000,
            1 => 0b0000000000000000000000000000000000000000000000000000010100000000,
            2 => 0b0000000000000000000000000000000000000000000000000000101000000000,
            3 => 0b0000000000000000000000000000000000000000000000000001010000000000,
            4 => 0b0000000000000000000000000000000000000000000000000010100000000000,
            5 => 0b0000000000000000000000000000000000000000000000000101000000000000,
            6 => 0b0000000000000000000000000000000000000000000000001010000000000000,
            7 => 0b0000000000000000000000000000000000000000000000000100000000000000,
            8 => 0b0000000000000000000000000000000000000000000000100000000000000000,
            9 => 0b0000000000000000000000000000000000000000000001010000000000000000,
            10 => 0b0000000000000000000000000000000000000000000010100000000000000000,
            11 => 0b0000000000000000000000000000000000000000000101000000000000000000,
            12 => 0b0000000000000000000000000000000000000000001010000000000000000000,
            13 => 0b0000000000000000000000000000000000000000010100000000000000000000,
            14 => 0b0000000000000000000000000000000000000000101000000000000000000000,
            15 => 0b0000000000000000000000000000000000000000010000000000000000000000,
            16 => 0b0000000000000000000000000000000000000010000000000000000000000000,
            17 => 0b0000000000000000000000000000000000000101000000000000000000000000,
            18 => 0b0000000000000000000000000000000000001010000000000000000000000000,
            19 => 0b0000000000000000000000000000000000010100000000000000000000000000,
            20 => 0b0000000000000000000000000000000000101000000000000000000000000000,
            21 => 0b0000000000000000000000000000000001010000000000000000000000000000,
            22 => 0b0000000000000000000000000000000010100000000000000000000000000000,
            23 => 0b0000000000000000000000000000000001000000000000000000000000000000,
            24 => 0b0000000000000000000000000000001000000000000000000000000000000000,
            25 => 0b0000000000000000000000000000010100000000000000000000000000000000,
            26 => 0b0000000000000000000000000000101000000000000000000000000000000000,
            27 => 0b0000000000000000000000000001010000000000000000000000000000000000,
            28 => 0b0000000000000000000000000010100000000000000000000000000000000000,
            29 => 0b0000000000000000000000000101000000000000000000000000000000000000,
            30 => 0b0000000000000000000000001010000000000000000000000000000000000000,
            31 => 0b0000000000000000000000000100000000000000000000000000000000000000,
            32 => 0b0000000000000000000000100000000000000000000000000000000000000000,
            33 => 0b0000000000000000000001010000000000000000000000000000000000000000,
            34 => 0b0000000000000000000010100000000000000000000000000000000000000000,
            35 => 0b0000000000000000000101000000000000000000000000000000000000000000,
            36 => 0b0000000000000000001010000000000000000000000000000000000000000000,
            37 => 0b0000000000000000010100000000000000000000000000000000000000000000,
            38 => 0b0000000000000000101000000000000000000000000000000000000000000000,
            39 => 0b0000000000000000010000000000000000000000000000000000000000000000,
            40 => 0b0000000000000010000000000000000000000000000000000000000000000000,
            41 => 0b0000000000000101000000000000000000000000000000000000000000000000,
            42 => 0b0000000000001010000000000000000000000000000000000000000000000000,
            43 => 0b0000000000010100000000000000000000000000000000000000000000000000,
            44 => 0b0000000000101000000000000000000000000000000000000000000000000000,
            45 => 0b0000000001010000000000000000000000000000000000000000000000000000,
            46 => 0b0000000010100000000000000000000000000000000000000000000000000000,
            47 => 0b0000000001000000000000000000000000000000000000000000000000000000,
            48 => 0b0000001000000000000000000000000000000000000000000000000000000000,
            49 => 0b0000010100000000000000000000000000000000000000000000000000000000,
            50 => 0b0000101000000000000000000000000000000000000000000000000000000000,
            51 => 0b0001010000000000000000000000000000000000000000000000000000000000,
            52 => 0b0010100000000000000000000000000000000000000000000000000000000000,
            53 => 0b0101000000000000000000000000000000000000000000000000000000000000,
            54 => 0b1010000000000000000000000000000000000000000000000000000000000000,
            55 => 0b0100000000000000000000000000000000000000000000000000000000000000,
            _ => 0b0
        }
    } else {
        match king_index {
            8 => 0b0000000000000000000000000000000000000000000000000000000000000010,
            9 => 0b0000000000000000000000000000000000000000000000000000000000000101,
            10 => 0b0000000000000000000000000000000000000000000000000000000000001010,
            11 => 0b0000000000000000000000000000000000000000000000000000000000010100,
            12 => 0b0000000000000000000000000000000000000000000000000000000000101000,
            13 => 0b0000000000000000000000000000000000000000000000000000000001010000,
            14 => 0b0000000000000000000000000000000000000000000000000000000010100000,
            15 => 0b0000000000000000000000000000000000000000000000000000000001000000,
            16 => 0b0000000000000000000000000000000000000000000000000000001000000000,
            17 => 0b0000000000000000000000000000000000000000000000000000010100000000,
            18 => 0b0000000000000000000000000000000000000000000000000000101000000000,
            19 => 0b0000000000000000000000000000000000000000000000000001010000000000,
            20 => 0b0000000000000000000000000000000000000000000000000010100000000000,
            21 => 0b0000000000000000000000000000000000000000000000000101000000000000,
            22 => 0b0000000000000000000000000000000000000000000000001010000000000000,
            23 => 0b0000000000000000000000000000000000000000000000000100000000000000,
            24 => 0b0000000000000000000000000000000000000000000000100000000000000000,
            25 => 0b0000000000000000000000000000000000000000000001010000000000000000,
            26 => 0b0000000000000000000000000000000000000000000010100000000000000000,
            27 => 0b0000000000000000000000000000000000000000000101000000000000000000,
            28 => 0b0000000000000000000000000000000000000000001010000000000000000000,
            29 => 0b0000000000000000000000000000000000000000010100000000000000000000,
            30 => 0b0000000000000000000000000000000000000000101000000000000000000000,
            31 => 0b0000000000000000000000000000000000000000010000000000000000000000,
            32 => 0b0000000000000000000000000000000000000010000000000000000000000000,
            33 => 0b0000000000000000000000000000000000000101000000000000000000000000,
            34 => 0b0000000000000000000000000000000000001010000000000000000000000000,
            35 => 0b0000000000000000000000000000000000010100000000000000000000000000,
            36 => 0b0000000000000000000000000000000000101000000000000000000000000000,
            37 => 0b0000000000000000000000000000000001010000000000000000000000000000,
            38 => 0b0000000000000000000000000000000010100000000000000000000000000000,
            39 => 0b0000000000000000000000000000000001000000000000000000000000000000,
            40 => 0b0000000000000000000000000000001000000000000000000000000000000000,
            41 => 0b0000000000000000000000000000010100000000000000000000000000000000,
            42 => 0b0000000000000000000000000000101000000000000000000000000000000000,
            43 => 0b0000000000000000000000000001010000000000000000000000000000000000,
            44 => 0b0000000000000000000000000010100000000000000000000000000000000000,
            45 => 0b0000000000000000000000000101000000000000000000000000000000000000,
            46 => 0b0000000000000000000000001010000000000000000000000000000000000000,
            47 => 0b0000000000000000000000000100000000000000000000000000000000000000,
            48 => 0b0000000000000000000000100000000000000000000000000000000000000000,
            49 => 0b0000000000000000000001010000000000000000000000000000000000000000,
            50 => 0b0000000000000000000010100000000000000000000000000000000000000000,
            51 => 0b0000000000000000000101000000000000000000000000000000000000000000,
            52 => 0b0000000000000000001010000000000000000000000000000000000000000000,
            53 => 0b0000000000000000010100000000000000000000000000000000000000000000,
            54 => 0b0000000000000000101000000000000000000000000000000000000000000000,
            55 => 0b0000000000000000010000000000000000000000000000000000000000000000,
            56 => 0b0000000000000010000000000000000000000000000000000000000000000000,
            57 => 0b0000000000000101000000000000000000000000000000000000000000000000,
            58 => 0b0000000000001010000000000000000000000000000000000000000000000000,
            59 => 0b0000000000010100000000000000000000000000000000000000000000000000,
            60 => 0b0000000000101000000000000000000000000000000000000000000000000000,
            61 => 0b0000000001010000000000000000000000000000000000000000000000000000,
            62 => 0b0000000010100000000000000000000000000000000000000000000000000000,
            63 => 0b0000000001000000000000000000000000000000000000000000000000000000,
            _ => 0b0
        }
    };

    mask & layer_color & layer_pawn != 0b0
}

fn is_checked_by_knight(king_index: u32, knight_layer: u64) -> bool {
    let mask: u64 = match king_index {
        00 => 0b0000000000000000000000000000000000000000000000100000010001000000,
        01 => 0b0000000000000000000000000000000000000000000001010000100000000000,
        02 => 0b0000000000000000000000000000000000000000000010100001000000000000,
        03 => 0b0000000000000000000000000000000000000000000101000010000000000000,
        04 => 0b0000000000000000000000000000000000000000001010000100000000000000,
        05 => 0b0000000000000000000000000000000000000000010100001000000000000000,
        06 => 0b0000000000000000000000000000000000000000101000000000000000000000,
        07 => 0b0000000000000000000000000000000000000000010000000000000000000000,
        08 => 0b0000000000000000000000000000000000000010000001000100000000000100,
        09 => 0b0000000000000000000000000000000000000101000010000000000000001000,
        10 => 0b0000000000000000000000000000000000001010000100000000000000010001,
        11 => 0b0000000000000000000000000000000000010100001000000000000000100010,
        12 => 0b0000000000000000000000000000000000101000010000000000000001000100,
        13 => 0b0000000000000000000000000000000001010000100000000000000010001000,
        14 => 0b0000000000000000000000000000000010100000000000000000000000010000,
        15 => 0b0000000000000000000000000000000001000000000000000000000000100000,
        16 => 0b0000000000000000000000000000001000000100010000000000010000000010,
        17 => 0b0000000000000000000000000000010100001000000000000000100000000101,
        18 => 0b0000000000000000000000000000101000010000000000000001000100001010,
        19 => 0b0000000000000000000000000001010000100000000000000010001000010100,
        20 => 0b0000000000000000000000000010100001000000000000000100010000101000,
        21 => 0b0000000000000000000000000101000010000000000000001000100001010000,
        22 => 0b0000000000000000000000001010000000000000000000000001000010100000,
        23 => 0b0000000000000000000000000100000000000000000000000010000001000000,
        24 => 0b0000000000000000000000100000010001000000000001000000001000000000,
        25 => 0b0000000000000000000001010000100000000000000010000000010100000000,
        26 => 0b0000000000000000000010100001000000000000000100010000101000000000,
        27 => 0b0000000000000000000101000010000000000000001000100001010000000000,
        28 => 0b0000000000000000001010000100000000000000010001000010100000000000,
        29 => 0b0000000000000000010100001000000000000000100010000101000000000000,
        30 => 0b0000000000000000101000000000000000000000000100001010000000000000,
        31 => 0b0000000000000000010000000000000000000000001000000100000000000000,
        32 => 0b0000000000000010000001000100000000000100000000100000000000000000,
        33 => 0b0000000000000101000010000000000000001000000001010000000000000000,
        34 => 0b0000000000001010000100000000000000010001000010100000000000000000,
        35 => 0b0000000000010100001000000000000000100010000101000000000000000000,
        36 => 0b0000000000101000010000000000000001000100001010000000000000000000,
        37 => 0b0000000001010000100000000000000010001000010100000000000000000000,
        38 => 0b0000000010100000000000000000000000010000101000000000000000000000,
        39 => 0b0000000001000000000000000000000000100000010000000000000000000000,
        40 => 0b0000001000000100010000000000010000000010000000000000000000000000,
        41 => 0b0000010100001000000000000000100000000101000000000000000000000000,
        42 => 0b0000101000010000000000000001000100001010000000000000000000000000,
        43 => 0b0001010000100000000000000010001000010100000000000000000000000000,
        44 => 0b0010100001000000000000000100010000101000000000000000000000000000,
        45 => 0b0101000010000000000000001000100001010000000000000000000000000000,
        46 => 0b1010000000000000000000000001000010100000000000000000000000000000,
        47 => 0b0100000000000000000000000010000001000000000000000000000000000000,
        48 => 0b0000010001000000000001000000001000000000000000000000000000000000,
        49 => 0b0000100000000000000010000000010100000000000000000000000000000000,
        50 => 0b0001000000000000000100010000101000000000000000000000000000000000,
        51 => 0b0010000000000000001000100001010000000000000000000000000000000000,
        52 => 0b0100000000000000010001000010100000000000000000000000000000000000,
        53 => 0b1000000000000000100010000101000000000000000000000000000000000000,
        54 => 0b0000000000000000000100001010000000000000000000000000000000000000,
        55 => 0b0000000000000000001000000100000000000000000000000000000000000000,
        56 => 0b0000000000000100000000100000000000000000000000000000000000000000,
        57 => 0b0000000000001000000001010000000000000000000000000000000000000000,
        58 => 0b0000000000010001000010100000000000000000000000000000000000000000,
        59 => 0b0000000000100010000101000000000000000000000000000000000000000000,
        60 => 0b0000000001000100001010000000000000000000000000000000000000000000,
        61 => 0b0000000010001000010100000000000000000000000000000000000000000000,
        62 => 0b0000000000010000101000000000000000000000000000000000000000000000,
        _ => 0b0000000000100000010000000000000000000000000000000000000000000000
    };
    
    mask & knight_layer != 0b0
}

fn is_checked_in_diagonal_line(
    king_layer: u64,
    king_index: u32,
    pieces_layer: u64,
    all_pieces_layer: u64
) -> bool {
    let mut square_to_check: u64 = king_layer;
    let mut square_index: u32 = king_index;
    while square_index % 8 > 0 && square_index > 7 {
        square_to_check >>= 9; // up-left
        square_index -= 9;

        if pieces_layer & square_to_check != 0b0 {
            return true
        }
        if all_pieces_layer & square_to_check != 0b0 {
            break
        }
    }

    square_to_check = king_layer;
    square_index = king_index;
    while square_index % 8 < 7 && square_index > 7 {
        square_to_check >>= 7; // up-right
        square_index -= 7;

        if pieces_layer & square_to_check != 0b0 {
            return true
        }
        if all_pieces_layer & square_to_check != 0b0 {
            break
        }
    }

    square_to_check = king_layer;
    square_index = king_index;
    while square_index % 8 > 0 && square_index < 56 {
        square_to_check <<= 7; // down-left
        square_index += 7;

        if pieces_layer & square_to_check != 0b0 {
            return true
        }
        if all_pieces_layer & square_to_check != 0b0 {
            break
        }
    }

    square_to_check = king_layer;
    square_index = king_index;
    while square_index % 8 < 7 && square_index < 56 {
        square_to_check <<= 9; // down-left
        square_index += 9;

        if pieces_layer & square_to_check != 0b0 {
            return true
        }
        if all_pieces_layer & square_to_check != 0b0 {
            break
        }
    }


    false
}

fn is_checked_in_straight_line(
    king_layer: u64,
    king_index: u32,
    pieces_layer: u64,
    all_pieces_layer: u64
) -> bool {
    let mut square_to_check: u64 = king_layer;
    while square_to_check != 0b0 {
        square_to_check >>= 8; // up

        if pieces_layer & square_to_check != 0b0 {
            return true
        }
        if all_pieces_layer & square_to_check != 0b0 {
            break
        }
    }

    square_to_check = king_layer;
    while square_to_check != 0b0 {
        square_to_check <<= 8; // down

        if pieces_layer & square_to_check != 0b0 {
            return true
        }
        if all_pieces_layer & square_to_check != 0b0 {
            break
        }
    }

    let mut square_index: u32 = king_index;
    square_to_check = king_layer;
    while square_index % 8 > 0 {
        square_to_check >>= 1; // left
        square_index -= 1;

        if pieces_layer & square_to_check != 0b0 {
            return true
        }
        if all_pieces_layer & square_to_check != 0b0 {
            break
        }
    }

    square_index = king_index;
    square_to_check = king_layer;
    while square_index % 8 < 7 {
        square_to_check <<= 1; // right
        square_index += 1;

        if pieces_layer & square_to_check != 0b0 {
            return true
        }
        if all_pieces_layer & square_to_check != 0b0 {
            break
        }
    }

    false
}

pub fn is_checkmate(board: &Board, player_turn: bool) -> bool {
    get_all_possible_moves(board, !player_turn).into_iter()
        .map(|(from, to)| {
            let mut new_board: Board = board.clone();
            new_board.move_from_to(&from, &to);
            new_board
        })
        .all(|new_board| is_check(&new_board, player_turn))
}

// Possible draws:
//  - Stalemate (unchecked, without having any move left)
//  - Dead position
//      - King vs King
//      - King, Bishop vs King
//      - King, Knight vs King
//      - King, Bishop vs King, Bishop (with the same color Bishop)
//  - Mutual Agreement
//  - Threefold Repitition
//  - 50-move rule (50 moves without a capture or pawn move)
pub fn is_remis(board: &Board, player_turn: bool) -> bool {
    !board.clone().iterator_positions_and_pieces()
        .filter_map(|(pos, piece)| {
            if piece.get_color() != player_turn {
                Some(pos)
            } else {
                None
            }
        })
        .any(|from_pos| {
            has_possible_moves(board, !player_turn, &from_pos)
        })
}

fn is_position_on_board(position: &Position) -> bool {
    position.row <= 7 && position.column <= 7
}

fn are_pieces_valid(from_piece: &Piece, to_piece: &Piece, player_turn: bool) -> bool {
    from_piece.piece_type() != PieceType::Empty
        && from_piece.get_color() == player_turn
        && (to_piece.piece_type() == PieceType::Empty || to_piece.get_color() != player_turn)
}

fn is_path_clear(board: &Board, from: &Position, to: &Position) -> bool {
    let layer_occupied: u64 = !board.get_empty_layer();

    let mut position_check: Position = from.clone();
    position_check.move_towards(to);

    while position_check != *to {
        if Board::get_layer_value_at(layer_occupied, &position_check) {
            return false;
        }

        position_check.move_towards(to);
    }

    true
}

fn is_valid_pawn_move(
    to_piece: &Piece,
    player_turn: bool,
    from: &Position,
    to: &Position
) -> bool {
    to_piece.piece_type() == PieceType::Empty
        && (player_turn && (from.row + 1 == to.row && from.column == to.column)
            || !player_turn && (from.row == to.row + 1 && from.column == to.column))
    || (to_piece.piece_type() == PieceType::Empty
        && (player_turn && (from.row == 1 && to.row == 3 && from.column == to.column)
            || !player_turn && (from.row == 6 && to.row == 4 && from.column == to.column)))
    || (to_piece.piece_type() != PieceType::Empty
        && (player_turn && (from.row + 1 == to.row && from.column.abs_diff(to.column) == 1)
            || !player_turn && (from.row == to.row + 1 && from.column.abs_diff(to.column) == 1)))
    // add en pasant, but honestly just leave as it is because that much requires:
    // 1. The capturing pawn must have advanced exactly three ranks to perform this move.
    // 2. The captured pawn must have moved two squares in one move, landing right next to the capturing pawn.
    // 3. The en passant capture must be performed on the turn immediately after the pawn being captured moves. If the player does not capture en passant on that turn, they no longer can do it later.
}

fn is_valid_knight_move(from: &Position, to: &Position) -> bool {
    (from.row.abs_diff(to.row) == 2 && from.column.abs_diff(to.column) == 1)
        || (from.row.abs_diff(to.row) == 1 && from.column.abs_diff(to.column) == 2)
}

fn is_valid_bishop_move(from: &Position, to: &Position) -> bool {
    from.row.abs_diff(to.row) == from.column.abs_diff(to.column)
}

fn is_valid_rook_move(from: &Position, to: &Position) -> bool {
    from.row == to.row || from.column == to.column
}

fn is_valid_queen_move(from: &Position, to: &Position) -> bool {
    from.row.abs_diff(to.row) == from.column.abs_diff(to.column)
        || from.row == to.row
        || from.column == to.column
}

fn is_valid_king_move(layer_not_moved: u64, from: &Position, to: &Position) -> bool {
    let king_not_moved: bool = Board::get_layer_value_at(layer_not_moved, from);
    let rook_not_moved: bool =
        from.column < to.column && Board::get_layer_value_at(layer_not_moved, &Position { row: from.row, column: 7 })
        || from.column > to.column && Board::get_layer_value_at(layer_not_moved, &Position { row: from.row, column: 0 });
    
    king_not_moved && rook_not_moved && from.row.abs_diff(to.row) == 0 && from.column.abs_diff(to.column) == 2
    || from.row.abs_diff(to.row) <= 1 && from.column.abs_diff(to.column) <= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Instant;

    #[test]
    fn time_test() {
        let king_index: u32 = 28;
        let player_turn: bool = true;
        let mut board = Board::new();
        
        board.layer_color = 0b1111111100000000000000000000000011111111000000000000000000000000;
        board.layer_pawn = 0b0000000011111111000000000000000000000000111111110000000000000000;
        board.layer_rook = 0b1000000100000000000000000000000010000001000000000000000000000000;
        board.layer_knight = 0b0100001000000000000000000000000001000010000000000000000000000000;
        board.layer_bishop = 0b0010010000000000000000000000000000100100000000000000000000000000;
        board.layer_queen = 0b0000100000000000000000000000000000001000000000000000000000000000;
        board.layer_king = 0b0001000000000000000000000000000000010000000000000000000000000000;

        let iterations = 100000000; // 1.14
        
        // Time the optimized function
        let start = Instant::now();
        for _ in 0..iterations {
            is_checked_by_pawn(king_index, player_turn, board.layer_color, board.layer_pawn);
        }
        let optimized_duration = start.elapsed();
        
        // Time the hard-coded function (same logic for comparison)
        // let start = Instant::now();
        // for _ in 0..iterations {
        //     hard_coded_is_checked_by_knight(king_layer, king_index, knight_layer);
        // }
        // let hardcoded_duration = start.elapsed();
        
        println!("Optimized function: {:?}", optimized_duration);
        // println!("Hard-coded function: {:?}", hardcoded_duration);
        // println!("Ratio: {:.2}", optimized_duration.as_nanos() as f64 / hardcoded_duration.as_nanos() as f64);
    }

    #[test]
    fn testi() {
        let player_turn: bool = false;
        for i in 0..64 {
            let king_layer: u64 = 0b1 << i;

            let mut pawn_mask: u64 = 0b0;

            if player_turn && i < 56 {

                if i % 8 > 0 {
                    pawn_mask |= king_layer << 7;
                }
                if i % 8 < 7 {
                    pawn_mask |= king_layer << 9;
                }
            } else if !player_turn && i > 7 {
                if i % 8 > 0 {
                    pawn_mask |= king_layer >> 9;
                }
                if i % 8 < 7 {
                    pawn_mask |= king_layer >> 7;
                }
            }

            println!("{} => {:#066b},", i, pawn_mask);
        }
    }
}

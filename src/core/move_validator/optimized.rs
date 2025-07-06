use crate::core::{board::Board, move_generator::{get_all_possible_moves, get_possible_moves, has_possible_moves}, piece::{Piece, PieceType}, position::Position};


pub fn is_move_valid(
    board: &Board,
    player_turn: bool,
    from: &Position,
    to: &Position,
    checking_check: bool
) -> bool {
    if !is_position_on_board(from) || !is_position_on_board(to) || from == to {
        return false;
    }

    let from_piece: Piece = board.get_piece_at(from);
    let to_piece: Piece = board.get_piece_at(to);

    if !are_pieces_valid(&from_piece, &to_piece, player_turn) {
        return false;
    }

    if !is_path_clear(board, from, to) && from_piece.piece_type() != PieceType::Knight {
        return false;
    }

    if checking_check {
        let mut applied_board: Board = board.clone();
        applied_board.move_from_to(from, to);

        if is_check(&applied_board, !player_turn) {
            return false;
        }
    }

    match from_piece.piece_type() {
        PieceType::Empty => false,
        PieceType::Pawn => is_valid_pawn_move(&to_piece, player_turn, from, to),
        PieceType::Knight => is_valid_knight_move(from, to),
        PieceType::Bishop => is_valid_bishop_move(from, to),
        PieceType::Rook => is_valid_rook_move(from, to),
        PieceType::Queen => is_valid_queen_move(from, to),
        PieceType::King => is_valid_king_move(board.layer_not_moved, from, to),
    }
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
        king_layer,
        king_index,
        player_turn,
        board.layer_color,
        board.layer_pawn
    ) || is_checked_by_knight(
        king_layer,
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

fn is_checked_by_pawn(king_layer: u64, king_index: u32, player_turn: bool, layer_color: u64, layer_pawn: u64) -> bool {
    if player_turn && king_index < 56 {
        let mut pawn_mask: u64 = 0b0;

        if king_index % 8 > 0 {
            pawn_mask |= king_layer << 7;
        }
        if king_index % 8 < 7 {
            pawn_mask |= king_layer << 9;
        }
        
        let pawns: u64 = layer_color & layer_pawn;
        
        pawn_mask & pawns != 0b0 
    } else if !player_turn && king_index > 7 {
        let mut pawn_mask: u64 = 0b0;

        if king_index % 8 > 0 {
            pawn_mask |= king_layer >> 9;
        }
        if king_index % 8 < 7 {
            pawn_mask |= king_layer >> 7;
        }
        
        let pawns: u64 = layer_color & layer_pawn;
        
        pawn_mask & pawns != 0b0
    } else {
        false
    }
}

fn is_checked_by_knight(king_layer: u64, king_index: u32, knight_layer: u64) -> bool {
    let mut knight_mask: u64 = 0b0;
    
    if king_index % 8 != 0 && king_index > 15 { // up-left
        knight_mask |= king_layer >> 17;
    }
    if king_index % 8 != 7 && king_index > 15 { // up-right
        knight_mask |= king_layer >> 15;
    }

    if king_index % 8 < 6 && king_index > 7 { // right-up
        knight_mask |= king_layer >> 6;
    }
    if king_index % 8 < 6 && king_index < 56 { // right-down
        knight_mask |= king_layer << 10;
    }

    if king_index % 8 != 7 && king_index < 48 { // down-right
        knight_mask |= king_layer << 17;
    }
    if king_index % 8 != 0 && king_index < 48 { // down-left
        knight_mask |= king_layer << 15;
    }

    if king_index % 8 > 1 && king_index > 7 { // left-up
        knight_mask |= king_layer >> 10;
    }
    if king_index % 8 < 1 && king_index < 56 { // left-down
        knight_mask |= king_layer << 6;
    }
    
    knight_layer & knight_mask != 0b0
}

fn is_checked_in_diagonal_line(king_layer: u64, king_index: u32, pieces_layer: u64, all_pieces_layer: u64) -> bool {
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

fn is_checked_in_straight_line(king_layer: u64, king_index: u32, pieces_layer: u64, all_pieces_layer: u64) -> bool {
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
    get_all_possible_moves(board, !player_turn, true).into_iter()
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
            has_possible_moves(board, !player_turn, &from_pos, true)
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

fn is_valid_pawn_move(to_piece: &Piece, player_turn: bool, from: &Position, to: &Position) -> bool {
    let valid_one_move: bool = to_piece.piece_type() == PieceType::Empty
        && (player_turn && (from.row + 1 == to.row && from.column == to.column)
            || !player_turn && (from.row == to.row + 1 && from.column == to.column));
    let valid_double_move: bool = to_piece.piece_type() == PieceType::Empty
        && (player_turn && (from.row == 1 && to.row == 3 && from.column == to.column)
            || !player_turn && (from.row == 6 && to.row == 4 && from.column == to.column));
    let valid_attack_move: bool = to_piece.piece_type() != PieceType::Empty
        && (player_turn && (from.row + 1 == to.row && from.column.abs_diff(to.column) == 1)
            || !player_turn && (from.row == to.row + 1 && from.column.abs_diff(to.column) == 1));
    // add en pasant, but honestly just leave as it is because that much requires:
    // 1. The capturing pawn must have advanced exactly three ranks to perform this move.
    // 2. The captured pawn must have moved two squares in one move, landing right next to the capturing pawn.
    // 3. The en passant capture must be performed on the turn immediately after the pawn being captured moves. If the player does not capture en passant on that turn, they no longer can do it later.

    valid_one_move || valid_double_move || valid_attack_move
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

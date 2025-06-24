use crate::core::move_generator::get_possible_moves;

use super::{
    board::Board, move_generator::{get_all_possible_moves, has_possible_moves}, piece::{Piece, PieceType}, position::Position
};

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
        return true;
    }
    let king_index: u32 = king_layer.ilog2();

    board.iterator_positions_and_pieces()
        .filter_map(|(pos, piece)| {
            if piece.get_color() == player_turn {
                Some(pos)
            } else {
                None
            }
        })
        .any(|from_pos| {
            get_possible_moves(board, player_turn, &from_pos, false)
                .into_iter()
                .any(|to_pos| {
                    king_index == to_pos.as_u32()
                })
        })
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

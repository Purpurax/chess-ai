use super::{position::Position, board::Board, piece::{Piece, PieceType}};

pub fn is_move_valid(
        board: Board,
        player_turn: bool,
        from: Position,
        to: Position
    ) -> bool {
    if !is_position_on_board(&from)
    || !is_position_on_board(&to)
    || from == to {
        return false
    }
    
    let from_piece: Piece = board.get_piece_at(&from);
    let to_piece: Piece = board.get_piece_at(&to);

    if !are_pieces_valid(&from_piece, &to_piece, player_turn) {
        return false
    }

    if !is_path_clear(board, &from, &to) && from_piece.piece_type() != PieceType::Knight {
        return false
    }

    match from_piece.piece_type() {
        PieceType::Empty => false,
        PieceType::Pawn => is_valid_pawn_move(&to_piece, player_turn, &from, &to),
        PieceType::Knight => is_valid_knight_move(&from, &to),
        PieceType::Bishop => is_valid_bishop_move(&from, &to),
        PieceType::Rook => is_valid_rook_move(&from, &to),
        PieceType::Queen => is_valid_queen_move(&from, &to),
        PieceType::King => is_valid_king_move(&from, &to),
    }
}

fn is_position_on_board(position: &Position) -> bool {
    position.row <= 7 && position.column <= 7
}

fn are_pieces_valid(from_piece: &Piece, to_piece: &Piece, player_turn: bool) -> bool {
    from_piece.piece_type() != PieceType::Empty
    && from_piece.get_color() == player_turn
    && (
        to_piece.piece_type() == PieceType::Empty
        || to_piece.get_color() == !player_turn
    )
}

fn is_path_clear(board: Board, from: &Position, to: &Position) -> bool {
    let layer_occupied: u64 = !board.get_empty_layer();

    let mut position_check: Position = from.clone();
    position_check.move_towards(&to);

    while position_check != *to {
        if Board::get_layer_value_at(&layer_occupied, &position_check) {
            return false
        }

        position_check.move_towards(&to);
    }

    return true
}

fn is_valid_pawn_move(to_piece: &Piece, player_turn: bool, from: &Position, to: &Position) -> bool {
    let valid_one_move: bool =
        player_turn && (
            from.row + 1 == to.row
            && from.column == to.column
        ) || !player_turn && (
            from.row == to.row + 1
            && from.column == to.column
        );
    let valid_double_move: bool =
        player_turn && (
            from.row == 1
            && to.row == 3
            && from.column == to.column
        ) || !player_turn && (
            from.row == 6
            && to.row == 4
            && from.column == to.column
        );
    let valid_attack_move: bool =
        to_piece.piece_type() != PieceType::Empty && (
            player_turn && (
                from.row + 1 == to.row
                && from.column.abs_diff(to.column) == 1
            ) || !player_turn && (
                from.row == to.row + 1
                && from.column.abs_diff(to.column) == 1
            )
        );
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
    || from.row == to.row || from.column == to.column
}

fn is_valid_king_move(from: &Position, to: &Position) -> bool {
    // TODO: castling king
    from.row.abs_diff(to.row) <= 1
    && from.column.abs_diff(to.column) <= 1
}
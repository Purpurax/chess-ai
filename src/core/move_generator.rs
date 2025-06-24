use super::{
    board::Board,
    move_validator::is_move_valid,
    piece::{Piece, PieceType},
    position::Position,
};

pub fn get_all_possible_moves(
    board: &Board,
    player_turn: bool,
    checking_check: bool
) -> Vec<(Position, Position)> {
    board.iterator_positions_and_pieces()
        .filter(|(_pos, piece)| {
            piece.piece_type() != PieceType::Empty
            && piece.get_color() == player_turn
        }).flat_map(|(from_pos, _)| {
            get_possible_moves(board, player_turn, &from_pos, checking_check)
                .into_iter()
                .map(move |to_pos| {
                    (from_pos.clone(), to_pos)
                })
        }).collect::<Vec<(Position, Position)>>()
}

pub fn get_possible_moves(
    board: &Board,
    player_turn: bool,
    pos: &Position,
    checking_check: bool,
) -> Vec<Position> {
    let from_piece: Piece = board.get_piece_at(pos);

    match from_piece.piece_type() {
        PieceType::Empty => vec![],
        PieceType::Pawn => get_possible_moves_pawn(pos),
        PieceType::Knight => get_possible_moves_knight(pos),
        PieceType::Bishop => get_possible_moves_bishop(pos),
        PieceType::Rook => get_possible_moves_rook(pos),
        PieceType::Queen => get_possible_moves_queen(pos),
        PieceType::King => get_possible_moves_king(pos),
    }
    .into_iter()
    .filter(|to| is_move_valid(board, player_turn, pos, to, checking_check))
    .collect()
}

pub fn has_possible_moves(
    board: &Board,
    player_turn: bool,
    pos: &Position,
    checking_check: bool,
) -> bool {
    let from_piece: Piece = board.get_piece_at(pos);

    !match from_piece.piece_type() {
        PieceType::Empty => vec![],
        PieceType::Pawn => get_possible_moves_pawn(pos),
        PieceType::Knight => get_possible_moves_knight(pos),
        PieceType::Bishop => get_possible_moves_bishop(pos),
        PieceType::Rook => get_possible_moves_rook(pos),
        PieceType::Queen => get_possible_moves_queen(pos),
        PieceType::King => get_possible_moves_king(pos),
    }
    .into_iter()
    .filter(|to| is_move_valid(board, player_turn, pos, to, checking_check))
    .collect::<Vec<Position>>().is_empty()
}

fn get_possible_moves_pawn(pos: &Position) -> Vec<Position> {
    vec![
        Position::new(pos.row.saturating_sub(2), pos.column),
        Position::new(pos.row.saturating_sub(1), pos.column.saturating_sub(1)),
        Position::new(pos.row.saturating_sub(1), pos.column),
        Position::new(pos.row.saturating_sub(1), pos.column + 1),
        Position::new(pos.row + 1, pos.column.saturating_sub(1)),
        Position::new(pos.row + 1, pos.column),
        Position::new(pos.row + 1, pos.column + 1),
        Position::new(pos.row + 2, pos.column),
    ]
}

fn get_possible_moves_knight(pos: &Position) -> Vec<Position> {
    vec![
        Position::new(pos.row.saturating_sub(2), pos.column.saturating_sub(1)),
        Position::new(pos.row.saturating_sub(2), pos.column + 1),
        Position::new(pos.row.saturating_sub(1), pos.column.saturating_sub(2)),
        Position::new(pos.row.saturating_sub(1), pos.column + 2),
        Position::new(pos.row + 1, pos.column.saturating_sub(2)),
        Position::new(pos.row + 1, pos.column + 2),
        Position::new(pos.row + 2, pos.column.saturating_sub(1)),
        Position::new(pos.row + 2, pos.column + 1),
    ]
}

fn get_possible_moves_bishop(pos: &Position) -> Vec<Position> {
    vec![
        // Top-left diagonal
        Position::new(pos.row.saturating_sub(1), pos.column.saturating_sub(1)),
        Position::new(pos.row.saturating_sub(2), pos.column.saturating_sub(2)),
        Position::new(pos.row.saturating_sub(3), pos.column.saturating_sub(3)),
        Position::new(pos.row.saturating_sub(4), pos.column.saturating_sub(4)),
        Position::new(pos.row.saturating_sub(5), pos.column.saturating_sub(5)),
        Position::new(pos.row.saturating_sub(6), pos.column.saturating_sub(6)),
        Position::new(pos.row.saturating_sub(7), pos.column.saturating_sub(7)),
        
        // Top-right diagonal
        Position::new(pos.row.saturating_sub(1), pos.column + 1),
        Position::new(pos.row.saturating_sub(2), pos.column + 2),
        Position::new(pos.row.saturating_sub(3), pos.column + 3),
        Position::new(pos.row.saturating_sub(4), pos.column + 4),
        Position::new(pos.row.saturating_sub(5), pos.column + 5),
        Position::new(pos.row.saturating_sub(6), pos.column + 6),
        Position::new(pos.row.saturating_sub(7), pos.column + 7),
        
        // Bottom-left diagonal
        Position::new(pos.row + 1, pos.column.saturating_sub(1)),
        Position::new(pos.row + 2, pos.column.saturating_sub(2)),
        Position::new(pos.row + 3, pos.column.saturating_sub(3)),
        Position::new(pos.row + 4, pos.column.saturating_sub(4)),
        Position::new(pos.row + 5, pos.column.saturating_sub(5)),
        Position::new(pos.row + 6, pos.column.saturating_sub(6)),
        Position::new(pos.row + 7, pos.column.saturating_sub(7)),
        
        // Bottom-right diagonal
        Position::new(pos.row + 1, pos.column + 1),
        Position::new(pos.row + 2, pos.column + 2),
        Position::new(pos.row + 3, pos.column + 3),
        Position::new(pos.row + 4, pos.column + 4),
        Position::new(pos.row + 5, pos.column + 5),
        Position::new(pos.row + 6, pos.column + 6),
        Position::new(pos.row + 7, pos.column + 7),
    ]
}

fn get_possible_moves_rook(pos: &Position) -> Vec<Position> {
    vec![
        Position::new(0, pos.column),
        Position::new(1, pos.column),
        Position::new(2, pos.column),
        Position::new(3, pos.column),
        Position::new(4, pos.column),
        Position::new(5, pos.column),
        Position::new(6, pos.column),
        Position::new(7, pos.column),
        Position::new(pos.row, 0),
        Position::new(pos.row, 1),
        Position::new(pos.row, 2),
        Position::new(pos.row, 3),
        Position::new(pos.row, 4),
        Position::new(pos.row, 5),
        Position::new(pos.row, 6),
        Position::new(pos.row, 7),
    ]
}

fn get_possible_moves_queen(pos: &Position) -> Vec<Position> {
    let mut bishop_moves = get_possible_moves_bishop(pos);
    let mut rook_moves = get_possible_moves_rook(pos);
    
    bishop_moves.append(&mut rook_moves);
    bishop_moves
}

fn get_possible_moves_king(pos: &Position) -> Vec<Position> {
    vec![
        Position::new(pos.row.saturating_sub(1), pos.column.saturating_sub(1)),
        Position::new(pos.row.saturating_sub(1), pos.column),
        Position::new(pos.row.saturating_sub(1), pos.column + 1),
        Position::new(pos.row, pos.column.saturating_sub(1)),
        Position::new(pos.row, pos.column + 1),
        Position::new(pos.row + 1, pos.column.saturating_sub(1)),
        Position::new(pos.row + 1, pos.column),
        Position::new(pos.row + 1, pos.column + 1),
        
        Position::new(pos.row, pos.column.saturating_sub(2)),
        Position::new(pos.row, pos.column + 2),
    ]
}

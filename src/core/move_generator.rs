use super::{position::Position, board::Board, piece::{Piece, PieceType}, move_validator::is_move_valid};

pub fn get_all_possible_moves(board: &Board, player_turn: bool, pos: &Position, checking_check: bool) -> Vec<Position> {
    let from_piece: Piece = board.get_piece_at(&pos);

    match from_piece.piece_type() {
        PieceType::Empty => vec![],
        PieceType::Pawn => get_possible_moves_pawn(pos),
        PieceType::Knight => get_possible_moves_bishop(pos),
        PieceType::Bishop => get_possible_moves_bishop(pos),
        PieceType::Rook => get_possible_moves_rook(pos),
        PieceType::Queen => get_possible_moves_queen(pos),
        PieceType::King => get_possible_moves_king(pos),
    }.into_iter()
    .filter(|to|
        is_move_valid(board, player_turn, pos, to, checking_check)
    ).collect()
}

fn get_possible_moves_pawn(pos: &Position) -> Vec<Position> {
    [
        (-2, 0),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (1, 0),
        (1, 1),
        (2, 0)
    ].into_iter().filter_map(|(row_mod, column_mod)|
        if (row_mod == -2 && pos.row <= 1)
        || (row_mod == -1 && pos.row == 0)
        || (row_mod == 1 && pos.row == 7)
        || (row_mod == 2 && pos.row >= 6)
        || (column_mod == -1 && pos.column == 0)
        || (column_mod == 1 && pos.column == 7) {
            None
        } else {
            Some(Position::new((pos.row as i8 + row_mod) as u8, (pos.column as i8 + column_mod) as u8))
        }
    ).collect()
}

fn get_possible_moves_knight(pos: &Position) -> Vec<Position> {
    [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1)
    ].iter().filter_map(|(row_mod, column_mod)|
        if (*row_mod == -2 && pos.row <= 1)
        || (*row_mod == -1 && pos.row == 0)
        || (*row_mod == 1 && pos.row == 7)
        || (*row_mod == 2 && pos.row >= 6)
        || (*column_mod == -2 && pos.column <= 1)
        || (*column_mod == -1 && pos.column == 0)
        || (*column_mod == 1 && pos.column == 7)
        || (*column_mod == 2 && pos.column >= 6) {
            None
        } else {
            Some(Position::new((pos.row as i8 + *row_mod) as u8, (pos.column as i8 + *column_mod) as u8))
        }
    ).collect()
}

fn get_possible_moves_bishop(pos: &Position) -> Vec<Position> {
    let mut moves: Vec<Position> = vec![];
    let mut t_row: u8 = pos.row;
    let mut t_column: u8 = pos.column;
    while t_row != 0 && t_column != 0 {
        t_row -= 1;
        t_column -= 1;
        moves.push(Position::new(t_row, t_column));
    }
    t_row = pos.row;
    t_column = pos.column;
    while t_row != 0 && t_column != 7 {
        t_row -= 1;
        t_column += 1;
        moves.push(Position::new(t_row, t_column));
    }
    t_row = pos.row;
    t_column = pos.column;
    while t_row != 7 && t_column != 0 {
        t_row += 1;
        t_column -= 1;
        moves.push(Position::new(t_row, t_column));
    }
    t_row = pos.row;
    t_column = pos.column;
    while t_row != 7 && t_column != 7 {
        t_row += 1;
        t_column += 1;
        moves.push(Position::new(t_row, t_column));
    }
    
    moves
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
        Position::new(pos.row, 7)
    ]
}

fn get_possible_moves_queen(pos: &Position) -> Vec<Position> {
    let mut moves: Vec<Position> = vec![];
    let mut t_row: u8 = pos.row;
    let mut t_column: u8 = pos.column;
    while t_row != 0 && t_column != 0 {
        t_row -= 1;
        t_column -= 1;
        moves.push(Position::new(t_row, t_column));
    }
    t_row = pos.row;
    t_column = pos.column;
    while t_row != 0 && t_column != 7 {
        t_row -= 1;
        t_column += 1;
        moves.push(Position::new(t_row, t_column));
    }
    t_row = pos.row;
    t_column = pos.column;
    while t_row != 7 && t_column != 0 {
        t_row += 1;
        t_column -= 1;
        moves.push(Position::new(t_row, t_column));
    }
    t_row = pos.row;
    t_column = pos.column;
    while t_row != 7 && t_column != 7 {
        t_row += 1;
        t_column += 1;
        moves.push(Position::new(t_row, t_column));
    }
    
    moves.extend([
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
        Position::new(pos.row, 7)
    ]);

    moves
}

fn get_possible_moves_king(pos: &Position) -> Vec<Position> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1)
    ].iter().filter_map(|(row_mod, column_mod)|
        if (*row_mod == -1 && pos.row == 0)
        || (*row_mod == 1 && pos.row == 7)
        || (*column_mod == -1 && pos.column == 0)
        || (*column_mod == 1 && pos.column == 7) {
            None
        } else {
            Some(Position::new((pos.row as i8 + *row_mod) as u8, (pos.column as i8 + *column_mod) as u8))
        }
    ).collect()
}
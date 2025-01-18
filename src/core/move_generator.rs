use super::{position::Position, board::Board, piece::{Piece, PieceType}, move_validator::is_move_valid};

pub fn get_all_possible_moves(board: Board, player_turn: bool, pos: Position) -> Vec<Position> {
    let from_piece: Piece = board.get_piece_at(&pos);

    let row: u8 = pos.row;
    let column: u8 = pos.column;

    match from_piece.piece_type() {
        PieceType::Empty => vec![],
        PieceType::Pawn => {
            [
                (-2, 0),
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (1, -1),
                (1, 0),
                (1, 1),
                (2, 0)
            ].iter().filter_map(|(row_mod, column_mod)|
                if (*row_mod == -2 && row <= 1)
                || (*row_mod == -1 && row == 0)
                || (*row_mod == 1 && row == 7)
                || (*row_mod == 2 && row >= 6)
                || (*column_mod == -1 && column == 0)
                || (*column_mod == 1 && column == 7) {
                    None
                } else {
                    Some(Position::new((row as i8 + *row_mod) as u8, (column as i8 + *column_mod) as u8))
                }
            ).filter(|to| is_move_valid(board.clone(), player_turn, pos.clone(), to.clone())
            ).collect()
        },
        PieceType::Knight => {
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
                if (*row_mod == -2 && row <= 1)
                || (*row_mod == -1 && row == 0)
                || (*row_mod == 1 && row == 7)
                || (*row_mod == 2 && row >= 6)
                || (*column_mod == -2 && column <= 1)
                || (*column_mod == -1 && column == 0)
                || (*column_mod == 1 && column == 7)
                || (*column_mod == 2 && column >= 6) {
                    None
                } else {
                    Some(Position::new((row as i8 + *row_mod) as u8, (column as i8 + *column_mod) as u8))
                }
            ).filter(|to| is_move_valid(board.clone(), player_turn, pos.clone(), to.clone())
            ).collect()
        },
        PieceType::Bishop => {
            let mut moves: Vec<Position> = vec![];
            let mut t_row: u8 = row;
            let mut t_column: u8 = column;
            while t_row != 0 && t_column != 0 {
                t_row -= 1;
                t_column -= 1;
                moves.push(Position::new(t_row, t_column));
            }
            t_row = row;
            t_column = column;
            while t_row != 0 && t_column != 7 {
                t_row -= 1;
                t_column += 1;
                moves.push(Position::new(t_row, t_column));
            }
            t_row = row;
            t_column = column;
            while t_row != 7 && t_column != 0 {
                t_row += 1;
                t_column -= 1;
                moves.push(Position::new(t_row, t_column));
            }
            t_row = row;
            t_column = column;
            while t_row != 7 && t_column != 7 {
                t_row += 1;
                t_column += 1;
                moves.push(Position::new(t_row, t_column));
            }
            
            moves.into_iter().filter(|to| is_move_valid(board.clone(), player_turn, pos.clone(), to.clone())
            ).collect()
        },
        PieceType::Rook => {
            [
                Position::new(0, column),
                Position::new(1, column),
                Position::new(2, column),
                Position::new(3, column),
                Position::new(4, column),
                Position::new(5, column),
                Position::new(6, column),
                Position::new(7, column),
                Position::new(row, 0),
                Position::new(row, 1),
                Position::new(row, 2),
                Position::new(row, 3),
                Position::new(row, 4),
                Position::new(row, 5),
                Position::new(row, 6),
                Position::new(row, 7)
            ].into_iter().filter(|to| *to != pos && is_move_valid(board.clone(), player_turn, pos.clone(), to.clone())
            ).collect()
        },
        PieceType::Queen => {
            let mut moves: Vec<Position> = vec![];
            let mut t_row: u8 = row;
            let mut t_column: u8 = column;
            while t_row != 0 && t_column != 0 {
                t_row -= 1;
                t_column -= 1;
                moves.push(Position::new(t_row, t_column));
            }
            t_row = row;
            t_column = column;
            while t_row != 0 && t_column != 7 {
                t_row -= 1;
                t_column += 1;
                moves.push(Position::new(t_row, t_column));
            }
            t_row = row;
            t_column = column;
            while t_row != 7 && t_column != 0 {
                t_row += 1;
                t_column -= 1;
                moves.push(Position::new(t_row, t_column));
            }
            t_row = row;
            t_column = column;
            while t_row != 7 && t_column != 7 {
                t_row += 1;
                t_column += 1;
                moves.push(Position::new(t_row, t_column));
            }
            
            moves.into_iter().filter(|to| is_move_valid(board.clone(), player_turn, pos.clone(), to.clone())
            ).chain([
                Position::new(0, column),
                Position::new(1, column),
                Position::new(2, column),
                Position::new(3, column),
                Position::new(4, column),
                Position::new(5, column),
                Position::new(6, column),
                Position::new(7, column),
                Position::new(row, 0),
                Position::new(row, 1),
                Position::new(row, 2),
                Position::new(row, 3),
                Position::new(row, 4),
                Position::new(row, 5),
                Position::new(row, 6),
                Position::new(row, 7)
            ].into_iter().filter(|to| *to != pos && is_move_valid(board.clone(), player_turn, pos.clone(), to.clone())
            )).collect()
        },
        PieceType::King => {
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
                if (*row_mod == -1 && row == 0)
                || (*row_mod == 1 && row == 7)
                || (*column_mod == -1 && column == 0)
                || (*column_mod == 1 && column == 7) {
                    None
                } else {
                    Some(Position::new((row as i8 + *row_mod) as u8, (column as i8 + *column_mod) as u8))
                }
            ).filter(|to| is_move_valid(board.clone(), player_turn, pos.clone(), to.clone())
            ).collect()
        },
    }
}
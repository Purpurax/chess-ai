use std::collections::HashMap;

use ggez::cgmath::{Point2, Vector2};
use ggez::graphics::Image;
use good_web_game as ggez;

use crate::core::board::Board;
use crate::core::{
    piece::{Piece, PieceType},
    position::Position,
};

const BOARD_BOARDER: f32 = 30.0;

fn valid_click_coordinates(x: f32, y: f32) -> bool {
    x > 0.0 && y > 0.0 && x < 1280.0 && y < 1280.0
}

fn get_position(x: f32, y: f32) -> Position {
    let row: u8 = (y / 160.0) as u8;
    let column: u8 = (x / 160.0) as u8;

    Position::new(7 - row, column)
}

pub fn get_position_of_coordinates(
    x: f32,
    y: f32,
    offsets: &Point2<f32>,
    scales: &Vector2<f32>,
) -> Option<Position> {
    let logical_x: f32 = (x - offsets.x) / scales.x - BOARD_BOARDER;
    let logical_y: f32 = (y - offsets.y) / scales.y - BOARD_BOARDER;

    if !valid_click_coordinates(logical_x, logical_y) {
        return None;
    }

    Some(get_position(logical_x, logical_y))
}

pub fn determine_taken_pieces_images(images: &HashMap<String, Image>, board: &Board) -> Vec<Image> {
    let white_pawn_count: u32 = (board.layer_color & board.layer_pawn).count_ones();
    let white_knight_count: u32 = (board.layer_color & board.layer_knight).count_ones();
    let white_bishop_count: u32 = (board.layer_color & board.layer_bishop).count_ones();
    let white_rook_count: u32 = (board.layer_color & board.layer_rook).count_ones();
    let white_queen_count: u32 = (board.layer_color & board.layer_queen).count_ones();
    let black_pawn_count: u32 = (!board.layer_color & board.layer_pawn).count_ones();
    let black_knight_count: u32 = (!board.layer_color & board.layer_knight).count_ones();
    let black_bishop_count: u32 = (!board.layer_color & board.layer_bishop).count_ones();
    let black_rook_count: u32 = (!board.layer_color & board.layer_rook).count_ones();
    let black_queen_count: u32 = (!board.layer_color & board.layer_queen).count_ones();

    let mut result: Vec<Image> = vec![];

    if white_pawn_count < 8 {
        result.push(
            images[&format!("taken pieces panel white pawn {}", 8 - white_pawn_count)].clone()
        );
    }
    if white_knight_count < 2 {
        result.push(
            images[&format!("taken pieces panel white knight {}", 2 - white_knight_count)].clone()
        );
    }
    if white_bishop_count < 2 {
        result.push(
            images[&format!("taken pieces panel white bishop {}", 2 - white_bishop_count)].clone()
        );
    }
    if white_rook_count < 2 {
        result.push(
            images[&format!("taken pieces panel white rook {}", 2 - white_rook_count)].clone()
        );
    }
    if white_queen_count < 1 {
        result.push(
            images["taken pieces panel white queen"].clone()
        );
    }

    if black_pawn_count < 8 {
        result.push(
            images[&format!("taken pieces panel black pawn {}", 8 - black_pawn_count)].clone()
        );
    }
    if black_knight_count < 2 {
        result.push(
            images[&format!("taken pieces panel black knight {}", 2 - black_knight_count)].clone()
        );
    }
    if black_bishop_count < 2 {
        result.push(
            images[&format!("taken pieces panel black bishop {}", 2 - black_bishop_count)].clone()
        );
    }
    if black_rook_count < 2 {
        result.push(
            images[&format!("taken pieces panel black rook {}", 2 - black_rook_count)].clone()
        );
    }
    if black_queen_count < 1 {
        result.push(
            images["taken pieces panel black queen"].clone()
        );
    }

    result
}

pub fn determine_image(images: &HashMap<String, Image>, piece: &Piece) -> Option<Image> {
    match <(bool, PieceType)>::from(piece) {
        (_, PieceType::Empty) => None,
        (false, PieceType::Pawn) => Some(images["black pawn"].clone()),
        (false, PieceType::Knight) => Some(images["black knight"].clone()),
        (false, PieceType::Bishop) => Some(images["black bishop"].clone()),
        (false, PieceType::Rook) => Some(images["black rook"].clone()),
        (false, PieceType::Queen) => Some(images["black queen"].clone()),
        (false, PieceType::King) => Some(images["black king"].clone()),
        (true, PieceType::Pawn) => Some(images["white pawn"].clone()),
        (true, PieceType::Knight) => Some(images["white knight"].clone()),
        (true, PieceType::Bishop) => Some(images["white bishop"].clone()),
        (true, PieceType::Rook) => Some(images["white rook"].clone()),
        (true, PieceType::Queen) => Some(images["white queen"].clone()),
        (true, PieceType::King) => Some(images["white king"].clone()),
    }
}

pub fn determine_image_position(
    position: &Position,
    offsets: &Point2<f32>,
    scales: &Vector2<f32>,
) -> Point2<f32> {
    let x: f32 = offsets.x + scales.x * (BOARD_BOARDER + 160.0 * (position.column as f32));
    let y: f32 = offsets.y + scales.y * (BOARD_BOARDER + 160.0 * ((7 - position.row) as f32));

    Point2::new(x, y)
}

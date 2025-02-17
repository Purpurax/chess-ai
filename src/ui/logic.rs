use std::collections::HashMap;

use ggez::cgmath::{Point2, Vector2};
use ggez::graphics::Image;
use good_web_game as ggez;

use crate::core::{
    piece::{Piece, PieceType},
    position::Position,
};

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
    let logical_x: f32 = (x - offsets.x) / scales.x;
    let logical_y: f32 = (y - offsets.y) / scales.y;

    if !valid_click_coordinates(logical_x, logical_y) {
        return None;
    }

    Some(get_position(logical_x, logical_y))
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
    let x: f32 = offsets.x + 160.0 * scales.x * (position.column as f32);
    let y: f32 = offsets.y + 160.0 * scales.y * ((7 - position.row) as f32);

    Point2::new(x, y)
}

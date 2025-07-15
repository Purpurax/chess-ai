use std::collections::HashMap;

use ggez::cgmath::{Point2, Vector2};
use ggez::graphics::Image;
use good_web_game as ggez;

use crate::agent::{Agent, AgentType};
use crate::core::board::Board;
use crate::core::{
    piece::{Piece, PieceType},
    position::Position,
};
use crate::ui::BOARD_BORDER;

use super::{BOARD_SIZE, PIECE_SIZE, TAKE_PANEL_HEIGHT};

const BOTTOM_PANEL_BUTTONS: [(f32, usize); 6] = [
    (60.0, 0),
    (240.0, 1),
    (420.0, 2),
    (720.0, 3),
    (900.0, 4),
    (1080.0, 5)
];

const RESET_BUTTON: [f32; 4] = [650.0, 690.0, 1390.0, 1430.0]; // x, x, y, y

fn valid_click_coordinates(x: f32, y: f32) -> bool {
    x > 0.0 && y > 0.0 && x < BOARD_SIZE && y < BOARD_SIZE
}

fn get_position(x: f32, y: f32) -> Position {
    let row: u8 = (y / PIECE_SIZE) as u8;
    let column: u8 = (x / PIECE_SIZE) as u8;

    Position::new(7 - row, column)
}

pub fn get_position_of_coordinates(
    x: f32,
    y: f32,
    offsets: &Point2<f32>,
    scales: &Vector2<f32>,
) -> Option<Position> {
    let logical_x: f32 = (x - offsets.x) / scales.x - BOARD_BORDER;
    let logical_y: f32 = (y - offsets.y) / scales.y - BOARD_BORDER;

    if !valid_click_coordinates(logical_x, logical_y) {
        return None;
    }

    Some(get_position(logical_x, logical_y))
}

pub fn get_position_of_coordinates_bottom_panel(
    x: f32,
    y: f32,
    offsets: &Point2<f32>,
    scales: &Vector2<f32>,
) -> Option<usize> {
    let logical_x: f32 = (x - offsets.x) / scales.x;
    let logical_y: f32 = (y - offsets.y) / scales.y;

    if logical_y < BOARD_BORDER + BOARD_SIZE + BOARD_BORDER + TAKE_PANEL_HEIGHT + 50.0
    || logical_y > BOARD_BORDER + BOARD_SIZE + BOARD_BORDER + TAKE_PANEL_HEIGHT + 190.0 {
        return None;
    }

    BOTTOM_PANEL_BUTTONS.into_iter()
        .filter(|(x, _)| BOARD_BORDER + x < logical_x && logical_x < BOARD_BORDER + x + 140.0)
        .map(|(_, index)| {
            index
        })
        .next()
}

pub fn reset_button_is_clicked(
    x: f32,
    y: f32,
    offsets: &Point2<f32>,
    scales: &Vector2<f32>
) -> bool {
    let logical_x: f32 = (x - offsets.x) / scales.x;
    let logical_y: f32 = (y - offsets.y) / scales.y;

    RESET_BUTTON[0] < logical_x && logical_x < RESET_BUTTON[1]
    && RESET_BUTTON[2] < logical_y && logical_y < RESET_BUTTON[3]
}

pub fn get_new_agents_after_button_press(
    white_agent: &mut Option<Agent>,
    black_agent: &mut Option<Agent>,
    button_index: usize
) {
    match button_index {
        0 => { // hardest white agent clicked
            if white_agent.is_some()
            && matches!(white_agent.as_ref().unwrap().agent_type, AgentType::Minimax) {
                *white_agent = None
            } else {
                *white_agent = Some(Agent::new_minimax())
            }
        },
        1 => { // medium white agent clicked
            if white_agent.is_some()
            && matches!(white_agent.as_ref().unwrap().agent_type, AgentType::MonteCarlo(_)) {
                *white_agent = None
            } else {
                *white_agent = Some(Agent::new_monte_carlo())
            }
        },
        2 => { // easy white agent clicked
            if white_agent.is_some()
            && matches!(white_agent.as_ref().unwrap().agent_type, AgentType::NeuralNetwork(_)) {
                *white_agent = None
            } else {
                *white_agent = Some(Agent::new_neural_network())
            }
        },
        3 => { // easy black agent clicked
            if black_agent.is_some()
            && matches!(black_agent.as_ref().unwrap().agent_type, AgentType::NeuralNetwork(_)) {
                *black_agent = None
            } else {
                *black_agent = Some(Agent::new_neural_network())
            }
        },
        4 => { // medium black agent clicked
            if black_agent.is_some()
            && matches!(black_agent.as_ref().unwrap().agent_type, AgentType::MonteCarlo(_)) {
                *black_agent = None
            } else {
                *black_agent = Some(Agent::new_monte_carlo())
            }
        },
        _ => { // hardest black agent clicked
            if black_agent.is_some()
            && matches!(black_agent.as_ref().unwrap().agent_type, AgentType::Minimax) {
                *black_agent = None
            } else {
                *black_agent = Some(Agent::new_minimax())
            }
        }
    }
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
    let x: f32 = offsets.x + scales.x * (BOARD_BORDER + PIECE_SIZE * (position.column as f32));
    let y: f32 = offsets.y + scales.y * (BOARD_BORDER + PIECE_SIZE * ((7 - position.row) as f32));

    Point2::new(x, y)
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

pub fn determine_bottom_panel_image(images: &HashMap<String, Image>, white_agent: &Option<Agent>, black_agent: &Option<Agent>) -> Image {
    let mut image_index = match white_agent {
        Some(agent) => {
            match agent.agent_type {
                crate::agent::AgentType::Random => 0,
                crate::agent::AgentType::NeuralNetwork(_) => 4,
                crate::agent::AgentType::MonteCarlo(_) => 8,
                crate::agent::AgentType::Minimax => 12,
            }
        },
        None => 0
    };

    image_index += match black_agent {
        Some(agent) => {
            match agent.agent_type {
                crate::agent::AgentType::Random => 0,
                crate::agent::AgentType::NeuralNetwork(_) => 1,
                crate::agent::AgentType::MonteCarlo(_) => 2,
                crate::agent::AgentType::Minimax => 3,
            }
        },
        None => 0
    };

    let image_name = format!("bottom panel {}", image_index);
    images[&image_name].clone()
}

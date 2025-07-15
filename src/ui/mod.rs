mod carry_piece;
mod logic;

use ggez::cgmath::{Point2, Vector2};
use ggez::event::EventHandler;
use ggez::graphics::{DrawParam, Image, Rect};
use ggez::{event, graphics, Context, GameError, GameResult};
use good_web_game::{self as ggez, timer};
use good_web_game::graphics::Color;

use logic::{determine_bottom_panel_image, determine_taken_pieces_images, get_new_agents_after_button_press, get_position_of_coordinates_bottom_panel, reset_button_is_clicked};
use miniquad::GraphicsContext;
use std::collections::HashMap;

use crate::agent::{Agent, AgentType};
use crate::core::board::Board;
use crate::core::game::Game;
use crate::core::move_generator::get_possible_moves;
use crate::core::piece::{Piece, PieceType};
use crate::core::position::Position;
use crate::core::snapshot;
use crate::ui::logic::get_position_of_coordinates;

use self::carry_piece::CarryPiece;
use self::logic::{determine_image, determine_image_position};

const COOLDOWN_TIME: f64 = 0.2;
pub const BOARD_BORDER: f32 = 30.0;
pub const PIECE_SIZE: f32 = 160.0;
pub const BOARD_SIZE: f32 = 8.0 * PIECE_SIZE;
pub const TAKE_PANEL_HEIGHT: f32 = 150.0;
const BOTTOM_PANEL_HEIGHT: f32 = 240.0;

pub struct Engine {
    game: Game,

    images: HashMap<String, Image>,
    offsets: Point2<f32>,
    scales: Vector2<f32>,
    force_draw: bool,

    carry_piece: CarryPiece,
    cooldown_until: f64,

    white_agent: Option<Agent>,
    black_agent: Option<Agent>,

    debug: bool,
}

impl Engine {
    pub fn new(ctx: &mut Context, quad_ctx: &mut GraphicsContext) -> GameResult<Engine> {
        let game: Game = Game::new();

        let images: HashMap<String, Image> = Engine::load_images(ctx, quad_ctx);

        let (window_width, window_height): (f32, f32) = graphics::drawable_size(quad_ctx);
        let offsets: Point2<f32> = Engine::calculate_offsets(window_width, window_height);
        let scales: Vector2<f32> = Engine::calculate_scale(window_width, window_height);

        let carry_piece: CarryPiece = CarryPiece::new();
        let cooldown_until: f64 = timer::time();

        Ok(Engine {
            game,
            images,
            offsets,
            scales,
            force_draw: true,
            carry_piece,
            cooldown_until,
            white_agent: None,
            black_agent: None,
            debug: false,
        })
    }

    fn load_images(ctx: &mut Context, quad_ctx: &mut GraphicsContext) -> HashMap<String, Image> {
        let mut images = [
            ("board", "/assets/board.png"),
            ("black pawn", "/assets/pieces/piece_black_pawn.png"),
            ("black knight", "/assets/pieces/piece_black_knight.png"),
            ("black bishop", "/assets/pieces/piece_black_bishop.png"),
            ("black rook", "/assets/pieces/piece_black_rook.png"),
            ("black queen", "/assets/pieces/piece_black_queen.png"),
            ("black king", "/assets/pieces/piece_black_king.png"),
            ("white pawn", "/assets/pieces/piece_white_pawn.png"),
            ("white knight", "/assets/pieces/piece_white_knight.png"),
            ("white bishop", "/assets/pieces/piece_white_bishop.png"),
            ("white rook", "/assets/pieces/piece_white_rook.png"),
            ("white queen", "/assets/pieces/piece_white_queen.png"),
            ("white king", "/assets/pieces/piece_white_king.png"),
            ("outline green", "/assets/outline_green.png"),
            ("outline red", "/assets/outline_red.png"),
        ]
        .map(|(key, value)| (key.to_string(), Image::new(ctx, quad_ctx, value).unwrap()))
        .into_iter()
        .collect::<HashMap<String, Image>>();

        [
            ("taken pieces panel empty", "/assets/taken_panel/taken_pieces_panel_empty.png"),
            ("taken pieces panel white pawn 1", "/assets/taken_panel/taken_pieces_panel_white_pawn_1.png"),
            ("taken pieces panel white pawn 2", "/assets/taken_panel/taken_pieces_panel_white_pawn_2.png"),
            ("taken pieces panel white pawn 3", "/assets/taken_panel/taken_pieces_panel_white_pawn_3.png"),
            ("taken pieces panel white pawn 4", "/assets/taken_panel/taken_pieces_panel_white_pawn_4.png"),
            ("taken pieces panel white pawn 5", "/assets/taken_panel/taken_pieces_panel_white_pawn_5.png"),
            ("taken pieces panel white pawn 6", "/assets/taken_panel/taken_pieces_panel_white_pawn_6.png"),
            ("taken pieces panel white pawn 7", "/assets/taken_panel/taken_pieces_panel_white_pawn_7.png"),
            ("taken pieces panel white pawn 8", "/assets/taken_panel/taken_pieces_panel_white_pawn_8.png"),
            ("taken pieces panel white knight 1", "/assets/taken_panel/taken_pieces_panel_white_knight_1.png"),
            ("taken pieces panel white knight 2", "/assets/taken_panel/taken_pieces_panel_white_knight_2.png"),
            ("taken pieces panel white bishop 1", "/assets/taken_panel/taken_pieces_panel_white_bishop_1.png"),
            ("taken pieces panel white bishop 2", "/assets/taken_panel/taken_pieces_panel_white_bishop_2.png"),
            ("taken pieces panel white rook 1", "/assets/taken_panel/taken_pieces_panel_white_rook_1.png"),
            ("taken pieces panel white rook 2", "/assets/taken_panel/taken_pieces_panel_white_rook_2.png"),
            ("taken pieces panel white queen", "/assets/taken_panel/taken_pieces_panel_white_queen.png"),
            ("taken pieces panel black pawn 1", "/assets/taken_panel/taken_pieces_panel_black_pawn_1.png"),
            ("taken pieces panel black pawn 2", "/assets/taken_panel/taken_pieces_panel_black_pawn_2.png"),
            ("taken pieces panel black pawn 3", "/assets/taken_panel/taken_pieces_panel_black_pawn_3.png"),
            ("taken pieces panel black pawn 4", "/assets/taken_panel/taken_pieces_panel_black_pawn_4.png"),
            ("taken pieces panel black pawn 5", "/assets/taken_panel/taken_pieces_panel_black_pawn_5.png"),
            ("taken pieces panel black pawn 6", "/assets/taken_panel/taken_pieces_panel_black_pawn_6.png"),
            ("taken pieces panel black pawn 7", "/assets/taken_panel/taken_pieces_panel_black_pawn_7.png"),
            ("taken pieces panel black pawn 8", "/assets/taken_panel/taken_pieces_panel_black_pawn_8.png"),
            ("taken pieces panel black knight 1", "/assets/taken_panel/taken_pieces_panel_black_knight_1.png"),
            ("taken pieces panel black knight 2", "/assets/taken_panel/taken_pieces_panel_black_knight_2.png"),
            ("taken pieces panel black bishop 1", "/assets/taken_panel/taken_pieces_panel_black_bishop_1.png"),
            ("taken pieces panel black bishop 2", "/assets/taken_panel/taken_pieces_panel_black_bishop_2.png"),
            ("taken pieces panel black rook 1", "/assets/taken_panel/taken_pieces_panel_black_rook_1.png"),
            ("taken pieces panel black rook 2", "/assets/taken_panel/taken_pieces_panel_black_rook_2.png"),
            ("taken pieces panel black queen", "/assets/taken_panel/taken_pieces_panel_black_queen.png"),
        ]
        .map(|(key, value)| (key.to_string(), Image::new(ctx, quad_ctx, value).unwrap()))
        .into_iter()
        .for_each(|(key, value)| {
            images.insert(key, value);
        });

        for i in 0..16 {
            let key = format!("bottom panel {}", i);
            let value = Image::new(ctx, quad_ctx, &format!("/assets/bottom_panel/chess_bottom_panel_{}.png", i)).unwrap();
            images.insert(key, value);
        }

        images
    }

    fn calculate_offsets(window_width: f32, window_height: f32) -> Point2<f32> {
        const GAME_IMAGES_WIDTH: f32 = BOARD_BORDER + BOARD_SIZE + BOARD_BORDER;
        const GAME_IMAGES_HEIGHT: f32 = BOARD_BORDER + BOARD_SIZE + BOARD_BORDER + TAKE_PANEL_HEIGHT + BOTTOM_PANEL_HEIGHT;

        let scale: Vector2<f32> = Engine::calculate_scale(window_width, window_height);

        let offset_x: f32 = (window_width - GAME_IMAGES_WIDTH * scale.x) / 2.0;
        let offset_y: f32 = (window_height - GAME_IMAGES_HEIGHT * scale.y) / 2.0;

        Point2::new(offset_x, offset_y)
    }

    fn calculate_scale(window_width: f32, window_height: f32) -> Vector2<f32> {
        const GAME_IMAGES_WIDTH: f32 = BOARD_BORDER + BOARD_SIZE + BOARD_BORDER;
        const GAME_IMAGES_HEIGHT: f32 = BOARD_BORDER + BOARD_SIZE + BOARD_BORDER + TAKE_PANEL_HEIGHT + BOTTOM_PANEL_HEIGHT;

        let window_ratio: f32 = window_width / window_height;
        let game_images_ratio: f32 = GAME_IMAGES_WIDTH / GAME_IMAGES_HEIGHT;

        let scale: f32 = if window_ratio > game_images_ratio {
            window_height / GAME_IMAGES_HEIGHT
        } else {
            window_width / GAME_IMAGES_WIDTH
        };

        Vector2::new(scale, scale)
    }

    fn perform_move(&mut self, from_pos: &Position, to_pos: &Position) {
        if let Some(white_agent) = &mut self.white_agent {
            white_agent.inform_about_move(from_pos, to_pos);
        }
        if let Some(black_agent) = &mut self.black_agent {
            black_agent.inform_about_move(from_pos, to_pos);
        }
        
        self.game.perform_move(from_pos, to_pos);

        self.cooldown_until = timer::time() + COOLDOWN_TIME;
    }

    fn reset(&mut self, ctx: &mut Context, quad_ctx: &mut GraphicsContext) {
        let game: Game = Game::new();

        let images: HashMap<String, Image> = Engine::load_images(ctx, quad_ctx);

        let (window_width, window_height): (f32, f32) = graphics::drawable_size(quad_ctx);
        let offsets: Point2<f32> = Engine::calculate_offsets(window_width, window_height);
        let scales: Vector2<f32> = Engine::calculate_scale(window_width, window_height);

        let carry_piece: CarryPiece = CarryPiece::new();
        let cooldown_until: f64 = timer::time();

        let white_agent = if let Some(agent) = &self.white_agent {
            Some(match agent.agent_type {
                AgentType::Random => Agent::new_random(),
                AgentType::Minimax => Agent::new_minimax(),
                AgentType::MonteCarlo(_) => Agent::new_monte_carlo(),
                AgentType::NeuralNetwork(_) => Agent::new_neural_network()
            })
        } else {
            None
        };
        let black_agent = if let Some(agent) = &self.black_agent {
            Some(match agent.agent_type {
                AgentType::Random => Agent::new_random(),
                AgentType::Minimax => Agent::new_minimax(),
                AgentType::MonteCarlo(_) => Agent::new_monte_carlo(),
                AgentType::NeuralNetwork(_) => Agent::new_neural_network()
            })
        } else {
            None
        };

        *self = Engine {
            game,
            images,
            offsets,
            scales,
            force_draw: true,
            carry_piece,
            cooldown_until,
            white_agent,
            black_agent,
            debug: false,
        }
    }
}

impl EventHandler<GameError> for Engine {
    fn update(&mut self, _ctx: &mut Context, _quad_ctx: &mut GraphicsContext) -> GameResult {
        if timer::time() < self.cooldown_until {
            return Ok(())
        }

        if self.game.get_winner().is_some() || self.force_draw {
            return Ok(())
        } else if self.game.player_turn && self.white_agent.is_some() {
            let agent_move: (Position, Position) = self.white_agent.clone().unwrap().get_next_turn();
            self.perform_move(&agent_move.0, &agent_move.1);
        } else if !self.game.player_turn && self.black_agent.is_some() {
            let agent_move: (Position, Position) = self.black_agent.clone().unwrap().get_next_turn();
            self.perform_move(&agent_move.0, &agent_move.1);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut GraphicsContext) -> GameResult {
        /* Background */
        if self.debug {
            graphics::clear(ctx, quad_ctx, Color::from_rgb_u32(0x46a12f));
        } else {
            graphics::clear(ctx, quad_ctx, Color::from_rgb_u32(0x3F2832));
        }

        let param: DrawParam = DrawParam::new().dest(self.offsets).scale(self.scales);
        graphics::draw(ctx, quad_ctx, &self.images["board"], param)?;

        /* Pieces */
        let mut row: u8 = 0;
        let mut column: u8 = 0;

        self.game.board.iterator_pieces().for_each(|piece| {
            let position: Position = Position::new(row, column);

            column += 1;
            if column == 8 {
                row += 1;
                column = 0;
            }

            if *self.carry_piece.position() == Some(position.clone()) {
                return;
            }

            let image: Option<Image> = determine_image(&self.images, &piece);
            if image.is_none() {
                return;
            }

            let dest: Point2<f32> =
                determine_image_position(&position, &self.offsets, &self.scales);

            let param: DrawParam = DrawParam::new().dest(dest).scale(self.scales);
            let _ = graphics::draw(ctx, quad_ctx, &image.unwrap(), param);
        });

        /* Possible moves and takes */
        if let Some(carry_position) = self.carry_piece.position() {
            if self.game.player_turn && self.white_agent.is_none()
            || !self.game.player_turn && self.black_agent.is_none() {
                get_possible_moves(&self.game.board, self.game.player_turn, carry_position)
                    .into_iter()
                    .for_each(|to| {
                        let image: Image =
                            if Board::get_layer_value_at(self.game.board.get_empty_layer(), &to) {
                                self.images["outline green"].clone()
                            } else {
                                self.images["outline red"].clone()
                            };

                        let dest: Point2<f32> = determine_image_position(&to, &self.offsets, &self.scales);

                        let param: DrawParam = DrawParam::new().dest(dest).scale(self.scales);
                        let _ = graphics::draw(ctx, quad_ctx, &image, param);
                });
            }
        } else {
            if self.game.player_turn && self.white_agent.is_none()
            || !self.game.player_turn && self.black_agent.is_none() {
                self.game.board
                    .iterator_positions_and_pieces()
                    .filter(|(_, piece)| piece.get_color() == self.game.player_turn)
                    .for_each(|(pos, piece)| {
                        if get_possible_moves(
                            &self.game.board,
                            piece.get_color(),
                            &pos
                        ).into_iter().peekable().peek().is_some() {
                            let image: Image = self.images["outline green"].clone();
            
                            let dest: Point2<f32> = determine_image_position(&pos, &self.offsets, &self.scales);
            
                            let param: DrawParam = DrawParam::new().dest(dest).scale(self.scales);
                            let _ = graphics::draw(ctx, quad_ctx, &image, param);
                        }
                    });
            }
        }

        /* Show winner */
        if let Some(winner) = self.game.get_winner() {
            self.game.board
                .iterator_positions_and_pieces()
                .filter(|(_, piece)| piece.piece_type() != PieceType::Empty)
                .for_each(|(pos, piece)| {
                    let image = if piece.get_color() && winner == 1
                        || !piece.get_color() && winner == 0 {
                            self.images["outline green"].clone()
                        } else {
                            self.images["outline red"].clone()
                        };
            
                    let dest: Point2<f32> = determine_image_position(&pos, &self.offsets, &self.scales);
    
                    let param: DrawParam = DrawParam::new().dest(dest).scale(self.scales);
                    let _ = graphics::draw(ctx, quad_ctx, &image, param);
                });
        }

        /* Taken pieces panel */
        let image: Image = self.images.get("taken pieces panel empty").unwrap().clone();
        let dest: Point2<f32> = Point2::new(
            self.offsets.x + BOARD_BORDER * self.scales.x,
            self.offsets.y + (BOARD_BORDER + BOARD_SIZE + BOARD_BORDER) * self.scales.y);
            
        let param: DrawParam = DrawParam::new().dest(dest).scale(self.scales);
        graphics::draw( ctx, quad_ctx, &image, param)?;
            
        for image in determine_taken_pieces_images(&self.images, &self.game.board) {            
            graphics::draw( ctx, quad_ctx, &image, param)?;
        }

        /* Agent selection panel */
        let image: Image = determine_bottom_panel_image(&self.images, &self.white_agent, &self.black_agent);
        let dest: Point2<f32> = Point2::new(
            self.offsets.x + BOARD_BORDER * self.scales.x,
            self.offsets.y + (BOARD_BORDER + BOARD_SIZE + BOARD_BORDER + TAKE_PANEL_HEIGHT) * self.scales.y);
        
        let param: DrawParam = DrawParam::new().dest(dest).scale(self.scales);
        graphics::draw(ctx, quad_ctx, &image, param)?;

        /* Grabbed Piece */
        if let Some(piece) = self.carry_piece.piece() {
            let image: Option<Image> = determine_image(&self.images, piece);

            if image.is_some() {
                let dest: Point2<f32> = ctx.mouse_context.mouse_position() - (80.0 * self.scales);
                let param: DrawParam = DrawParam::new().dest(dest).scale(self.scales);
                graphics::draw(ctx, quad_ctx, &image.unwrap(), param)?;
            }
        }

        self.force_draw = false;

        graphics::present(ctx, quad_ctx)
    }

    fn resize_event(
        &mut self,
        ctx: &mut Context,
        _quad_ctx: &mut GraphicsContext,
        width: f32,
        height: f32,
    ) {
        self.offsets = Engine::calculate_offsets(width, height);
        self.scales = Engine::calculate_scale(width, height);
        ctx.gfx_context
            .set_screen_coordinates(Rect::new(0.0, 0.0, width, height));
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut GraphicsContext,
        _button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if let Some(button_index) = get_position_of_coordinates_bottom_panel(x, y, &self.offsets, &self.scales) {
            get_new_agents_after_button_press(&mut self.white_agent, &mut self.black_agent, button_index);
            self.force_draw = true;
            return
        }

        if reset_button_is_clicked(x, y, &self.offsets, &self.scales) {
            self.reset(ctx, quad_ctx);
            self.force_draw = true;
        }

        if self.game.player_turn && self.white_agent.is_some()
        || !self.game.player_turn && self.black_agent.is_some() {
            return
        }

        if let Some(position) = get_position_of_coordinates(x, y, &self.offsets, &self.scales) {
            let piece: Piece = self.game.board.get_piece_at(&position);

            if piece.get_color() == self.game.player_turn {
                self.carry_piece.set(&position, &piece);
            }
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut GraphicsContext,
        _button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if self.game.player_turn && self.white_agent.is_some()
        || !self.game.player_turn && self.black_agent.is_some() {
            self.carry_piece.clear();
            return
        }

        if let Some(position) = get_position_of_coordinates(x, y, &self.offsets, &self.scales) {
            if let Some(from_pos) = self.carry_piece.position() {
                self.perform_move(&from_pos.clone(), &position);
            }
        }
        
        self.carry_piece.clear();
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut GraphicsContext,
        keycode: miniquad::KeyCode,
        _keymods: event::KeyMods,
    ) {
        // Unimportant debug stuff
        // if keycode == miniquad::KeyCode::Enter {
        //     self.debug = !self.debug;
        //     if self.debug {
        //         snapshot::enter_debug(&self.game);

        //         self.game
        //             .board
        //             .clone()
        //             .iterator_positions_and_pieces()
        //             .flat_map(|(from_pos, piece)| {
        //                 get_possible_moves(
        //                     &self.game.board,
        //                     piece.get_color(),
        //                     from_pos.clone(),
        //                     true)
        //                     .map(move |to_pos| (from_pos.clone(), to_pos.clone()))
        //                     .map(|(from, to)| {
        //                         let mut new_board: Board = self.game.board.clone();
        //                         new_board.move_from_to(&from, &to);
        //                         new_board
        //                     })
        //             })
        //             .for_each(|board| {
        //                 println!("{}", board);
        //                 snapshot::save_state(&board);
        //             });
        //     } else {
        //         self.game.board = Board::import(snapshot::exit_debug());
        //     }
        // }

        if !self.debug {
            return;
        }

        if keycode == miniquad::KeyCode::Left {
            self.game.board = Board::import(snapshot::debug_left());
        } else if keycode == miniquad::KeyCode::Right {
            self.game.board = Board::import(snapshot::debug_right());
        }
    }
}

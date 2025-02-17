mod carry_piece;
mod logic;

use ggez::cgmath::{Point2, Vector2};
use ggez::event::EventHandler;
use ggez::graphics::{DrawParam, Image, Rect};
use ggez::{event, graphics, Context, GameError, GameResult};
use good_web_game as ggez;
use good_web_game::graphics::Color;

use miniquad::GraphicsContext;
use std::collections::HashMap;

use crate::core::board::Board;
use crate::core::game::Game;
use crate::core::move_generator::get_all_possible_moves;
use crate::core::piece::Piece;
use crate::core::position::Position;
use crate::core::snapshot;
use crate::ui::logic::get_position_of_coordinates;

use self::carry_piece::CarryPiece;
use self::logic::{determine_image, determine_image_position};

pub struct Engine {
    game: Game,

    images: HashMap<String, Image>,
    offsets: Point2<f32>,
    scales: Vector2<f32>,

    carry_piece: CarryPiece,

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

        Ok(Engine {
            game,
            images,
            offsets,
            scales,
            carry_piece,
            debug: false,
        })
    }

    fn load_images(ctx: &mut Context, quad_ctx: &mut GraphicsContext) -> HashMap<String, Image> {
        [
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
        .collect()
    }

    fn calculate_offsets(window_width: f32, window_height: f32) -> Point2<f32> {
        const GAME_IMAGES_WIDTH: f32 = 1280.0;
        const GAME_IMAGES_HEIGHT: f32 = 1280.0;

        let scale: Vector2<f32> = Engine::calculate_scale(window_width, window_height);

        let offset_x: f32 = (window_width - GAME_IMAGES_WIDTH * scale.x) / 2.0;
        let offset_y: f32 = (window_height - GAME_IMAGES_HEIGHT * scale.y) / 2.0;

        Point2::new(offset_x, offset_y)
    }

    fn calculate_scale(window_width: f32, window_height: f32) -> Vector2<f32> {
        const GAME_IMAGES_WIDTH: f32 = 1280.0;
        const GAME_IMAGES_HEIGHT: f32 = 1280.0;

        let window_ratio: f32 = window_width / window_height;
        let game_images_ratio: f32 = GAME_IMAGES_WIDTH / GAME_IMAGES_HEIGHT;

        let scale: f32 = if window_ratio > game_images_ratio {
            window_height / GAME_IMAGES_HEIGHT
        } else {
            window_width / GAME_IMAGES_WIDTH
        };

        Vector2::new(scale, scale)
    }
}

impl EventHandler<GameError> for Engine {
    fn update(&mut self, _ctx: &mut Context, _quad_ctx: &mut GraphicsContext) -> GameResult {
        if self.game.get_winner().is_some() {
            match self.game.get_winner().unwrap() {
                0 => println!("Black has won the game !!!"),
                1 => println!("White has won the game !!!"),
                _ => println!("Remis"),
            }
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
            get_all_possible_moves(
                &self.game.board,
                self.game.player_turn,
                carry_position,
                true,
            )
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

        /* Grabbed Piece */
        if let Some(piece) = self.carry_piece.piece() {
            let image: Option<Image> = determine_image(&self.images, piece);

            if image.is_some() {
                let dest: Point2<f32> = ctx.mouse_context.mouse_position() - (80.0 * self.scales);
                let param: DrawParam = DrawParam::new().dest(dest).scale(self.scales);
                graphics::draw(ctx, quad_ctx, &image.unwrap(), param)?;
            }
        }

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
        _ctx: &mut Context,
        _quad_ctx: &mut GraphicsContext,
        _button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
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
        if let Some(position) = get_position_of_coordinates(x, y, &self.offsets, &self.scales) {
            if let Some(from_pos) = self.carry_piece.position() {
                self.game.perform_move(from_pos, &position);
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
        if keycode == miniquad::KeyCode::Enter {
            self.debug = !self.debug;
            if self.debug {
                snapshot::enter_debug(&self.game);

                self.game
                    .board
                    .clone()
                    .iterator_positions_and_pieces()
                    .flat_map(|(from_pos, piece)| {
                        get_all_possible_moves(&self.game.board, piece.get_color(), &from_pos, true)
                            .into_iter()
                            .map(move |to_pos| (from_pos.clone(), to_pos.clone()))
                            .map(|(from, to)| {
                                let mut new_board: Board = self.game.board.clone();
                                new_board.move_from_to(&from, &to);
                                new_board
                            })
                    })
                    .for_each(|board| {
                        println!("{}", board);
                        snapshot::save_state(&board);
                    });
            } else {
                self.game.board = Board::import(snapshot::exit_debug());
            }
        }

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

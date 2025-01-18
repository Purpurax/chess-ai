mod logic;
mod carry_piece;

use good_web_game::graphics::Color;
use good_web_game as ggez;
use ggez::{event, graphics, GameError, GameResult, Context};
use ggez::event::EventHandler;
use ggez::graphics::{DrawParam, Image, Rect};
use ggez::cgmath::{Point2, Vector2};

use miniquad::GraphicsContext;
use std::collections::HashMap;

use crate::core::game::Game;
use crate::core::piece::Piece;
use crate::core::position::Position;
use crate::ui::logic::get_position_of_coordinates;

use self::carry_piece::CarryPiece;
use self::logic::{valid_click_coordinates, determine_image, determine_image_position};

pub struct Engine {
    game: Game,
    
    images: HashMap<String, Image>,
    offsets: Point2<f32>,
    scales: Vector2<f32>,

    carry_piece: CarryPiece
}

impl Engine {
    pub fn new(ctx: &mut Context, quad_ctx: &mut GraphicsContext) -> GameResult<Engine> {
        let game = Game::new();
        
        let images = Engine::load_images(ctx, quad_ctx);

        let (window_width, window_height) = graphics::drawable_size(quad_ctx);
        let offsets = Engine::calculate_offsets(window_width, window_height);
        let scales = Engine::calculate_scale(window_width, window_height);

        let carry_piece: CarryPiece = CarryPiece::new();

        Ok(Engine {
            game,
            images,
            offsets,
            scales,
            carry_piece
        })
    }

    fn load_images(ctx: &mut Context, quad_ctx: &mut GraphicsContext) -> HashMap<String, Image> {
        let board: Image = Image::new(ctx, quad_ctx, "/assets/board.png").unwrap();
        let black_pawn: Image = Image::new(ctx, quad_ctx, "/assets/pieces/piece_black_pawn.png").unwrap();
        let black_knight: Image = Image::new(ctx, quad_ctx, "/assets/pieces/piece_black_knight.png").unwrap();
        let black_bishop: Image = Image::new(ctx, quad_ctx, "/assets/pieces/piece_black_bishop.png").unwrap();
        let black_rook: Image = Image::new(ctx, quad_ctx, "/assets/pieces/piece_black_rook.png").unwrap();
        let black_queen: Image = Image::new(ctx, quad_ctx, "/assets/pieces/piece_black_queen.png").unwrap();
        let black_king: Image = Image::new(ctx, quad_ctx, "/assets/pieces/piece_black_king.png").unwrap();
        let white_pawn: Image = Image::new(ctx, quad_ctx, "/assets/pieces/piece_white_pawn.png").unwrap();
        let white_knight: Image = Image::new(ctx, quad_ctx, "/assets/pieces/piece_white_knight.png").unwrap();
        let white_bishop: Image = Image::new(ctx, quad_ctx, "/assets/pieces/piece_white_bishop.png").unwrap();
        let white_rook: Image = Image::new(ctx, quad_ctx, "/assets/pieces/piece_white_rook.png").unwrap();
        let white_queen: Image = Image::new(ctx, quad_ctx, "/assets/pieces/piece_white_queen.png").unwrap();
        let white_king: Image = Image::new(ctx, quad_ctx, "/assets/pieces/piece_white_king.png").unwrap();
        
        let images = HashMap::from([
            ("board".to_string(), board),
            ("black pawn".to_string(), black_pawn),
            ("black knight".to_string(), black_knight),
            ("black bishop".to_string(), black_bishop),
            ("black rook".to_string(), black_rook),
            ("black queen".to_string(), black_queen),
            ("black king".to_string(), black_king),
            ("white pawn".to_string(), white_pawn),
            ("white knight".to_string(), white_knight),
            ("white bishop".to_string(), white_bishop),
            ("white rook".to_string(), white_rook),
            ("white queen".to_string(), white_queen),
            ("white king".to_string(), white_king),
        ]);

        images
    }
    
    fn calculate_offsets(window_width: f32, window_height: f32) -> Point2<f32> {
        const GAME_IMAGES_WIDTH: f32 = 1280.0;
        const GAME_IMAGES_HEIGHT: f32= 1280.0;

        let scale: Vector2<f32> = Engine::calculate_scale(window_width, window_height);

        let offset_x: f32 = (window_width - GAME_IMAGES_WIDTH * scale.x) / 2.0;
        let offset_y: f32 = (window_height - GAME_IMAGES_HEIGHT * scale.y) / 2.0;

        Point2::new(offset_x, offset_y)
    }

    fn calculate_scale(window_width: f32, window_height: f32) -> Vector2<f32> {
        const GAME_IMAGES_WIDTH: f32 = 1280.0;
        const GAME_IMAGES_HEIGHT: f32= 1280.0;

        let window_ratio: f32 = window_width / window_height;
        let game_images_ratio: f32 = GAME_IMAGES_WIDTH / GAME_IMAGES_HEIGHT;

        let scale: f32 =
            if window_ratio > game_images_ratio {
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
            if self.game.get_winner().unwrap() {
                println!("White has won the game !!!");
            } else {
                println!("Black has won the game !!!");
            }
        }

        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut GraphicsContext) -> GameResult {
        /* Background */
        graphics::clear(ctx, quad_ctx, Color::from_rgb_u32(0x3F2832));
        
        let param: DrawParam = DrawParam::new().dest(self.offsets).scale(self.scales);
        graphics::draw(ctx, quad_ctx, &self.images["board"], param)?;

        /* Pieces */
        let mut row: u8 = 0;
        let mut column: u8 = 0;

        self.game.board.iterator_pieces().for_each(|piece| {
            let position: Position = Position::new(row as u8, column);

            column += 1;
            if column == 8 {
                row += 1;
                column = 0;
            }

            if self.carry_piece.has_grabbed() && self.carry_piece.position() == position {
                return
            }

            let image: Option<Image> = determine_image(&self.images, piece);
            if image.is_none() {
                return
            }

            let dest: Point2<f32> = determine_image_position(position, self.offsets, self.scales);

            let param: DrawParam = DrawParam::new().dest(dest).scale(self.scales);
            let _ = graphics::draw(ctx, quad_ctx, &image.unwrap(), param);
        });
        
        /* Grabbed Piece */
        if self.carry_piece.has_grabbed() {
            let piece: Piece = self.carry_piece.piece();
            let image: Option<Image> = determine_image(&self.images, piece);

            if image.is_some() {
                let dest: Point2<f32> = ctx.mouse_context.mouse_position() - (80.0*self.scales);
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
        ctx.gfx_context.set_screen_coordinates(Rect::new(0.0, 0.0, width, height));
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            _quad_ctx: &mut GraphicsContext,
            _button: event::MouseButton,
            x: f32,
            y: f32) {
        let logical_x: f32 = (x - self.offsets.x) / self.scales.x;
        let logical_y: f32 = (y - self.offsets.y) / self.scales.y;

        if !valid_click_coordinates(logical_x, logical_y) {
            return
        }

        let clicked_position = get_position_of_coordinates(logical_x, logical_y);
        let piece: Piece = self.game.board.get_piece_at(&clicked_position);

        if piece.get_color() == self.game.player_turn {
            self.carry_piece.set(clicked_position, piece);
        }
    }

    fn mouse_button_up_event(
            &mut self,
            _ctx: &mut Context,
            _quad_ctx: &mut GraphicsContext,
            _button: event::MouseButton,
            x: f32,
            y: f32) {
        if self.carry_piece.is_empty() {
            return
        }
        
        let logical_x: f32 = (x - self.offsets.x) / self.scales.x;
        let logical_y: f32 = (y - self.offsets.y) / self.scales.y;

        if !valid_click_coordinates(logical_x, logical_y) {
            self.carry_piece.clear();
            return
        }

        let clicked_position = get_position_of_coordinates(logical_x, logical_y);


        if !self.game.valid_turn(&self.carry_piece.position(), &clicked_position) {
            self.carry_piece.clear();
            return
        }

        self.game.perform_move(&self.carry_piece.position(), &clicked_position);
        self.game.next_player();

        self.carry_piece.clear();
    }
}
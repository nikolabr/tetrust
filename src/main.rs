extern crate sdl2;

use sdl2::event::Event;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

mod tetrust;
use crate::tetrust::tetrust::*;

fn tetris_collision(tetris: &mut Tetris, x: u16) -> Result<(), String> {
    println!("Collision!");
    tetris.disable_piece()?;
    tetris.set_piece(x, 0, rand::random())?;
    Ok(())
}


fn main() -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let mut font = ttf_context.load_font("assets/font.ttf", 128)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);
    let font_surface = font.render("Score: 0").blended(Color::RGB(255, 255, 255)).map_err(|e| e.to_string())?;

    let game_window = video_subsystem.window("rust-sdl2 demo", 640, 768)
        .position_centered()
        .build().map_err(|e| e.to_string())?;

    let mut game_canvas = game_window.into_canvas().software().build().map_err(|e| e.to_string())?;
    game_canvas.set_draw_color(BACKGROUND_COLOR);
    game_canvas.fill_rect(None)?;

    let texture_creator = game_canvas.texture_creator();
    let textures = TileTexture::new(&texture_creator)?;
    let font_texture = texture_creator.create_texture_from_surface(font_surface).map_err(|e| e.to_string())?;

    game_canvas.copy(&font_texture, None, Rect::new(64, 640, 192, 64))?;
    game_canvas.present();

    let pieces = Pieces::new();
    let mut tetris = Tetris::new(game_canvas, textures, TetrisPiece { x: 13, y: 0, piece: PieceEnum::I, buf: PieceEnum::buf(0, &PieceEnum::I), state: 0}, pieces, font);

    for i in 2..TILE_CANVAS_WIDTH - 2 {
        tetris.tile_canvas.set_tile(i, TILE_CANVAS_HEIGHT - 1, TileColor::Gray)?;
        tetris.tile_canvas.set_tile_state(i, TILE_CANVAS_HEIGHT - 1, false);
    }
    for i in 0..TILE_CANVAS_HEIGHT { 
        tetris.tile_canvas.set_tile(1, i, TileColor::Gray)?;
        tetris.tile_canvas.set_tile(TILE_CANVAS_WIDTH - 1, i, TileColor::Gray)?;
    }
    
    let mut ticks = 0;
    
    'running: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown{keycode: Some(Keycode::Up), ..} => { 
                    if tetris.rotate_piece()? == true {
                        tetris_collision(&mut tetris, rng.gen_range(2..TILE_CANVAS_WIDTH - 5))?;
                    }
                },
                Event::KeyDown{keycode: Some(Keycode::Down), ..} => {
                    if tetris.update_screen()? == true {
                        tetris_collision(&mut tetris, rng.gen_range(2..TILE_CANVAS_WIDTH - 5))?;
                    };
                },
                Event::KeyDown{keycode: Some(Keycode::Right), ..} => {
                    if tetris.move_piece(1, 0)? == true {
                        tetris_collision(&mut tetris, rng.gen_range(2..TILE_CANVAS_WIDTH - 5))?;
                    };

                },
                Event::KeyDown{keycode: Some(Keycode::Left), ..} => {
                    if tetris.move_piece(-1, 0)? == true {
                        tetris_collision(&mut tetris, rng.gen_range(2..TILE_CANVAS_WIDTH - 5))?;
                    };

                },
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ticks += 1;
        if ticks % 16 == 0 {
            if tetris.check_loss() == true { 
                break 'running 
            }

            if tetris.update_screen()? == true {
                tetris_collision(&mut tetris, rng.gen_range(2..TILE_CANVAS_WIDTH - 5))?;
            };
        }

        std::thread::sleep(std::time::Duration::from_millis(20));

    }
    Ok(())
}

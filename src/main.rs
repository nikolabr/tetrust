extern crate sdl2;

pub mod tetrust {
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };
    use std::ops::Index;
    use sdl2::video::*;
    use sdl2::render::Texture;
    use sdl2::surface::Surface;
    use sdl2::rect::Rect;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum TileColor {
        Empty, Red, Green, Blue, Purple, Cyan, Yellow, Orange
    }

    impl TileColor {
        fn get_asset_path(color: &TileColor) -> String {
            match color { 
                TileColor::Empty => "assets/empty.bmp".to_string(),
                TileColor::Red => "assets/red.bmp".to_string(),
                TileColor::Green => "assets/green.bmp".to_string(),
                TileColor::Blue => "assets/blue.bmp".to_string(),
                TileColor::Cyan => "assets/cyan.bmp".to_string(),
                TileColor::Purple => "assets/purple.bmp".to_string(),
                TileColor::Yellow => "assets/yellow.bmp".to_string(),
                TileColor::Orange => "assets/orange.bmp".to_string()
            }
        }
    }

    pub struct TileTexture<'t> {
        empty: Texture<'t>,
        red: Texture<'t>,
        green: Texture<'t>,
        blue: Texture<'t>,
        purple: Texture<'t>,
        cyan: Texture<'t>,
        yellow: Texture<'t>,
        orange: Texture<'t>,
    }

    impl<'t> TileTexture<'t> { 
        pub fn load_from_path(texture_creator: &'t sdl2::render::TextureCreator<WindowContext>, color: TileColor) -> Result<Texture<'t>, String> {
            let surface = Surface::load_bmp(TileColor::get_asset_path(&color))?;
            let texture = texture_creator.create_texture_from_surface(surface).map_err(|err| {err.to_string()})?;
            Ok(texture)
        }

        pub fn new(texture_creator: &'t sdl2::render::TextureCreator<WindowContext>) -> Result<TileTexture<'t>, String> {
            Ok(TileTexture { empty: TileTexture::load_from_path(texture_creator, TileColor::Empty)?,
                              red: TileTexture::load_from_path(texture_creator, TileColor::Red)?,
                              green: TileTexture::load_from_path(texture_creator, TileColor::Green)?,
                              blue: TileTexture::load_from_path(texture_creator, TileColor::Blue)?,
                              purple: TileTexture::load_from_path(texture_creator, TileColor::Purple)?,
                              cyan: TileTexture::load_from_path(texture_creator, TileColor::Cyan)?,
                              yellow: TileTexture::load_from_path(texture_creator, TileColor::Yellow)?,
                              orange: TileTexture::load_from_path(texture_creator, TileColor::Orange)?,
                            })
                }
    }

    impl<'t> Index<TileColor> for TileTexture<'t> {
        type Output = Texture<'t>; 

        fn index(&self, color: TileColor) -> &Self::Output {
            match color {
                TileColor::Empty => &self.empty,
                TileColor::Red => &self.red,
                TileColor::Green => &self.green,
                TileColor::Blue => &self.blue,
                TileColor::Purple => &self.purple,
                TileColor::Cyan => &self.cyan,
                TileColor::Yellow => &self.yellow,
                TileColor::Orange => &self.orange,
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Tile(TileColor, bool);

    pub struct TileCanvas<'t> { 
        canvas: sdl2::render::Canvas<Window>,
        textures: TileTexture<'t>,
        tiles: [[Tile; 16]; 20]
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum PieceEnum {
        O, S, Z, T, L, J, I 
    }

    pub struct Piece(pub Vec<Vec<u8>>, pub TileColor); 

    impl PieceEnum { 
        pub fn buf(state: u8, piece: &PieceEnum) -> (u16, u16){ 
            match piece {
                PieceEnum::O => (0, 2),
                PieceEnum::I => {
                    match state % 2 {
                        0 => (0, 0),
                        1 => (1, 2),
                        _ => (0, 0)
                    }
                }
                PieceEnum::S | PieceEnum::Z => {
                    match state % 2 {
                        0 => (0, 1),
                        1 => (0, 2),
                        _ => (0, 1)
                    }
                }
                _ => {
                    match state { 
                        1 => (1, 1),
                        3 => (0, 2),
                        _ => (0, 1)
                    }
                }
            }
        }
        pub fn rot(state: u8, piece: &PieceEnum) -> (Vec<(u8, u8)>, Vec<(u8, u8)>) {
            match piece {
                PieceEnum::O => (Vec::new(), Vec::new()),
                PieceEnum::I => match state % 2 { 
                    0 => (vec![(0, 1), (2, 1), (3, 1)], vec![(1, 0), (1, 2), (1, 3)]),
                    1 => (vec![(1, 0), (1, 2), (1, 3)], vec![(0, 1), (2, 1), (3, 1)]),
                    _ => (vec![(0, 1), (2, 1), (3, 1)], vec![(1, 0), (1, 2), (1, 3)]),
                },
                PieceEnum::S => match state % 2 {
                    0 => (vec![(0, 0), (2, 1)], vec![(0, 1), (0, 2)]),
                    1 => (vec![(0, 1), (0, 2)], vec![(0, 0), (2, 1)]),
                    _ => (vec![(0, 0), (3, 1)], vec![(0, 2), (0, 3)]),
                },
                PieceEnum::Z => match state % 2 {
                    0 => (vec![(0, 2), (2, 1)], vec![(0, 0), (0, 1)]),
                    1 => (vec![(0, 0), (0, 1)], vec![(0, 2), (2, 1)]),
                    _ => (vec![(0, 2), (2, 1)], vec![(0, 0), (0, 1)]),
                },
                PieceEnum::T => match state {
                    0 => (vec![(2, 1)], vec![(1, 0)]),
                    1 => (vec![(1, 0)], vec![(0, 1)]),
                    2 => (vec![(0, 1)], vec![(1, 2)]),
                    3 => (vec![(1, 2)], vec![(2, 1)]),
                    _ => (vec![(2, 1)], vec![(0, 0)]),
                },
                PieceEnum::L => match state {
                    0 => (vec![(0, 1), (2, 1), (2, 2)], vec![(1, 0), (0, 2), (1, 2)]),
                    1 => (vec![(1, 0), (2, 0), (1, 2)], vec![(0, 1), (2, 1), (2, 2)]),
                    2 => (vec![(0, 0), (0, 1), (2, 1)], vec![(1, 0), (2, 0), (1, 2)]),
                    3 => (vec![(1, 0), (0, 2), (1, 2)], vec![(0, 0), (0, 1), (2, 1)]),
                    _ => (vec![(0, 1), (2, 1), (2, 2)], vec![(1, 0), (0, 2), (1, 2)]),
                },
                PieceEnum::J => match state {
                    0 => (vec![(0, 1), (0, 2), (2, 1)], vec![(0, 0), (1, 0), (1, 2)]),
                    1 => (vec![(1, 0), (1, 2), (2, 2)], vec![(0, 1), (0, 2), (2, 1)]),
                    2 => (vec![(0, 1), (2, 0), (2, 1)], vec![(1, 0), (1, 2), (2, 2)]),
                    3 => (vec![(0, 0), (1, 0), (1, 2)], vec![(0, 1), (2, 0), (2, 1)]),
                    _ => (vec![(0, 1), (0, 2), (2, 1)], vec![(0, 0), (1, 0), (1, 2)]),
                }
            }
        }
    }

    pub struct Pieces { 
        pub o: Piece, 
        pub s: Piece, 
        pub z: Piece, 
        pub t: Piece, 
        pub l: Piece, 
        pub j: Piece, 
        pub i: Piece, 
    }

    impl Distribution<PieceEnum> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceEnum {
            match rng.gen_range(0..=6) {
                0 => PieceEnum::O,
                1 => PieceEnum::S,
                2 => PieceEnum::Z,
                3 => PieceEnum::T,
                4 => PieceEnum::L,
                5 => PieceEnum::J,
                _ => PieceEnum::I,
            }
        }
    }

    impl Pieces { 
        pub fn new() -> Pieces { 
            Pieces {
                o: Piece(vec![ vec![0, 1], vec![0, 1], vec![]], TileColor::Yellow),
                s: Piece(vec![ vec![1, 2], vec![0, 1], vec![]], TileColor::Green),
                z: Piece(vec![ vec![0, 1], vec![1, 2], vec![]], TileColor::Red),
                t: Piece(vec![ vec![1], vec![0, 1, 2], vec![]], TileColor::Purple),
                l: Piece(vec![ vec![2], vec![0, 1, 2], vec![]], TileColor::Orange),
                j: Piece(vec![ vec![0], vec![0, 1, 2], vec![]], TileColor::Blue),
                i: Piece(vec![ vec![], vec![0, 1, 2, 3], vec![]], TileColor::Cyan),
            }
        }
    }

    impl Index<PieceEnum> for Pieces {
        type Output = Piece; 

        fn index(&self, piece: PieceEnum) -> &Self::Output {
            match piece {
                PieceEnum::O => &self.o,
                PieceEnum::S => &self.s,
                PieceEnum::Z => &self.z,
                PieceEnum::T => &self.t,
                PieceEnum::L => &self.l,
                PieceEnum::J => &self.j,
                PieceEnum::I => &self.i,
            }
        }
    }

    impl<'t> TileCanvas<'t> { 
        pub fn new(tile_canvas: sdl2::render::Canvas<Window>, tile_textures: TileTexture<'t>) -> TileCanvas {
            TileCanvas { canvas: tile_canvas, tiles: [[Tile(TileColor::Empty, true); 16]; 20], textures: tile_textures}

        }
        pub fn update(&mut self) {
            self.canvas.present();
        }
        pub fn set_tile(&mut self, x: u16, y: u16, color: TileColor) -> Result<(), String>{ 
            let x_c = x as usize; 
            let y_c = y as usize;
            if self.tiles[x_c][y_c].1 == true {
                self.tiles[x_c][y_c].0 = color; 
                self.canvas.copy(&self.textures[color], None, Rect::new(x_c as i32 * 32, y_c as i32 * 32, 32, 32))?;
            }
            Ok(())
        }
        pub fn set_tile_state(&mut self, x: u16, y: u16, active: bool) {
            self.tiles[x as usize][(y % 16) as usize].1 = active
        }
        pub fn get_tile(&self, x: u16, y: u16) -> Option<Tile> { 
            let tile = self.tiles[(x % 20) as usize][(y % 16) as usize];
            match tile.0 { 
                TileColor::Empty => None,
                _ => match tile.1 {
                    true => Some(tile),
                    false => Some(Tile(TileColor::Empty, false))
                }
            }
        
        }
        pub fn draw_piece(&mut self, x: u16, y: u16, piece: &Piece) -> Result<(), String>{
            for i in 0..piece.0.len() {
                for t in piece.0[i].iter() {
                    self.set_tile(x + *t as u16, y + i as u16, piece.1)?;
                }
            }
            self.update();
            Ok(())
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct TetrisPiece { 
        pub x: u16, 
        pub y: u16,
        pub piece: PieceEnum,
        pub buf: (u16, u16),
        pub state: u8
    }

    pub struct Tetris<'t> {
        pub tile_canvas: TileCanvas<'t>,
        active_piece: TetrisPiece,
        pieces: Pieces
    }

    impl<'t> Tetris<'t> { 
        pub fn set_piece(&mut self, x_c: u16, y_c: u16, piece_enum: PieceEnum) -> Result<(u16, u16), String> {
            self.active_piece = TetrisPiece { x: x_c, y: y_c, piece: piece_enum, buf: PieceEnum::buf(0, &piece_enum), state: 0};
            self.tile_canvas.draw_piece(x_c, y_c, &self.pieces[piece_enum])?;
        
            Ok((x_c, y_c))
        }
        pub fn new(canvas: sdl2::render::Canvas<Window>, tile_textures: TileTexture<'t>, tetris_piece: TetrisPiece, tetris_pieces: Pieces) -> Tetris { 
            let mut tetris = Tetris { tile_canvas: TileCanvas::new(canvas, tile_textures), active_piece: tetris_piece, pieces: tetris_pieces};
            tetris.set_piece(tetris_piece.x, tetris_piece.y, tetris_piece.piece).unwrap();
            tetris
        }

        pub fn rotate_piece(&mut self) -> Result<bool, String> {
            let x_c = self.active_piece.x;
            let y_c = self.active_piece.y;
            let mut collision = false;
            if x_c >= 2 && (x_c + self.active_piece.buf.1 - if self.active_piece.piece == PieceEnum::I { 1 } else {0}  < 18) {

                let rots = PieceEnum::rot(self.active_piece.state, &self.active_piece.piece);
                if rots.0.iter().all(|&x| self.tile_canvas.get_tile(x_c + x.1 as u16, y_c + x.0 as u16).is_none()){
                    for r in rots.1 {
                        self.tile_canvas.set_tile(x_c + r.1 as u16, y_c + r.0 as u16, TileColor::Empty)?;
                    }
                    for r in rots.0 {
                        self.tile_canvas.set_tile(x_c + r.1 as u16, y_c + r.0 as u16, self.pieces[self.active_piece.piece].1)?;
                        collision |= match self.tile_canvas.get_tile(x_c + r.1 as u16, y_c + r.0 as u16 + 1) {
                                    Some(t) => !t.1, 
                                    None => false
                                };
                    }
                    self.tile_canvas.update();
                    self.active_piece.state += 1;
                    self.active_piece.state %= 4;
                
                    self.active_piece.buf = PieceEnum::buf(self.active_piece.state, &self.active_piece.piece);
                }
            }

            Ok(collision)
        }

        pub fn move_piece(&mut self, x: i16, y: i16) -> Result<bool, String>  { 
            let mut collision = false;
            let p = self.active_piece;
            if !(x > 0 && p.x + (4 - p.buf.1) > 17) && !(x < 0 && p.x + p.buf.0 < 3) {
                for i in match x < 0 {
                    false => [3, 2, 1, 0],
                    true => [0, 1, 2, 3]
                    }
                    { 
                    for j in (0..4).rev() { 
                        if let Some(Tile(tile, true)) = self.tile_canvas.get_tile(p.x + i, p.y + j) {
                                let x_i = p.x + i;
                                let y_j = p.y + j;
                                collision |= match self.tile_canvas.get_tile(x_i.wrapping_add(x as u16), (y_j + 1).wrapping_add(y as u16)) {
                                    Some(t) => !t.1, 
                                    None => false
                                };

                                self.tile_canvas.set_tile(x_i, y_j, TileColor::Empty)?;
                                self.tile_canvas.set_tile(x_i.wrapping_add(x as u16), (y_j).wrapping_add(y as u16), tile)?;
                                self.tile_canvas.update();
                            }
                        }
                    }
                self.active_piece.x = self.active_piece.x.wrapping_add(x as u16);
                self.active_piece.y = self.active_piece.y.wrapping_add(y as u16);

                self.tile_canvas.update();
                println!("{}, {:?}", collision, self.active_piece);
            }
            Ok(collision)
        }
        pub fn disable_piece(&mut self) {
            let p = self.active_piece; 
            for i in 0..5 {
                for j in 0..5 {
                    if let Some(..) = self.tile_canvas.get_tile(p.x + i, p.y + j) {
                        self.tile_canvas.set_tile_state(p.x + i, p.y + j, false);
                    }
                }
            }
        }

        pub fn update_screen(&mut self) -> Result<bool, String> {
            Ok(self.move_piece(0, 1)?)
        }
    }

}

use sdl2::event::Event;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use crate::tetrust::*;

fn main() -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let game_window = video_subsystem.window("rust-sdl2 demo", 640, 768)
        .position_centered()
        .build().map_err(|e| e.to_string())?;

    let game_canvas = game_window.into_canvas().software().build().map_err(|e| e.to_string())?;

    let texture_creator = game_canvas.texture_creator();
    let textures = TileTexture::new(&texture_creator)?;

    let pieces = Pieces::new();
    let mut tetris = Tetris::new(game_canvas, textures, TetrisPiece { x: 13, y: 0, piece: PieceEnum::T, buf: PieceEnum::buf(0, &PieceEnum::T), state: 0}, pieces);

    for i in 2..18 {
        tetris.tile_canvas.set_tile(i, 15, TileColor::Red)?;
        tetris.tile_canvas.set_tile_state(i, 15, false);
    }

    'running: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown{keycode: Some(Keycode::Up), ..} => { 
                    if tetris.rotate_piece()? == true {
                        println!("Collision!");
                        tetris.disable_piece();
                        tetris.set_piece(rng.gen_range(2..16), 0, rand::random())?;
                    }
                },
                Event::KeyDown{keycode: Some(Keycode::Down), ..} => {
                    if tetris.update_screen()? == true {
                        println!("Collision!");
                        tetris.disable_piece();
                        tetris.set_piece(rng.gen_range(2..16), 3, rand::random())?;
                    };

                    std::thread::sleep(std::time::Duration::from_millis(20));
                },
                Event::KeyDown{keycode: Some(Keycode::Right), ..} => {
                    tetris.move_piece(1, 0)?;
                    std::thread::sleep(std::time::Duration::from_millis(20));
                },
                Event::KeyDown{keycode: Some(Keycode::Left), ..} => {
                    tetris.move_piece(-1, 0)?;
                    std::thread::sleep(std::time::Duration::from_millis(20));
                },
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
    }
    Ok(())
}

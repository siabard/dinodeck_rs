use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::WindowCanvas;

pub struct State {
    map: [[usize; 8]; 8],
}

impl State {
    pub fn new() -> State {
        State {
            map: [
                [165, 165, 165, 165, 172, 165, 165, 165],
                [165, 165, 165, 165, 172, 165, 165, 165],
                [165, 165, 165, 165, 172, 165, 165, 165],
                [165, 165, 165, 165, 172, 165, 165, 165],
                [165, 165, 165, 165, 165, 165, 165, 165],
                [165, 165, 165, 165, 165, 165, 165, 165],
                [165, 165, 165, 165, 165, 165, 165, 165],
                [165, 165, 165, 165, 165, 165, 165, 165],
            ],
        }
    }

    pub fn update(&mut self, _dt: f64) {}

    pub fn render(&self, canvas: &mut WindowCanvas, texture: &Texture, textures: &Vec<Rect>) {
        for y in 0..8 {
            for x in 0..8 {
                canvas
                    .copy_ex(
                        &texture,
                        Some(textures[self.map[y][x]]),
                        Some(Rect::new(x as i32 * 16, y as i32 * 16, 16, 16)),
                        0.0,
                        None,
                        false,
                        false,
                    )
                    .unwrap();
            }
        }
    }
}

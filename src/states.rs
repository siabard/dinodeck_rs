use sdl2::keyboard::Keycode;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;

use crate::map;

pub struct State<'a> {
    pub x: i32,
    pub y: i32,
    map: map::Map<'a>,
}

impl<'a> State<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        path: &'static str,
    ) -> State<'a> {
        let map = map::Map::new(texture_creator, path).unwrap();

        State { x: 0, y: 0, map }
    }

    pub fn handle_key_event(&mut self, key: &Keycode) {
        match key {
            Keycode::Left => {
                self.x = 0.max(self.x - 1);
            }
            Keycode::Right => {
                self.x = 319.min(self.x + 1);
            }
            Keycode::Up => {
                self.y = 0.max(self.y - 1);
            }
            Keycode::Down => {
                self.y = 239.min(self.y + 1);
            }
            _ => {}
        }
    }

    pub fn update(&mut self, _dt: f64) {}

    pub fn render(&self, canvas: &mut WindowCanvas) {
        self.map.render(canvas, self.x, self.y);
    }
}

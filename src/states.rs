use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::WindowCanvas;

use crate::tile;

pub struct State<'a> {
    map: [[u32; 8]; 8],
    atlas: tile::TileAtlas,
    texture: &'a Texture<'a>,
}

impl<'a> State<'a> {
    pub fn new(texture: &'a Texture) -> State<'a> {
        let atlas = tile::TileAtlas::new(texture, 16, 16);
        State {
            map: [
                [0, 1, 2, 3, 4, 5, 6, 7],
                [8, 9, 10, 11, 12, 13, 14, 15],
                [16, 17, 146, 147, 148, 149, 150, 151],
                [152, 153, 154, 155, 156, 159, 160, 161],
                [22, 77, 143, 165, 165, 165, 165, 165],
                [33, 88, 145, 165, 165, 165, 165, 165],
                [44, 99, 1, 165, 165, 165, 165, 165],
                [55, 110, 156, 165, 165, 165, 165, 165],
            ],
            atlas,
            texture,
        }
    }

    pub fn update(&mut self, _dt: f64) {}

    pub fn render(&self, canvas: &mut WindowCanvas) {
        for y in 0..8 {
            for x in 0..8 {
                let map = self.map[y][x];
                let rect = self.atlas.get_tile_rect(self.texture, map);

                canvas
                    .copy_ex(
                        self.texture,
                        Some(rect),
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

use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;

use crate::map;
use crate::tile;

pub struct State<'a> {
    map: map::Map<'a>,
}

impl<'a> State<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        path: &'static str,
    ) -> State<'a> {
        let map = map::Map::new(texture_creator, path).unwrap();

        State { map }
    }

    pub fn update(&mut self, _dt: f64) {}

    pub fn render(&self, canvas: &mut WindowCanvas) {
        if let tiled::LayerData::Finite(tiles) = &self.map.layer.tiles {
            for y in 0..8 {
                for x in 0..8 {
                    if tiles[y][x].gid != 0 {
                        let rect = self.map.tile_atlas.get_tile_rect(tiles[y][x].gid - 1);

                        canvas
                            .copy_ex(
                                &self.map.texture,
                                Some(rect),
                                Some(Rect::new(
                                    x as i32 * self.map.tile_width as i32,
                                    y as i32 * self.map.tile_height as i32,
                                    self.map.tile_width,
                                    self.map.tile_height,
                                )),
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
    }
}

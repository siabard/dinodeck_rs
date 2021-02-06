/// Tiled를 읽어서 맵을 채운다.
use crate::tile;
use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::Texture, render::TextureCreator};
use sdl2::{render::WindowCanvas, video::WindowContext};
use std::fs::File;
use std::path::Path;
use tiled::parse;

pub struct Map<'a> {
    pub x: i32,     //  x
    pub y: i32,     //  y
    pub cam_x: i32, // camera_x
    pub cam_y: i32, // camera_y
    pub tile_atlas: tile::TileAtlas,
    pub width: u32,       // total number of tile in horizontal in this map
    pub height: u32,      // total numbr of tile in vertical in this map
    pub tile_width: u32,  // width of a tile in pixels
    pub tile_height: u32, // height of a tile in pixels
    pub layer: tiled::Layer,
    pub texture: Texture<'a>,
}

impl<'a> Map<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        path: &'static str,
    ) -> Option<Map<'a>> {
        let file = File::open(&Path::new(path)).unwrap();

        let map: tiled::Map = parse(file).unwrap();

        let layer: tiled::Layer = map.layers.into_iter().nth(0).unwrap();
        let tile_set: tiled::Tileset = map.tilesets.into_iter().nth(0).unwrap();
        let tile_width = tile_set.tile_width;
        let tile_height = tile_set.tile_height;

        let texture = texture_creator
            .load_texture(Path::new(
                &("assets/".to_owned() + &tile_set.images[0].source),
            ))
            .unwrap();

        let tile_atlas = tile::TileAtlas::new(&texture, tile_width, tile_height);
        Some(Map {
            x: 0,
            y: 0,
            cam_x: 0,
            cam_y: 0,
            tile_atlas,
            width: map.width,
            height: map.height,
            tile_width,
            tile_height,
            layer,
            texture,
        })
    }

    /// translate position (left, top) to tile
    /// map is display rom x, y
    pub fn point_to_tile(&self, left: i32, top: i32) -> (i32, i32) {
        let o_x = self.x.max(left);
        let o_y = self.y.max(top);

        let clamp_x = o_x.min(left + (self.width * self.tile_width) as i32 - 1);
        let clamp_y = o_y.min(top + (self.height * self.tile_height) as i32 - 1);

        let tile_x = (clamp_x - self.x) / self.tile_width as i32;
        let tile_y = (clamp_y - self.y) / self.tile_height as i32;

        (tile_x, tile_y)
    }

    pub fn render(&self, canvas: &mut WindowCanvas, x: i32, y: i32) {
        if let tiled::LayerData::Finite(tiles) = &self.layer.tiles {
            let (tile_left, tile_top) = self.point_to_tile(x, y);
            let (tile_right, tile_bottom) = self.point_to_tile(x + 320, y + 200);

            for y in tile_top..tile_bottom {
                for x in tile_left..tile_right {
                    let gid = tiles[y as usize][x as usize].gid;
                    if gid != 0 {
                        let rect = self.tile_atlas.get_tile_rect(gid - 1);

                        canvas
                            .copy_ex(
                                &self.texture,
                                Some(rect),
                                Some(Rect::new(
                                    (x - tile_left) as i32 * self.tile_width as i32,
                                    (y - tile_top) as i32 * self.tile_height as i32,
                                    self.tile_width,
                                    self.tile_height,
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

/// Tiled를 읽어서 맵을 채운다.
use crate::tile;
use sdl2::video::WindowContext;
use sdl2::{image::LoadTexture, render::Texture, render::TextureCreator};
use std::fs::File;
use std::path::Path;
use tiled::parse;

pub struct Map<'a> {
    pub x: i32,
    pub y: i32,
    pub cam_x: i32,
    pub cam_y: i32,
    pub tile_atlas: tile::TileAtlas,
    pub width: u32,
    pub height: u32,
    pub tile_width: u32,
    pub tile_height: u32,
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
}

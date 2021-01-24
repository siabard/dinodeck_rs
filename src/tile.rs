use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureQuery;

/// TileAtlas
/// struct tiles
pub struct TileAtlas {
    /// width of texutre
    pub texture_width: u32,
    /// height of texture
    pub texture_height: u32,
    /// total number of horizontal tiles
    pub hor_length: u32,
    /// total number of vertical tiles,
    pub ver_length: u32,
    /// atlas table
    pub atlas: Vec<(f64, f64, f64, f64)>,
}

impl TileAtlas {
    pub fn new(texture: &Texture, w: u32, h: u32) -> TileAtlas {
        let query: TextureQuery = texture.query();

        let mut atlas: Vec<(f64, f64, f64, f64)> = vec![];
        let x_size: u32 = query.width / w;
        let y_size: u32 = query.height / h;

        let u_unit: f64 = w as f64 / query.width as f64;
        let v_unit: f64 = h as f64 / query.height as f64;

        let mut u_acc: f64;
        let mut v_acc: f64 = 0.0;
        for _ in 0..y_size {
            u_acc = 0.0;
            let next_v_acc = v_acc + v_unit;
            for _ in 0..x_size {
                let next_u_acc = u_acc + u_unit;
                atlas.push((u_acc, v_acc, next_u_acc, next_v_acc));
                u_acc = next_u_acc;
            }
            v_acc = next_v_acc;
        }

        let hor_length: u32 = query.width / w;
        let ver_length: u32 = query.height / h;

        TileAtlas {
            texture_width: query.width,
            texture_height: query.height,
            hor_length,
            ver_length,
            atlas,
        }
    }

    /// return rect of tile
    pub fn get_tile_rect(&self, texture: &Texture, map: u32) -> Rect {
        let tile_uv = self.atlas[map as usize];
        Rect::new(
            (texture.query().width as f64 * tile_uv.0) as i32,
            (texture.query().height as f64 * tile_uv.1) as i32,
            (texture.query().width as f64 * (tile_uv.2 - tile_uv.0)) as u32,
            (texture.query().height as f64 * (tile_uv.3 - tile_uv.1)) as u32,
        )
    }
}

/// translate position to tile
/// x, y : position to translate to tile
/// tile_size : size of tile in pixels
/// left, top: left / top of map in pixel
/// map_width, map_height: number of tile on width / height
pub fn point_to_tile(
    x: i32,
    y: i32,
    tile_size: u32,
    left: i32,
    top: i32,
    map_width: u32,
    map_height: u32,
) -> (i32, i32) {
    let o_x = x.max(left);
    let o_y = y.max(top);

    let clamp_x = o_x.min(left + (map_width * tile_size) as i32 - 1);
    let clamp_y = o_y.min(top + (map_height * tile_size) as i32 - 1);

    let tile_x = (clamp_x - left) / tile_size as i32;
    let tile_y = (clamp_y - top) / tile_size as i32;

    (tile_x, tile_y)
}

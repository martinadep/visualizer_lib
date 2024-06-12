use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::Tile;
use tetra::{Context, State};
use tetra::graphics::{DrawParams, Texture};
use tetra::math::Vec2;

use crate::visualizer::{PIXEL, TOP_OFFSET};
use crate::visualizer::textures::{Drawable, upload_contentset, upload_tileset};
use crate::visualizer::visweather::VisWeather;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

///visualizable map structure
pub struct VisMap {
    discovered_map: Vec<Vec<Option<Tile>>>,
    visweather: VisWeather,

    world_size: usize,
    robot_position: (usize, usize),
}

impl VisMap {
    ///create a new viusalizable [size x size] map of 'None' contents
    pub fn new(size: usize) -> Self {
        let new_mappa = vec![vec![None; size]; size];

        Self {
            visweather: VisWeather::new(WeatherType::Rainy),
            discovered_map: new_mappa,
            robot_position: (1, 1),
            world_size: size,
        }
    }
    pub fn new_draw(&mut self, ctx: &mut Context, map_pos: (f32, f32), scale: f32, style : usize) {
        self.visweather.update(ctx);
        let mut y_pixel = map_pos.1;
        let mut x_pixel = map_pos.0;

        let tileset = upload_tileset(ctx, style);
        let contentset = upload_contentset(ctx, style);

        for (xrobot, row) in self.discovered_map.iter().enumerate() {
            for (yrobot, opt_tile) in row.iter().enumerate() {
                if opt_tile.is_some()
                    && x_pixel < WINDOW_WIDTH as f32
                    && y_pixel < WINDOW_HEIGHT as f32
                    && x_pixel >= 0.0
                    && y_pixel > 0.0
                {
                    let tile = opt_tile.clone().unwrap();
                    tile.tile_type.draw(tileset.clone(), ctx,
                                        DrawParams::new()
                                            .position(Vec2::new(x_pixel, TOP_OFFSET + y_pixel))
                                            .scale(Vec2::new(scale, scale)));
                    tile.content.draw(contentset.clone(), ctx,
                                      DrawParams::new()
                                          .position(Vec2::new(x_pixel, TOP_OFFSET + y_pixel))
                                          .scale(Vec2::new(scale, scale)));
                    if yrobot == self.robot_position.1 && xrobot == self.robot_position.0 {
                        Texture::new(ctx, "./resources/robot.png")
                            .expect("failed to upload robot image")
                            .draw(ctx,
                                  DrawParams::new()
                                      .position(Vec2::new(x_pixel, TOP_OFFSET + y_pixel))
                                      .scale(Vec2::new(scale, scale)),
                            );
                    }
                }

                y_pixel += PIXEL * scale;
            }
            y_pixel = map_pos.1;
            x_pixel += PIXEL * scale;
        }
        self.visweather.draw(ctx);
    }
    pub(crate) fn update_map(&mut self, view: Vec<Vec<Option<Tile>>>) {
        let mut valid_cells = vec![vec![true; 3]; 3];
        let c_row = self.robot_position.0;
        let c_col = self.robot_position.1;
        if c_row == 0 {
            valid_cells[0][0] = false;
            valid_cells[0][1] = false;
            valid_cells[0][2] = false;
        }
        if c_col == 0 {
            valid_cells[0][0] = false;
            valid_cells[1][0] = false;
            valid_cells[2][0] = false;
        }
        if c_row == self.world_size - 1 {
            valid_cells[2][0] = false;
            valid_cells[2][1] = false;
            valid_cells[2][2] = false;
        }
        if c_col == self.world_size - 1 {
            valid_cells[0][2] = false;
            valid_cells[1][2] = false;
            valid_cells[2][2] = false;
        }
        for (i, row) in valid_cells.iter().enumerate() {
            for (j, is_valid) in row.iter().enumerate() {
                if *is_valid {
                    if let None = self.discovered_map[c_row + i - 1][c_col + j - 1] {
                        let tile = view[i][j].clone().unwrap();
                        self.discovered_map[c_row + i - 1][c_col + j - 1] = Some(tile);
                    }
                }
            }
        }
    }
    ///updates the robot (texture pointer) position on the map
    pub fn update_robot_pos(&mut self, new_pos: (usize, usize)) {
        self.robot_position = new_pos;
    }

    ///updates the weather
    pub fn update_weather(&mut self, weather_type: WeatherType) {
        println!("weather updated from {:?} to {:?}", self.visweather, weather_type);
        self.visweather = VisWeather::new(weather_type);
    }
}
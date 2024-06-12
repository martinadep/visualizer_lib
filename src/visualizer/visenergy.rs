use robotics_lib::energy::Energy;
use tetra::Context;
use tetra::graphics::text::{Font, Text};
use tetra::math::Vec2;

use crate::visualizer::{PIXEL, SCALE};
use crate::WINDOW_WIDTH;

///visualizable texts structure
pub struct VisEnergy {
    font: Font,
    energy_text: Text,
    rizzler_text: Text,
}

impl VisEnergy {
    pub fn new(ctx: &mut Context) -> Self {
        let f = Font::vector(ctx, "./resources/fonts/roboto.ttf", 17.0)
            .expect("failed to upload font");
        Self {
            font: f.clone(),
            energy_text: Text::new(format!("Robot Energy : {:?}", Energy::default()), f.clone()),
            rizzler_text: Text::new("...", f.clone()),
        }
    }
    pub fn draw(&mut self, ctx: &mut Context) {
        self.energy_text.draw(ctx, Vec2::new(WINDOW_WIDTH as f32 / 1.25 - PIXEL * SCALE, 0.0));
        self.rizzler_text.draw(ctx, Vec2::new(10.0, 30.0));
    }
    pub fn update_energy(&mut self, new_energy: usize) {
        self.energy_text.set_content(format!("Robot Energy : {}", new_energy));
    }
    pub fn update_rizz(&mut self, new_rizz: String) {
        self.rizzler_text.set_content(format!("{}", new_rizz));
    }
}

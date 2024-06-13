use std::sync::mpsc::Receiver;

use rand::Rng;
use robotics_lib::world::world_generator::Generator;
use tetra::ContextBuilder;

use crate::visualizer::{VisData, Visualizer};

pub mod visualizer;

pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 800;

///Builds and runs the window
pub fn start(vis_receiver: Receiver<VisData>, size: usize) {
    let mut c = ContextBuilder::new("tyrannosauRUST-rex | Chi dorme non piglia pesci", WINDOW_WIDTH, WINDOW_HEIGHT)
        .show_mouse(true)
        .quit_on_escape(true)
        .resizable(true)
        .build().expect("failed to build context");

    let vis = Visualizer::new(&mut c, size, vis_receiver)
        .expect("failed to create visualizer");

    c.run(|_ctx| {
        Ok(vis)
    }).expect("failed to run");
}
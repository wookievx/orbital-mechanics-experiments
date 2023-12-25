pub mod browser_logic;
pub mod orbit_logic;
pub use browser_logic::BroweserCanvas;
use crate::model::{orbit::OrbitTimeSnapshot, Vector2d};

pub trait OrbitDrawer {
    fn draw_orbit_state(&self, state: &OrbitTimeSnapshot, center_offset: Vector2d<f64>, canvas_width: f64, canvas_height: f64);
}

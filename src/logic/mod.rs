pub mod browser_logic;
pub mod orbit_logic;
use crate::model::{orbit::OrbitTimeSnapshot, Planet, Vector2d};
pub use browser_logic::BroweserCanvas;
use leptos::leptos_dom::logging::console_log;

pub trait OrbitDrawer {
    fn draw_orbit_state(
        &self,
        state: &OrbitTimeSnapshot,
        center_offset: Vector2d<f64>,
        canvas_rect_size: f64,
        max_orbit: f64,
    );
}

pub trait PlanetDrawer {
    fn draw_planet(
        &self,
        planet: &Planet,
        center_offset: Vector2d<f64>,
        canvas_rect_size: f64,
        max_orbit: f64,
    );
}

pub fn draw_orbits<T: OrbitDrawer + PlanetDrawer>(
    drawer: &T,
    planet: &Planet,
    orbits: Vec<&OrbitTimeSnapshot>,
    center_offset: Vector2d<f64>,
    canvas_rect_size: f64,
) {
    let max_orbit = orbits
        .iter()
        .map(|o| o.orbit.a.clone())
        .max_by(|l, r| l.total_cmp(r));
    if let Some(max_orbit) = max_orbit {
        drawer.draw_planet(planet, center_offset.clone(), canvas_rect_size, max_orbit);
        for orbit in orbits {
            drawer.draw_orbit_state(orbit, center_offset.clone(), canvas_rect_size, max_orbit);
        }
    } else {
        console_log("no orbits to draw");
    }
}

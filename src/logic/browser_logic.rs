use leptos::leptos_dom::logging::console_log;
use web_sys::{CanvasRenderingContext2d, wasm_bindgen::JsValue};

use crate::model::Vector2d;

use super::OrbitDrawer;

pub struct BroweserCanvas(pub CanvasRenderingContext2d);

impl OrbitDrawer for BroweserCanvas {
    fn draw_orbit_state(
        &self,
        state: &crate::model::OrbitTimeSnapshot,
        center_offset: Vector2d<f64>,
        canvas_width: f64,
        canvas_height: f64
    ) {
        let eccentricity_2 = state.orbit.e.clone() * state.orbit.e.clone();
        let eccentricity = eccentricity_2.sqrt();
        let radius_x = canvas_width * 0.4;
        let ratio = radius_x / state.orbit.a;
        let radius_y = state.orbit.a * (1_f64 - eccentricity_2).sqrt() * ratio;
        let Vector2d { x, y } = center_offset;
        let rotation_cos = Vector2d {
            x: radius_x,
            y: 0_f64,
        } * state.orbit.e.clone()
            / eccentricity
            / radius_x;
        let rotation = rotation_cos.acos();

        console_log(
            format!(
                "Got parameters: {}, {}, {}, {}, {}, {}",
                &eccentricity, &x, &y, &radius_x, &radius_y, &rotation
            )
            .as_str(),
        );

        //commence drawing
        let pi = std::f64::consts::PI;

        self.0.begin_path();
        self.0.set_stroke_style(&JsValue::from_str("#000"));
        self.0.set_line_width(1_f64);
        self.0
            .ellipse(x, y, radius_x, radius_y, pi * 2.0 - rotation, 0.0, pi * 2.0)
            .unwrap();
        self.0.stroke();
        
        //draw orbiting body
        console_log(
            format!("Got true-anomaly: {}", &state.true_anomaly).as_str()
        );
        let arc_length_half = pi * 0.01;
        self.0.begin_path();
        self.0.set_line_width(10_f64);
        self.0.set_stroke_style(&JsValue::from_str("red"));
        self.0
            .ellipse(x, y, radius_x, radius_y, pi * 2.0 - rotation, state.true_anomaly - arc_length_half, state.true_anomaly + arc_length_half)
            .unwrap();
        self.0.stroke();
    }
}

use super::Vector2d;

#[derive(Debug, Clone)]
pub struct Orbit {
    pub e: Vector2d<f64>,
    pub a: f64,
    pub mue: f64
}

pub struct OrbitTimeSnapshot<'o> {
    pub orbit: &'o Orbit,
    pub time: f64,
    pub true_anomaly: f64,
}
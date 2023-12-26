
#[derive(Debug, Clone)]
pub struct Planet {
    pub name: &'static str,
    pub mass: f64,
    pub radius: f64,
    pub display_color: &'static str
}

impl Planet {
    pub fn new(name: &'static str, mass: f64, radius: f64, display_color: &'static str) -> Self {
        Planet { name, mass, radius, display_color }
    }
}
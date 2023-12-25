use crate::model::{Orbit, OrbitTimeSnapshot};

impl Orbit {
    pub fn calculate_orbit_at<'a>(&'a self, delta_t: f64) -> OrbitTimeSnapshot {
        let true_anomaly = self.calculate_true_anomaly(
            self.calculate_mean_anomaly(delta_t, self.calculate_mean_motion()),
        );

        OrbitTimeSnapshot {
            orbit: self,
            time: delta_t,
            true_anomaly,
        }
    }

    fn calculate_mean_motion(&self) -> f64 {
        let gravitational_constant = 6.67430e-11; // Gravitational constant in m^3 kg^-1 s^-2

        (gravitational_constant * self.mue / self.a.powi(3)).sqrt()
    }

    // Function to calculate mean anomaly for a given time
    fn calculate_mean_anomaly(&self, delta_t: f64, mean_motion: f64) -> f64 {
        let mean_anomaly = mean_motion * delta_t;
        let pi = std::f64::consts::PI;
        mean_anomaly % (2.0 * pi) // Ensure the result is within the range [0, 2*pi)
    }

    // Function to calculate true anomaly based on mean anomaly
    fn calculate_true_anomaly(&self, mean_anomaly: f64) -> f64 {
        // Previous true anomaly calculation method...
        let mut eccentric_anomaly = mean_anomaly;
        let tolerance = 1e-9; // Tolerance for iteration
        let max_iterations = 100;
        let eccentricity = (self.e.clone() * self.e.clone()).sqrt();

        for _ in 0..max_iterations {
            let next_eccentric_anomaly = eccentric_anomaly
                - (eccentric_anomaly - eccentricity * eccentric_anomaly - mean_anomaly)
                    / (1.0 - eccentricity * f64::cos(eccentric_anomaly));

            if (next_eccentric_anomaly - eccentric_anomaly).abs() < tolerance {
                eccentric_anomaly = next_eccentric_anomaly;
                break;
            }

            eccentric_anomaly = next_eccentric_anomaly;
        }

        // Calculate true anomaly using the eccentric anomaly
        let true_anomaly = 2.0
            * f64::atan2(
                f64::sqrt(1.0 + eccentricity) * f64::sin(eccentric_anomaly / 2.0),
                f64::sqrt(1.0 - eccentricity) * f64::cos(eccentric_anomaly / 2.0),
            );

        true_anomaly
    }
}

impl <'a> OrbitTimeSnapshot<'a> {
    pub fn calculate_orbital_velocity(&self) -> f64 {
        let gravitational_constant = 6.67430e-11; // Gravitational constant in m^3 kg^-1 s^-2
        let eccentricity = (self.orbit.e.clone() * self.orbit.e.clone()).sqrt();

        // Calculate current distance from the central body
        let current_distance = self.orbit.a * (1.0 - eccentricity.powi(2)) / (1.0 + eccentricity * self.true_anomaly.cos());

        // Calculate orbital velocity using the vis-viva equation
        (gravitational_constant * self.orbit.mue * (2.0 / current_distance - 1.0 / self.orbit.a)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Vector2d;

    use super::*;

    #[test]
    fn test_calculate_orbital_velocity() {
        let orbit = Orbit {
            e: Vector2d { x: 0.4, y: 0.3 },
            a: 7.0e6,
            mue: 5.972e24,
        };
        let orbit_at = orbit.calculate_orbit_at(100.0 * 60.0);
        let velocity = orbit_at.calculate_orbital_velocity();
        println!("{}", orbit_at.true_anomaly.to_degrees());
        println!("{}", velocity);
    }
}
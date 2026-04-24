use rand::RngExt;

#[derive(Debug)]
pub struct Target {
    number_of_shots: usize,
    radians_x: f32,
    radians_z: f32,
    distance: f32,
    loc: (f32, f32, f32),
}

impl Target {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let radians_x = rng.random_range(0.0..std::f32::consts::PI);
        let radians_z = rng.random_range(0.0..std::f32::consts::PI);

        // Porting note: the game would allow targets to be generated within
        // 20 km of the origin, which would make it impossible to destroy without
        // blowing yourself up. Instead, I'm modifying the original so that the
        // target's distance is at least 100km away.
        let distance = rng.random_range(100.0..100000.0);

        let loc = (
            radians_z.sin() * radians_x.cos() * distance,
            radians_z.sin() * radians_x.sin() * distance,
            radians_z.cos() * distance,
        );
        Self {
            // Porting note: the game would start the player off as though they had
            // already taken one shot, which makes the first round different to all
            // the subsequent rounds. Instead, I'm starting the shot count off at 0
            // so that there's no difference between the first round and the rest.
            number_of_shots: 0,
            radians_x,
            radians_z,
            distance,
            loc,
        }
    }

    pub fn approx_location(&self) -> TargetInfo {
        TargetInfo {
            loc: self.loc,
            radians_x: self.radians_x,
            radians_z: self.radians_z,
        }
    }

    pub fn estimated_distance(&self) -> f32 {
        match self.number_of_shots {
            0 => (self.distance / 20.0).round() * 20.0,
            1 => (self.distance / 10.0).round() * 10.0,
            2 => (self.distance / 5.0).round() * 5.0,
            3 => self.distance.round(),
            _ => self.distance,
        }
    }

    pub fn fire(&mut self, degrees_x: f32, degrees_z: f32, distance: f32) -> FireResult {
        self.number_of_shots += 1;

        if distance < 20.0 {
            return FireResult::SelfDestructed;
        }

        let radians_x = degrees_x.to_radians();
        let radians_z = degrees_z.to_radians();
        let x1 = distance * radians_z.sin() * radians_x.cos();
        let y1 = distance * radians_z.sin() * radians_x.sin();
        let z1 = distance * radians_z.cos();
        let delta_x = x1 - self.loc.0;
        let delta_y = y1 - self.loc.1;
        let delta_z = z1 - self.loc.2;
        let distance_from_target =
            (delta_x * delta_x + delta_y * delta_y + delta_z * delta_z).sqrt();

        if distance_from_target > 20.0 {
            FireResult::Miss(Explosion {
                radians_x,
                radians_z,
                delta: (delta_x, delta_y, delta_z),
                coords: (x1, y1, z1),
                distance_from_target,
            })
        } else {
            FireResult::Hit(self.number_of_shots, distance_from_target)
        }
    }
}

#[derive(Debug)]
pub enum FireResult {
    SelfDestructed,
    Miss(Explosion),
    Hit(usize, f32), // number of shots required, distance from target
}

#[derive(Debug)]
pub struct Explosion {
    pub radians_x: f32,
    pub radians_z: f32,
    pub delta: (f32, f32, f32),
    pub coords: (f32, f32, f32),
    pub distance_from_target: f32,
}

#[derive(Debug)]
pub struct TargetInfo {
    pub loc: (f32, f32, f32),
    pub radians_x: f32,
    pub radians_z: f32,
}

impl Default for Target {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_target() {
        let target = Target::new();
        assert_eq!(0, target.number_of_shots);
    }

    #[test]
    fn test_in_range() {
        let target = Target::new();
        assert!((-100000.0f32..100000.0).contains(&target.loc.0));
        assert!((-100000.0f32..100000.0).contains(&target.loc.1));
        assert!((-100000.0f32..100000.0).contains(&target.loc.2));
    }
}

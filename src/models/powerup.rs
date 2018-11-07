use geometry::{Collide, Point, Position, Size};
use rand::Rng;

const POWERUP_TTL: f32 = 10.0;
const POWERUP_SIZE: f32 = 20.0;

/// This is an enum of the different powerup types
#[derive(PartialEq, Clone, Copy)]
pub enum PowerupKind {
    /// Provides the player with a temporary shield
    Shield,
    /// Slows down enemies (as well as stars in backdrop)
    TimeSlow,
    /// Shoots three bullets at once in different directions
    TripleShot,
}

impl PowerupKind {
    pub fn random<R: Rng>(rng: &mut R) -> PowerupKind {
        let choices = [
            PowerupKind::Shield,
            PowerupKind::TimeSlow,
            PowerupKind::TripleShot,
        ];
        *rng.choose(&choices).unwrap()
    }
}

impl Default for PowerupKind {
    fn default() -> PowerupKind {
        PowerupKind::TripleShot
    }
}

/// This is the struct containing information about a powerup
pub struct Powerup {
    pub ttl: f32,
    pub kind: PowerupKind,
    pub color: u8,
    pub position: Point,
}

impl Powerup {
    pub fn new(kind: PowerupKind, position: Point) -> Powerup {
        Powerup {
            ttl: POWERUP_TTL,
            kind: kind,
            color: 0,
            position: position,
        }
    }

    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> Powerup {
        Powerup::new(PowerupKind::random(rng), Point::random(rng, bounds))
    }

    pub fn update(&mut self, elapsed_time: f32) {
        self.ttl -= elapsed_time;
        self.color = if self.color < u8::max_value() {
            self.color + 1
        } else {
            0
        }
    }
}

impl Position for Powerup {
    fn x(&self) -> f32 {
        self.position.x
    }
    fn y(&self) -> f32 {
        self.position.y
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.position.x
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.position.y
    }

    fn position(&self) -> Point {
        Point::new(self.x(), self.y())
    }
}

impl Collide for Powerup {
    fn radius(&self) -> f32 {
        POWERUP_SIZE * (self.ttl / POWERUP_TTL)
    }
}

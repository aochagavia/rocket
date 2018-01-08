use rand::Rng;
use geometry::{Position, Collide, Point, Size};

const POWERUP_TTL: f64 = 10.0;
const POWERUP_SIZE: f64 = 20.0;

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
            PowerupKind::TripleShot
        ];
        *rng.choose(&choices).unwrap()
    }
}

impl Default for PowerupKind {
    fn default() -> PowerupKind { PowerupKind::TripleShot }
}

/// This is the struct containing information about a powerup
pub struct Powerup {
    pub ttl: f64,
    pub kind: PowerupKind,
    pub color: u8,
    pub position: Point
}

impl Powerup {
    pub fn new(kind: PowerupKind, position: Point) -> Powerup {
        Powerup {
            ttl: POWERUP_TTL,
            kind: kind,
            color: 0,
            position: position
        }
    }

    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> Powerup {
        Powerup::new(PowerupKind::random(rng), Point::random(rng, bounds))
    }

    pub fn update(&mut self, elapsed_time: f64) {
        self.ttl -= elapsed_time;
        self.color = if self.color < u8::max_value() { self.color + 1 } else { 0 }
    }
}

impl Position for Powerup {
    fn x(&self) -> f64 { self.position.x }
    fn y(&self) -> f64 { self.position.y }
    fn x_mut(&mut self) -> &mut f64 { &mut self.position.x }
    fn y_mut(&mut self) -> &mut f64 { &mut self.position.y }

    fn position(&self) -> Point {
        Point::new(self.x(), self.y())
    }
}

impl Collide for Powerup {
    fn radius(&self) -> f64 { POWERUP_SIZE * (self.ttl / POWERUP_TTL) }
}
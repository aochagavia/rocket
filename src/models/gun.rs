const MAX_TEMP: f32 = 1.0;
const MIN_TEMP: f32 = 0.0;
const HEAT_PER_SHOT: f32 = 0.05;
const NATURAL_COOL_DOWN_RATE: f32 = 0.4;
const OVERHEAT_COOL_DOWN_RATE: f32 = 0.2;

/// This is the player's gun: it slowly overheats with every shot so that the player can't just 
/// spam shooting and ruin the gameplay
#[derive(Default)]
pub struct Gun {
    /// The current temperature of the gun - a percentage between 0 and 1
    pub temperature: f32,
    /// Whether or not the gun has overheated
    overheated: bool,
}

impl Gun {
    pub fn new() -> Gun {
        Gun::default()
    }

    /// This is called every tick and slowly cools the gun down
    pub fn cool_down(&mut self, dt: f32){
        if self.overheated{
            self.overheat_cool_down(dt);
        } else {
            self.natural_cool_down(dt);
        }
    }

    /// Whenever the gun is fired it heats up
    pub fn heat_up(&mut self){
        self.temperature = MAX_TEMP.min(self.temperature + MAX_TEMP * HEAT_PER_SHOT);
        if self.temperature == MAX_TEMP {
            self.overheated = true;
        }
    }

    /// Check if the gun has overheated or not
    pub fn is_available(&self) -> bool {
        !self.overheated
    }

    /// Reset the gun's state back to its defaults
    pub fn reset(&mut self){
        self.temperature = MIN_TEMP;
        self.overheated = false;
    }

    /// Cool down the gun naturally
    fn natural_cool_down(&mut self, dt: f32){
        self.temperature = MIN_TEMP.max(self.temperature - NATURAL_COOL_DOWN_RATE * dt);
    }

    /// The gun cools down slower if it has overheated
    fn overheat_cool_down(&mut self, dt: f32){
        self.temperature = MIN_TEMP.max(self.temperature - OVERHEAT_COOL_DOWN_RATE * dt);
        if self.temperature == MIN_TEMP {
            self.overheated = false;
        }
    }
}

const MAX_TEMP: f32 = 1.0;
const MIN_TEMP: f32 = 0.0;
const HEAT_PER_SHOT: f32 = 0.05;
const NATURAL_COOL_DOWN_RATE: f32 = 0.4;
const OVERHEAT_COOL_DOWN_RATE: f32 = 0.2;

pub struct Gun {
    pub temperature: f32,
    overheated: bool,
}

impl Default for Gun {
    fn default() -> Gun { 
        Gun {
            temperature: 0.0,
            overheated: false
        }
     }
}

impl Gun {
    pub fn new() -> Gun {
        Gun::default()
    }

    pub fn cool_down(&mut self, dt: f32){
        if self.overheated{
            self.overheat_cool_down(dt);
        }else {
            self.natural_cool_down(dt);
        }
    }

    pub fn heat_up(&mut self){
        self.temperature = MAX_TEMP.min(self.temperature + MAX_TEMP * HEAT_PER_SHOT);
        if self.temperature == MAX_TEMP {
            self.overheated = true;
        }
    }

    pub fn is_available(&self) -> bool {
        !self.overheated
    }

    pub fn reset(&mut self){
        self.temperature = MIN_TEMP;
        self.overheated = false;
    }

    fn natural_cool_down(&mut self, dt: f32){
        self.temperature = MIN_TEMP.max(self.temperature - NATURAL_COOL_DOWN_RATE * dt);
    }

    fn overheat_cool_down(&mut self, dt: f32){
        self.temperature = MIN_TEMP.max(self.temperature - OVERHEAT_COOL_DOWN_RATE * dt);
        if self.temperature == MIN_TEMP {
            self.overheated = false;
        }
    }
}

use std::time::{Instant};

const DEFAULT_NATURAL_COOL_DOWN_RATE: f32 = 0.2;
const DEFAULT_OVERHEAT_COOL_DOWN_RATE: f32 = 0.40;
const DEFAULT_MAXIMUM_CAPACITY: f32 = 1.0;
const DEFAULT_MINIMUM_CAPACITY: f32 = 0.0;
const DEFAULT_HEAT_UP: f32 = 0.05;

pub struct Gun {
    pub minimum_capacity: f32,
    pub maximum_capacity: f32,
    pub temperature: f32,
    natural_cool_down_rate: f32,
    overheated: bool,
    overheat_cool_down_rate: f32,
    last_update: Instant
}


impl Default for Gun {
    fn default() -> Gun { Gun::new() }
}

impl Gun {
    pub fn new() -> Gun {
        let maximum_capacity = DEFAULT_MAXIMUM_CAPACITY;
        let minimum_capacity = DEFAULT_MINIMUM_CAPACITY;
        Gun {
            minimum_capacity: minimum_capacity,
            maximum_capacity: maximum_capacity,
            temperature: minimum_capacity,
            natural_cool_down_rate: DEFAULT_NATURAL_COOL_DOWN_RATE * maximum_capacity,
            overheated: false,
            overheat_cool_down_rate: DEFAULT_OVERHEAT_COOL_DOWN_RATE * maximum_capacity,
            last_update: Instant::now()
        }
    }
    pub fn cool_down(&mut self){
        if self.overheated{
            self.overheat_cool_down();
        }else {
            self.natural_cool_down();
        }
        self.last_update = Instant::now();
    }
    pub fn heat_up(&mut self){
        self.temperature = self.maximum_capacity.min(self.temperature + self.maximum_capacity * DEFAULT_HEAT_UP);
        if self.temperature == self.maximum_capacity {
            self.overheat();
        }
        self.last_update = Instant::now();
    }
    pub fn is_available(&mut self) -> bool { !self.overheated }
    pub fn reset(&mut self){
        self.temperature = self.minimum_capacity;
        self.overheated = false;
    }
    fn natural_cool_down(&mut self){
        self.temperature = self.minimum_capacity.max(self.temperature - self.natural_cool_down_rate * self.time_elapsed_since_last_update());
    }
    fn overheat_cool_down(&mut self){
        self.temperature = self.minimum_capacity.max(self.temperature - self.overheat_cool_down_rate * self.time_elapsed_since_last_update());
        if self.temperature == self.minimum_capacity {
            self.overheated = false;
        }
    }
    fn overheat(&mut self){
        self.overheated = true;
    }
    fn time_elapsed_since_last_update(&mut self) -> f32 {
        (self.last_update.elapsed().subsec_nanos() as f32) / 1000000000.0
    }
}

use std::time::{Duration, Instant};

const HEAT_DECAY: f32 = 0.2;
const OVERHEAT_DECAY: f32 = 0.40;
const DEFAULT_TIMER_DURATION: u64 = 10; // in seconds
const DEFAULT_CAPACITY: f32 = 100.0;
const DEFAULT_MIN_CAPACITY: f32 = 0.0;
const DEFAULT_HEAT_UP: f32 = 0.05;

/// This is an enum of the different resource types
pub enum Resource {
    /// Every action increases it's value and slowly decays
    Heat (Heat),
    /// Starts at time X and counts down
    Timer (Timer)
}

pub struct Heat {
    min: f32,
    capacity: f32,
    current: f32,
    decay: f32,
    overheated: bool,
    overheat_decay: f32,
    last_update: Instant
}

pub struct Timer {
    start_time: Instant,
    time_left: Duration
}

impl Default for Resource {
    fn default() -> Resource { Resource::Timer ( Timer { start_time: Instant::now(), time_left: Duration::from_secs(DEFAULT_TIMER_DURATION) } ) }
}

impl Resource {
    pub fn is_available(&mut self) -> bool{
        match *self {
            Resource::Heat(ref mut heat) => heat.is_available(),
            Resource::Timer(ref Timer) => { unimplemented!(); false }
        }
    }
    pub fn spend(&mut self){
        match *self {
            Resource::Heat(ref mut heat) => heat.heat_up(),
            Resource::Timer(ref Timer) => { unimplemented!() }
        }
    }
    pub fn update(&mut self){
        match *self {
            Resource::Heat(ref mut heat) => heat.decay(),
            Resource::Timer(ref Timer) => { unimplemented!() }
        }
    }
    pub fn status(&mut self) -> f32 {
        match *self {
            Resource::Heat(ref mut heat) => heat.current,
            Resource::Timer(ref Timer) => { unimplemented!() }
        }
    }
    pub fn capacity(&mut self) -> f32 {
        match *self {
            Resource::Heat(ref mut heat) => heat.capacity,
            Resource::Timer(ref Timer) => { unimplemented!() }
        }
    }
}

impl Heat {
    pub fn new() -> Heat {
        let capacity = DEFAULT_CAPACITY;
        let min = DEFAULT_MIN_CAPACITY;
        Heat {
            min: min,
            capacity: capacity,
            current: min,
            decay: HEAT_DECAY * capacity, // per second
            overheated: false,
            overheat_decay: OVERHEAT_DECAY * capacity,
            last_update: Instant::now()
        }
    }
    /*pub fn reset_heat(&mut self){
        self.current = self.min;
        self.overheated = false;
        self.overheat_start = None;
    }*/
    pub fn decay(&mut self){
        if self.overheated{
            self.decay_overheat();
        }else {
            self.decay_regular();
        }
        self.last_update = Instant::now();
    }
    pub fn heat_up(&mut self){
        self.current = self.capacity.min(self.current + self.capacity * DEFAULT_HEAT_UP);
        if self.current == self.capacity {
            self.overheat();
        }
        self.last_update = Instant::now();
    }
    fn decay_regular(&mut self){
        self.current = self.min.max(self.current - self.decay * (self.last_update.elapsed().subsec_nanos() as f32) / 1000000000.0);
    }
    fn decay_overheat(&mut self){
        self.current = self.min.max(self.current - self.overheat_decay * (self.last_update.elapsed().subsec_nanos() as f32) / 1000000000.0);
        if self.current == self.min {
            self.overheated = false;
        }
    }
    fn overheat(&mut self){
        self.overheated = true;
    }
    pub fn is_available(&mut self) -> bool { !self.overheated }
}

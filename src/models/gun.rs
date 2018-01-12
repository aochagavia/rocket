pub struct Gun {
    pub minimum_temperature: f32,
    pub maximum_temperature: f32,
    pub temperature: f32,
    heat_up_per_shot: f32,
    natural_cool_down_rate: f32,
    overheated: bool,
    overheat_cool_down_rate: f32
}

impl Default for Gun {
    fn default() -> Gun { 
        Gun {
            minimum_temperature: 0.0,
            maximum_temperature: 1.0,
            temperature: 0.0,
            heat_up_per_shot: 0.05,
            natural_cool_down_rate: 0.2,
            overheat_cool_down_rate: 0.4,
            overheated: false
        }
     }
}

impl Gun {
    pub fn new() -> Gun { Gun::default() }
    pub fn cool_down(&mut self, dt: f32){
        if self.overheated{
            self.overheat_cool_down(dt);
        }else {
            self.natural_cool_down(dt);
        }
    }
    pub fn heat_up(&mut self){
        self.temperature = self.maximum_temperature.min(self.temperature + self.maximum_temperature * self.heat_up_per_shot);
        if self.temperature == self.maximum_temperature {
            self.overheated = true;
        }
    }
    pub fn is_available(&mut self) -> bool { !self.overheated }
    pub fn reset(&mut self){
        self.temperature = self.minimum_temperature;
        self.overheated = false;
    }
    fn natural_cool_down(&mut self, dt: f32){
        self.temperature = self.minimum_temperature.max(self.temperature - self.natural_cool_down_rate * dt);
    }
    fn overheat_cool_down(&mut self, dt: f32){
        self.temperature = self.minimum_temperature.max(self.temperature - self.overheat_cool_down_rate * dt);
        if self.temperature == self.minimum_temperature {
            self.overheated = false;
        }
    }
}

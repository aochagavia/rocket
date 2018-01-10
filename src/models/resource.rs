use std::cmp;

const HEAT_DECAY: u32 = 1;

/// This is an enum of the different resource types
#[derive(PartialEq, Clone, Copy)]
pub enum ResourceKind {
    /// Every action increases it's value and slowly decays
    Heat,
    /// Starts at time X and counts down
    Timer
}

/// This is the struct containing information about a resource
pub struct Resource {
    pub kind: ResourceKind,
    pub current: u32,
    pub min: u32,
    pub max: u32
}

impl Default for Resource {
    fn default() -> Resource { Resource::new(ResourceKind::Timer, 10) }
}

impl Resource{
    pub fn new(kind: ResourceKind, current: u32) -> Resource{
        Resource {
            kind: kind,
            current: current,
            min: 0,
            max: 10000,
        }
    }

    pub fn decay(&mut self, decay: u32){
        if self.kind = ResourceKind::Heat{ // TODO: use match
            self.current = cmp::max(self.min, self.current - HEAT_DECAY);
        }
    }

    pub fn update(&mut self, delta_current: u32){
        if self.kind = ResourceKind::Heat{
            self.current = cmp::min(self.max, self.current + delta_current);
        }
    }
}
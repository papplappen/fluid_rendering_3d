use glam::Vec3;

pub struct SimulationState {
    positions: Vec<Vec3>,
}

impl SimulationState {
    pub fn new(positions: Vec<Vec3>) -> Self {
        Self { positions }
    }
}

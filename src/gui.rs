use crate::constants::MPS_TO_KPH;
use crate::output::SimOutput;
use crate::VehicleTraits;

pub trait GUITraits {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f32;
    fn get_speed_unit(&self) -> String;
    fn get_speed_limit(&self) -> f32;
}

pub struct GUI {
    pub output: Box<dyn SimOutput>,
    pub speed_limit: f32,
}

impl GUITraits for GUI {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f32 {
        self.output.get_speed(vehicle) * MPS_TO_KPH
    }

    fn get_speed_unit(&self) -> String {
        self.output.get_speed_unit()
    }

    fn get_speed_limit(&self) -> f32 {
        self.speed_limit / MPS_TO_KPH
    }
}

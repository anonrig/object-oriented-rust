use crate::constants::MPS_TO_KPH;
use crate::output::SimOutput;
use crate::VehicleTraits;

pub trait GUITraits {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f32;
    fn get_speed_unit(&self) -> String;
    fn get_speed_limit(&self) -> f32;
}

pub struct MetricGUI {
    pub output: Box<dyn SimOutput>,
    pub speed_limit: f32,
}

impl GUITraits for MetricGUI {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f32 {
        self.output.get_speed(vehicle) * MPS_TO_KPH
    }

    fn get_speed_unit(&self) -> String {
        "kph".to_string()
    }

    fn get_speed_limit(&self) -> f32 {
        self.speed_limit / MPS_TO_KPH
    }
}

pub struct ImperialGUI {
    pub output: Box<dyn SimOutput>,
    pub speed_limit: f32,
}

impl GUITraits for ImperialGUI {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f32 {
        self.output.get_speed(vehicle) * MPS_TO_KPH
    }

    fn get_speed_unit(&self) -> String {
        "mph".to_string()
    }

    fn get_speed_limit(&self) -> f32 {
        self.speed_limit / MPS_TO_KPH
    }
}

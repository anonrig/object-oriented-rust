use crate::VehicleTraits;

pub trait SimOutput {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f32;
    fn get_speed_unit(&self) -> String;
}

pub struct ImperialOutput {}
pub struct MetricOutput {}

impl SimOutput for ImperialOutput {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f32 {
        vehicle.get_current_speed()
    }
    
    fn get_speed_unit(&self) -> String {
        "mph".to_string()
    }
}

impl SimOutput for MetricOutput {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f32 {
        vehicle.get_current_speed() * 1.6
    }

    fn get_speed_unit(&self) -> String {
        return "kmph".to_string()
    }
}

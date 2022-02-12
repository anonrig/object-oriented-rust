use crate::VehicleTraits;

pub trait SimOutput {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f32;
}

pub struct ImperialOutput {}
pub struct MetricOutput {}

impl SimOutput for ImperialOutput {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f32 {
        vehicle.get_current_speed()
    }
}

impl SimOutput for MetricOutput {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f32 {
        vehicle.get_current_speed() * 1.6
    }
}

use crate::constants::{METERS_TO_KM, METERS_TO_MILES, MPS_TO_KPH, MPS_TO_MPH};
use crate::road::{Heading, Road};
use crate::VehicleTraits;

pub trait GUITraits {
    fn create_road(
        &self,
        name: String,
        length: f32,
        location_x: f32,
        location_y: f32,
        heading: Heading,
    ) -> Road;
}

pub trait SimInputTraits {
    fn set_speed_limit(&self, vehicle: &mut dyn VehicleTraits, speed: f32);
}

pub trait SimOutputTraits {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f32;
}

pub struct MetricGUI {}
pub struct ImperialGUI {}

impl GUITraits for MetricGUI {
    fn create_road(
        &self,
        name: String,
        length: f32,
        location_x: f32,
        location_y: f32,
        heading: Heading,
    ) -> Road {
        Road::new(
            name,
            length / METERS_TO_KM,
            location_x / METERS_TO_KM,
            location_y / METERS_TO_KM,
            heading,
        )
    }
}

impl SimOutputTraits for MetricGUI {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f32 {
        vehicle.get_current_speed() * MPS_TO_KPH
    }
}

impl SimInputTraits for MetricGUI {
    fn set_speed_limit(&self, vehicle: &mut dyn VehicleTraits, speed: f32) {
        vehicle.set_desired_speed(speed);
    }
}

impl GUITraits for ImperialGUI {
    fn create_road(
        &self,
        name: String,
        length: f32,
        location_x: f32,
        location_y: f32,
        heading: Heading,
    ) -> Road {
        Road::new(
            name,
            length / METERS_TO_MILES,
            location_x / METERS_TO_MILES,
            location_y / METERS_TO_MILES,
            heading,
        )
    }
}

impl SimOutputTraits for ImperialGUI {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f32 {
        vehicle.get_current_speed() * MPS_TO_MPH
    }
}
impl SimInputTraits for ImperialGUI {
    fn set_speed_limit(&self, vehicle: &mut dyn VehicleTraits, speed: f32) {
        vehicle.set_desired_speed(speed / MPS_TO_MPH);
    }
}

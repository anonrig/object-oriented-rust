use crate::constants::{METERS_TO_KM, METERS_TO_MILES, MPS_TO_KPH, MPS_TO_MPH};
use crate::road::{Heading, Road};
use crate::road_items::{SpeedLimitSign, StopSign};
use crate::VehicleTraits;

pub trait GUITraits {
    fn create_road(
        &self,
        name: String,
        length: f64,
        location_x: f64,
        location_y: f64,
        heading: Heading,
    ) -> Road;

    fn create_stop_sign(&self, distance: f64) -> StopSign;

    fn create_speed_limit(&self, speed: f64, distance: f64) -> SpeedLimitSign;
}

pub trait SimInputTraits {
    fn set_speed_limit(&self, vehicle: &mut dyn VehicleTraits, speed: f64);
}

pub trait SimOutputTraits {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f64;
}

pub struct MetricGUI {}
pub struct ImperialGUI {}

impl GUITraits for MetricGUI {
    fn create_road(
        &self,
        name: String,
        length: f64,
        location_x: f64,
        location_y: f64,
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

    fn create_stop_sign(&self, distance: f64) -> StopSign {
        StopSign::new(distance / METERS_TO_KM)
    }

    fn create_speed_limit(&self, speed: f64, distance: f64) -> SpeedLimitSign {
        SpeedLimitSign::new(speed / METERS_TO_KM, distance / METERS_TO_KM)
    }
}

impl SimOutputTraits for MetricGUI {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f64 {
        vehicle.get_current_speed() * MPS_TO_KPH
    }
}

impl SimInputTraits for MetricGUI {
    fn set_speed_limit(&self, vehicle: &mut dyn VehicleTraits, speed: f64) {
        vehicle.set_desired_speed(speed);
    }
}

impl GUITraits for ImperialGUI {
    fn create_road(
        &self,
        name: String,
        length: f64,
        location_x: f64,
        location_y: f64,
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

    fn create_stop_sign(&self, distance: f64) -> StopSign {
        StopSign::new(distance / METERS_TO_MILES)
    }

    fn create_speed_limit(&self, speed: f64, distance: f64) -> SpeedLimitSign {
        SpeedLimitSign::new(speed / METERS_TO_MILES, distance / METERS_TO_MILES)
    }
}

impl SimOutputTraits for ImperialGUI {
    fn get_speed(&self, vehicle: &dyn VehicleTraits) -> f64 {
        vehicle.get_current_speed() * MPS_TO_MPH
    }
}
impl SimInputTraits for ImperialGUI {
    fn set_speed_limit(&self, vehicle: &mut dyn VehicleTraits, speed: f64) {
        vehicle.set_desired_speed(speed / MPS_TO_MPH);
    }
}

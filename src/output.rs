use crate::gui::{GUITraits, ImperialGUI, MetricGUI};
use crate::OutputType::Metric;
use crate::VehicleTraits;

#[derive(clap::ArgEnum, PartialEq, Debug, Clone)]
pub enum OutputType {
    Metric,
    Imperial,
}

impl OutputType {
    pub fn get_gui(&self, speed_limit: f32) -> Box<dyn GUITraits> {
        if self == &Metric {
            Box::new(MetricGUI {
                speed_limit,
                output: Box::new(MetricOutput {}),
            })
        } else {
            Box::new(ImperialGUI {
                speed_limit,
                output: Box::new(MetricOutput {}),
            })
        }
    }
}

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
        vehicle.get_current_speed()
    }
}

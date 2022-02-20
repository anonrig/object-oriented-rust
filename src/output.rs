use crate::gui::{GUITraits, GUI};
use crate::OutputType::Metric;
use crate::VehicleTraits;

#[derive(clap::ArgEnum, PartialEq, Debug, Clone)]
pub enum OutputType {
    Metric,
    Imperial,
}

impl OutputType {
    pub fn get_gui(&self, speed_limit: f32) -> Box<dyn GUITraits> {
        let output: Box<dyn SimOutput> = if self == &Metric {
            Box::new(MetricOutput {})
        } else {
            Box::new(ImperialOutput {})
        };

        Box::new(GUI {
            speed_limit,
            output,
        })
    }
}

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
        vehicle.get_current_speed()
    }
    fn get_speed_unit(&self) -> String {
        "kph".to_string()
    }
}

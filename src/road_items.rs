use crate::traits::RoadItemTraits;
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Serialize, Deserialize)]
pub struct StopSign {
    mile_marker: f64,
}

impl RoadItemTraits for StopSign {
    fn get_mile_marker(&self) -> f64 {
        self.mile_marker
    }

    fn set_mile_marker(&mut self, distance: f64) {
        self.mile_marker = distance
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl StopSign {
    pub fn new(distance: f64) -> StopSign {
        StopSign {
            mile_marker: distance,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SpeedLimitSign {
    mile_marker: f64,
    speed_limit: f64,
}

impl RoadItemTraits for SpeedLimitSign {
    fn get_mile_marker(&self) -> f64 {
        self.mile_marker
    }

    fn set_mile_marker(&mut self, distance: f64) {
        self.mile_marker = distance
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl SpeedLimitSign {
    pub fn new(posted_speed: f64, distance: f64) -> SpeedLimitSign {
        SpeedLimitSign {
            mile_marker: distance,
            speed_limit: posted_speed,
        }
    }

    pub fn get_speed_limit(&self) -> f64 {
        self.speed_limit
    }
}

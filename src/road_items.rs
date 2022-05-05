use crate::road_items::TrafficLightColor::Green;
use crate::traits::RoadItemTraits;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::any::Any;
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Clone, Copy)]
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

#[derive(Copy, Clone, Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum TrafficLightColor {
    Green = 0,
    Yellow = 1,
    Red = 2,
}

impl fmt::Display for TrafficLightColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct TrafficLight {
    red_time: u32,
    yellow_time: u32,
    green_time: u32,
    time_on: u32,
    current_color: TrafficLightColor,
    mile_marker: f64,
}

impl RoadItemTraits for TrafficLight {
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

impl TrafficLight {
    fn new(
        red_time: u32,
        yellow_time: u32,
        green_time: u32,
        color: TrafficLightColor,
        mile_marker: f64,
    ) -> TrafficLight {
        TrafficLight {
            red_time,
            yellow_time,
            green_time,
            time_on: 0,
            current_color: color,
            mile_marker,
        }
    }

    fn update(&mut self, seconds: u32) {
        self.time_on += seconds;

        match self.current_color {
            Green => {
                if self.time_on > self.green_time {
                    self.current_color = TrafficLightColor::Yellow;
                    self.time_on = 0;
                }
            }
            TrafficLightColor::Yellow => {
                if self.time_on > self.yellow_time {
                    self.current_color = TrafficLightColor::Red;
                    self.time_on = 0;
                }
            }
            TrafficLightColor::Red => {
                if self.time_on > self.red_time {
                    self.current_color = TrafficLightColor::Green;
                    self.time_on = 0;
                }
            }
        }

        println!("Changed traffic light color to {}", self.current_color);
    }
}

use crate::road::Road;
use crate::sui::{CharMatrix, PrintDriver};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    #[serde(rename = "Roads")]
    roads: Vec<Road>,
}

impl Map {
    pub fn new() -> Map {
        Map { roads: vec![] }
    }

    pub fn add_road(&mut self, road: Road) {
        self.roads.push(road)
    }

    pub fn print(&self, driver: &dyn PrintDriver, matrix: &mut CharMatrix) {
        for road in &self.roads {
            road.print(driver, matrix);
        }
    }
}

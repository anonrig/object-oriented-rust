use crate::road::Road;
use crate::sui::{CharMatrix, PrintDriver};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    #[serde(rename = "Roads")]
    roads: Vec<Road>,
}

impl Map {
    pub fn new() -> Map {
        Map { roads: vec![] }
    }

    pub fn create_from_json_file(path: String) -> Map {
        let mut file = File::open(path).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        serde_json::from_str(&buffer).unwrap()
    }

    pub fn save_to_json(&mut self, path: String) {
        match File::create(path.clone()) {
            Ok(file) => match serde_json::to_writer(&file, &self) {
                Ok(_) => {
                    println!("Successfully created file {:?}", path);
                }
                Err(error) => {
                    println!("Failed to save input to file {:?} due to {:?}", path, error);
                }
            },
            Err(error) => {
                println!("Failed to create file {:?} due to {:?}", path, error);
            }
        }
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

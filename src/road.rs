use crate::sui::{CharMatrix, PrintDriver};
use crate::traits::RoadItemTraits;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub static mut NUMBER_OF_ROADS: i32 = 0;

#[derive(Copy, Clone, Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum Heading {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

#[derive(Serialize, Deserialize)]
pub struct Road {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Length")]
    length: f64,
    #[serde(rename = "XLocation")]
    x_location: f64,
    #[serde(rename = "YLocation")]
    y_location: f64,
    #[serde(rename = "Heading")]
    heading: Heading,
    #[serde(rename = "RoadItems")]
    road_items: Vec<Box<dyn RoadItemTraits>>,
}

impl Road {
    pub fn new(
        name: String,
        length: f64,
        x_location: f64,
        y_location: f64,
        heading: Heading,
    ) -> Road {
        let road = Road {
            name,
            length,
            x_location,
            y_location,
            heading,
            road_items: vec![],
        };

        unsafe {
            NUMBER_OF_ROADS += 1;
        }

        road
    }

    pub fn get_length(&self) -> f64 {
        self.length
    }

    pub fn get_x_location(&self) -> f64 {
        self.x_location
    }

    pub fn get_y_location(&self) -> f64 {
        self.y_location
    }

    pub fn get_heading(&self) -> Heading {
        self.heading
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn print(&self, driver: &dyn PrintDriver, matrix: &mut CharMatrix) {
        driver.print_road(self, matrix);
    }

    pub fn add_road_item(&mut self, item: Box<dyn RoadItemTraits>) {
        self.road_items.push(item);
    }
}

use crate::sui::{CharMatrix, PrintDriver};

pub static mut NUMBER_OF_ROADS: i32 = 0;

#[derive(Copy, Clone, Debug)]
pub enum Heading {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub struct Road {
    name: String,
    length: f32,
    x_location: f32,
    y_location: f32,
    heading: Heading,
}

impl Road {
    pub fn new(
        name: String,
        length: f32,
        x_location: f32,
        y_location: f32,
        heading: Heading,
    ) -> Road {
        let road = Road {
            name,
            length,
            x_location,
            y_location,
            heading,
        };

        unsafe {
            NUMBER_OF_ROADS += 1;
        }

        road
    }

    pub fn get_length(&self) -> f32 {
        self.length
    }

    pub fn get_x_location(&self) -> f32 {
        self.x_location
    }

    pub fn get_y_location(&self) -> f32 {
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
}

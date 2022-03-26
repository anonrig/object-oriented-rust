use crate::constants::CHAR_MAP_SIZE;
use crate::conversions::{wclength_to_cclength, wcpoint_to_ccpoint};
use crate::road::{Heading, Road};
use crate::Car;

#[derive(Debug)]
pub struct CharMatrix {
    pub map: Vec<Vec<char>>,
}

impl CharMatrix {
    pub fn new() -> CharMatrix {
        CharMatrix {
            map: vec![vec![' '; CHAR_MAP_SIZE]; CHAR_MAP_SIZE],
        }
    }
}

pub trait PrintDriver {
    fn print_road(&self, road: &Road, matrix: &mut CharMatrix);
    fn print_car(&self, car: &Car, matrix: &mut CharMatrix);
}

pub struct ConsolePrint {}

impl PrintDriver for ConsolePrint {
    fn print_road(&self, road: &Road, matrix: &mut CharMatrix) {
        let mut distance: i32 = 0;
        let cc_x = wcpoint_to_ccpoint(road.get_x_location());
        let cc_y = wcpoint_to_ccpoint(-road.get_y_location());
        let road_length = wclength_to_cclength(road.get_length());

        match road.get_heading() {
            Heading::North => {
                let x = cc_x as usize;

                if x < CHAR_MAP_SIZE {
                    while distance < road_length {
                        let y = (cc_y - distance as f32) as usize;

                        if y < CHAR_MAP_SIZE {
                            matrix.map[y][x] = '|';
                            matrix.map[y][x + 2] = '|';
                            matrix.map[y][x + 4] = '|';
                        }

                        distance += 1;
                    }
                }
            }
            Heading::South => {}
            Heading::East => {
                let y = cc_y as usize;

                if y < CHAR_MAP_SIZE {
                    while distance < road_length {
                        let x = (cc_x + distance as f32) as usize;

                        if x < CHAR_MAP_SIZE {
                            matrix.map[y][x] = '-';
                            matrix.map[y + 2][x] = '-';
                            matrix.map[y + 4][x] = '-';
                        }

                        distance += 1;
                    }
                }
            }
            Heading::West => {}
        }
    }

    fn print_car(&self, _car: &Car, _matrix: &mut CharMatrix) {
        todo!()
    }
}

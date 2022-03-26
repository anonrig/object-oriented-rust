use crate::vehicles::{Car, VehicleTraits};

mod constants;
mod conversions;
mod environment;
mod gui;
mod map;
mod road;
mod road_items;
mod sui;
mod traits;
mod vehicles;

use crate::constants::CHAR_MAP_SIZE;
use crate::gui::{GUITraits, MetricGUI};
use crate::map::Map;
use crate::road::Heading;
use crate::sui::{CharMatrix, ConsolePrint};

fn main() {
    let console_print = ConsolePrint {};
    let sim_input = MetricGUI {};
    let mut matrix = CharMatrix::new();
    let mut map = Map::new();

    let uptown = sim_input.create_road("Uptown".to_string(), 0.180, 0.0, -0.09, Heading::North);
    let crosstown =
        sim_input.create_road("Crosstown".to_string(), 0.180, -0.09, 0.0, Heading::East);

    map.add_road(uptown);
    map.add_road(crosstown);
    map.print(&console_print, &mut matrix);

    for i in 0..CHAR_MAP_SIZE {
        println!("{:?}", String::from_iter(&matrix.map[i]));
    }
}

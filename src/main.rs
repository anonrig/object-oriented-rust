use crate::vehicles::{Car, VehicleTraits};

mod constants;
mod conversions;
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
use crate::road_items::{TrafficLight, TrafficLightColor};
use crate::sui::{CharMatrix, ConsolePrint};

fn main() {
    let console_print = ConsolePrint {};
    let mut sim_input = MetricGUI { road_items: vec![] };
    let mut matrix = CharMatrix::new();
    let mut map = Map::new();

    let mut uptown = sim_input.create_road("Uptown".to_string(), 0.180, 0.0, -0.09, Heading::North);
    let mut crosstown =
        sim_input.create_road("Crosstown".to_string(), 0.180, -0.09, 0.0, Heading::East);

    map.add_road(uptown);
    map.add_road(crosstown);

    // let sign = sim_input.create_stop_sign(0.01);
    // crosstown.add_road_item(Box::new(sign));
    //
    // let sign2 = sim_input.create_stop_sign(0.23);
    // uptown.add_road_item(Box::new(sign2));
    //
    // let sign3 = sim_input.create_stop_sign(0.32);
    // uptown.add_road_item(Box::new(sign3));
    //
    // let sign4 = sim_input.create_stop_sign(0.302);
    // uptown.add_road_item(Box::new(sign4));
    //
    // let limit = sim_input.create_speed_limit(80.0, 0.02);
    // crosstown.add_road_item(Box::new(limit));
    //
    // let limit2 = sim_input.create_speed_limit(50.0, 0.123);
    // uptown.add_road_item(Box::new(limit2));

    let traffic_light = TrafficLight::new(7, 2, 5, TrafficLightColor::Red, 0.0);
    let second_traffic_light = TrafficLight::new(7, 2, 5, TrafficLightColor::Red, 0.0);

    sim_input.road_items.push(Box::new(traffic_light));
    sim_input.road_items.push(Box::new(second_traffic_light));

    for i in 0..20 {
        sim_input
            .road_items
            .iter_mut()
            .for_each(|item| item.update(i));
    }

    map.print(&console_print, &mut matrix);
    // map.save_to_json("src/resources/output.json".to_string());

    for i in 0..CHAR_MAP_SIZE {
        println!("{:?}", String::from_iter(&matrix.map[i]));
    }
}

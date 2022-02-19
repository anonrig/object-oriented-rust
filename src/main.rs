use crate::output::OutputType;
use crate::vehicles::{Car, Truck, VehicleTraits};

mod constants;
mod environment;
mod gui;
mod output;
mod road_items;
mod traits;
mod vehicles;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CommandLineArguments {
    #[clap(arg_enum)]
    output: OutputType,

    #[clap(short, long)]
    speed_limit: u16,
}

fn main() {
    let arguments = CommandLineArguments::parse();
    let gui = arguments.output.get_gui(arguments.speed_limit as f32);

    let mut car: Car = Car {
        current_speed: 0.0,
        desired_speed: gui.get_speed_limit(),
    };

    let mut truck1: Truck = Truck {
        current_speed: 0.0,
        desired_speed: gui.get_speed_limit(),
        load_weight: 4,
    };

    let mut truck2: Truck = Truck {
        current_speed: 0.0,
        desired_speed: gui.get_speed_limit(),
        load_weight: 8,
    };

    let mut vehicles: Vec<&mut dyn VehicleTraits> = Vec::new();

    vehicles.push(&mut car);
    vehicles.push(&mut truck1);
    vehicles.push(&mut truck2);

    for _n in 1..11 {
        for vehicle in vehicles.iter_mut() {
            vehicle.update_speed(1.0);
            println!(
                "{} speed: {} {}",
                vehicle.get_type(),
                gui.get_speed(*vehicle),
                gui.get_speed_unit()
            );
        }
    }
}

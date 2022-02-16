use crate::output::{MetricOutput, SimOutput};
use crate::vehicles::{Car, Truck, VehicleTraits};

mod environment;
mod output;
mod road_items;
mod traits;
mod vehicles;

fn main() {
    let sim_output: MetricOutput = MetricOutput {};

    let mut car: Car = Car {
        current_speed: 0.0,
        desired_speed: 65.0,
    };

    let mut truck1: Truck = Truck {
        current_speed: 0.0,
        desired_speed: 55.0,
        load_weight: 4,
    };

    let mut truck2: Truck = Truck {
        current_speed: 0.0,
        desired_speed: 55.0,
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
                sim_output.get_speed(*vehicle),
                sim_output.get_speed_unit()
            );
        }
    }
}

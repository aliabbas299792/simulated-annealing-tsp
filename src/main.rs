mod brute_force;
mod common;
mod sim_annealing;

use log::{error, LevelFilter};
use std::io::Write;

use brute_force::brute_force_tsp;
use sim_annealing::simulated_annealing_tsp;
use common::{generate_map, path_cost};

fn main() {
    // setup logging
    env_logger::Builder::new()
        .format(|buff, record| {
            writeln!(
                buff,
                "({}) [{}:{}] - {}",
                record.level(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Error)
        .init();

    // generate map
    let map = generate_map(9, (1, 10)).unwrap_or_default();

    // get the correct TSP path using brute force
    match brute_force_tsp(&map) {
        Err(err) => error!("Brute Force TSP finding failed: {}", err),
        Ok((_, optimal_cost)) => {
            println!(
                "(Using Brute Force) The optimal path cost was {:}",
                optimal_cost
            )
        }
    }

    const MAX_ITERATIONS: u64 = 200000;
    const TEMPERATURE: u64 = 200;

    // and get it using simulated annealing
    match simulated_annealing_tsp(&map, TEMPERATURE, MAX_ITERATIONS) {
        Err(err) => error!("Simulated Annealing TSP finding failed: {}", err),
        Ok((_, optimal_cost)) => {
            println!(
                "(Using Simulated Annealing) The optimal path cost was {:}",
                optimal_cost
            )
        }
    }
}

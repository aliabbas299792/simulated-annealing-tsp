use itertools::Itertools;
use log::{error, LevelFilter};
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::io::Write;

fn valid_city_map(intercity_map: &Vec<Vec<u16>>) -> bool {
    intercity_map.len() != 0 && intercity_map[0].len() == intercity_map.len()
}

fn generate_map(num_cities: u16, weight_range: (u16, u16)) -> Option<Vec<Vec<u16>>> {
    let mut gen = thread_rng();
    let (low, high) = weight_range;

    if high <= low {
        error!("Weight range cannot be reversed or empty");
        return None;
    }

    let num_cities = num_cities as usize; // widen the type so that it may be used for the vector
    let mut intercity_map = vec![vec![0u16; num_cities]; num_cities];
    for i in 0..num_cities {
        for j in 0..num_cities {
            intercity_map[i][j] = if i > j {
                intercity_map[j][i] // already calculated
            } else if i == j {
                0 // distance to same city
            } else {
                gen.gen_range(low..high) // generate new city weights
            };
        }
    }

    Some(intercity_map)
}

fn path_cost(intercity_map: &Vec<Vec<u16>>, path: &Vec<u16>) -> Option<u64> {
    if !valid_city_map(&intercity_map) {
        error!("The provided map must be square");
        return None;
    }

    Some(
        path.windows(2)
            .map(|endpoints| intercity_map[endpoints[0] as usize][endpoints[1] as usize] as u64)
            .sum(),
    )
}

fn generate_random_path(intercity_map: &Vec<Vec<u16>>) -> Option<Vec<u16>> {
    if !valid_city_map(&intercity_map) {
        error!("The provided map must be square");
        return None;
    }

    let num_cities = intercity_map.len();
    let mut path: Vec<u16> = (0..(num_cities as u16)).collect();
    path.shuffle(&mut thread_rng());

    Some(path)
}

fn brute_force_tsp(intercity_map: &Vec<Vec<u16>>) -> Option<(Vec<u16>, u64)> {
    if !valid_city_map(&intercity_map) {
        error!("The provided map must be square");
        return None;
    }

    let num_cities = intercity_map.len();
    let initial_path = intercity_map.iter().enumerate().map(|(idx, _)| idx as u16);
    let cost = |p: &Vec<u16>| path_cost(&intercity_map, p);

    let optimal_path = initial_path
        .permutations(num_cities)
        .min_by(|p1, p2| cost(p1).cmp(&cost(p2)))
        .unwrap();

    let optimal_cost = path_cost(&intercity_map, &optimal_path);

    match optimal_cost {
        None => {
            error!("The optimal cost failed to be found");
            None
        }
        Some(optimal_cost) => Some((optimal_path, optimal_cost)),
    }
}

fn simulated_annealing_tsp(intercity_map: &Vec<Vec<u16>>) -> Option<(Vec<u16>, u64)> {
    if !valid_city_map(&intercity_map) {
        error!("The provided map must be square");
        return None;
    }

    let cost = |p: &Vec<u16>| path_cost(&intercity_map, p);

    let mut optimal_path = intercity_map
        .iter()
        .enumerate()
        .map(|(idx, _)| idx as u16)
        .collect::<Vec<u16>>();

    let k = 32;
    let mut optimal_cost = cost(&optimal_path);
    for _ in 0..k {
        let mut new_path = optimal_path.clone();
        new_path.shuffle(&mut thread_rng());
        let new_cost = cost(&new_path);
        if new_cost < optimal_cost {
            optimal_path = new_path;
            optimal_cost = new_cost;
        }
    }

    match optimal_cost {
        None => {
            error!("The optimal cost failed to be found");
            None
        }
        Some(optimal_cost) => Some((optimal_path, optimal_cost)),
    }
}

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
    let map = generate_map(10, (1, 1)).unwrap_or_default();

    // get the correct TSP path using brute force
    match brute_force_tsp(&map) {
        None => error!("Brute Force TSP finding failed"),
        Some((optimal_path, optimal_cost)) => {
            println!(
                "(Using Brute Force) The optimal path for the map {:#?} was {:#?}, and cost {:}",
                map, optimal_path, optimal_cost
            )
        }
    }

    // and get it using simulated annealing
    match simulated_annealing_tsp(&map) {
        None => error!("Simulated Annealing TSP finding failed"),
        Some((optimal_path, optimal_cost)) => {
            println!(
                "(Using Simulated Annealing) The optimal path for the map{:#?} was {:#?}, and cost {:}",
                map, optimal_path, optimal_cost
            )
        }
    }
}

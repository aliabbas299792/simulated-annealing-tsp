use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng, Rng};

fn valid_city_map(intercity_map: &Vec<Vec<u16>>) -> bool {
    intercity_map.len() != 0 && intercity_map[0].len() == intercity_map.len()
}

fn generate_map(num_cities: u16, weight_range: (u16, u16)) -> Option<Vec<Vec<u16>>> {
    let mut gen = thread_rng();
    let (low, high) = weight_range;

    if high < low {
        return None; // weight range cannot be inverted
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
        return None; // must be a square map
    }

    let num_cities = intercity_map.len();
    let mut path: Vec<u16> = (0..(num_cities as u16)).collect();
    path.shuffle(&mut thread_rng());

    Some(path)
}

fn brute_force_tsp(intercity_map: &Vec<Vec<u16>>) -> Option<(Vec<u16>, u64)> {
    if !valid_city_map(&intercity_map) {
        return None;
    }

    let num_cities = intercity_map.len();
    let initial_path = intercity_map.iter().enumerate().map(|(idx, _)| idx as u16);
    let cost = |p: &Vec<u16>| path_cost(&intercity_map, p);

    let optimal_path = initial_path.permutations(num_cities)
        .max_by(|p1, p2| cost(p1).cmp(&cost(p2)))
        .unwrap();

    let optimal_cost = path_cost(&intercity_map, &optimal_path);

    match optimal_cost {
        None => None,
        Some(optimal_cost) => Some((optimal_path, optimal_cost)),
    }
}

fn main() {
    let map = generate_map(10, (10, 15)).unwrap_or_default();

    match brute_force_tsp(&map) {
        None => println!("Brute force TSP finding failed"),
        Some((optimal_path, optimal_cost)) => {
            println!(
                "The optimal path for the map {:#?} was {:#?}, and cost {:}",
                map, optimal_path, optimal_cost
            )
        }
    }
}

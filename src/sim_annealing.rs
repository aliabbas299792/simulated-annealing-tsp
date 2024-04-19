use log::error;
use rand::{seq::SliceRandom, thread_rng, Rng};

use super::common::{generate_default_path, path_cost, valid_city_map, TSPError};

struct SimulatedAnnealing {
    intercity_map: Vec<Vec<u16>>,
    curr_path: Vec<u16>,
    curr_path_cost: i64, // the energy of the current solution
    init_temperature: u64,
    curr_iteration: u64,
    max_iterations: u64,
}

impl SimulatedAnnealing {
    fn accept_candidate(&self, new_path: &Vec<u16>) -> bool {
        let gen = &mut thread_rng();
        let new_cost = path_cost(&self.intercity_map, &new_path);
        let temp = self.init_temperature as f64 / (1. + self.curr_iteration as f64);
        let diff = (new_cost as i64 - self.curr_path_cost) as f64;
        let metropolis_criterion = (-diff / temp).exp();

        diff < 0. || gen.gen_range(0.0..1.0) < metropolis_criterion
    }

    fn calculate_optimal(&mut self) -> (Vec<u16>, u32) {
        while self.next().is_some() {}
        (
            self.curr_path.clone(),
            path_cost(&self.intercity_map, &self.curr_path),
        )
    }

    fn new(
        intercity_map: Vec<Vec<u16>>,
        temperature: u64,
        max_iterations: u64,
    ) -> SimulatedAnnealing {
        let default_path = generate_default_path(&intercity_map);
        let curr_cost = path_cost(&intercity_map, &default_path);

        SimulatedAnnealing {
            intercity_map,
            curr_path: default_path,
            curr_path_cost: curr_cost as i64,
            init_temperature: temperature,
            curr_iteration: 0,
            max_iterations,
        }
    }
}

impl Iterator for SimulatedAnnealing {
    type Item = Vec<u16>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.max_iterations == self.curr_iteration {
            return None;
        }

        let mut new_path = self.curr_path.clone();
        new_path.shuffle(&mut thread_rng());

        if self.accept_candidate(&new_path) {
            self.curr_path = new_path.clone();
            self.curr_path_cost = path_cost(&self.intercity_map, &new_path) as i64;
            println!("cost {}, iter {}", self.curr_path_cost, self.curr_iteration)
        }

        self.curr_iteration += 1;

        Some(new_path)
    }
}

pub fn simulated_annealing_tsp(
    intercity_map: &Vec<Vec<u16>>,
    temperature: u64,
    max_iterations: u64,
) -> Result<(Vec<u16>, u32), TSPError> {
    if !valid_city_map(&intercity_map) {
        error!("The provided map must be square");
        return Err(TSPError::InvalidMapShape);
    }

    let mut state = SimulatedAnnealing::new(intercity_map.clone(), temperature, max_iterations);
    Ok(state.calculate_optimal())
}

mod tests {
    use crate::common::generate_map;
    use crate::simulated_annealing_tsp;

    const TEST_TEMPERATURE: u64 = 200;
    const TEST_MAX_ITERATIONS: u64 = 3000;

    #[test]
    fn test_simulated_annealing() {
        let num_checks = 30;

        for _ in 0..num_checks {
            let map = generate_map(5, (0, 300)).ok().unwrap();
            let res = simulated_annealing_tsp(&map, TEST_TEMPERATURE, TEST_MAX_ITERATIONS);
            assert!(res.is_ok());
            assert!(res.ok().is_some());
        }
    }
}

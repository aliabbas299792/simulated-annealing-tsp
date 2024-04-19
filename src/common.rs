use itertools::Itertools;
use log::error;
use rand::{seq::SliceRandom, thread_rng, Rng};

pub enum TSPError {
    InvalidMapShape,
    InvalidWeightRange, // weight range cannot be reversed or empty
}

impl std::fmt::Display for TSPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TSPError::InvalidMapShape => write!(f, "invalid map shape"),
            TSPError::InvalidWeightRange => write!(f, "invalid weight range"),
        }
    }
}

fn generate_random_path(intercity_map: &Vec<Vec<u16>>) -> Vec<u16> {
    let num_cities = intercity_map.len();
    let mut path: Vec<u16> = (0..(num_cities as u16)).collect();
    path.shuffle(&mut thread_rng());
    path
}

pub fn valid_city_map(intercity_map: &Vec<Vec<u16>>) -> bool {
    intercity_map.len() != 0 && intercity_map[0].len() == intercity_map.len()
}

pub fn generate_map(num_cities: u16, weight_range: (u16, u16)) -> Result<Vec<Vec<u16>>, TSPError> {
    let mut gen = thread_rng();
    let (low, high) = weight_range;

    if high <= low {
        error!("Weight range cannot be reversed or empty");
        return Err(TSPError::InvalidWeightRange);
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

    Ok(intercity_map)
}

pub fn path_cost(intercity_map: &Vec<Vec<u16>>, path: &Vec<u16>) -> u32 {
    path.windows(2)
        .map(|endpoints| intercity_map[endpoints[0] as usize][endpoints[1] as usize] as u32)
        .sum()
}

pub fn generate_default_path(intercity_map: &Vec<Vec<u16>>) -> Vec<u16> {
    intercity_map
        .iter()
        .enumerate()
        .map(|(idx, _)| idx as u16)
        .collect_vec()
}

mod tests {
    use crate::common::generate_random_path;
    use crate::{generate_map, path_cost};
    use itertools::zip_eq;
    use itertools::Itertools;

    #[test]
    fn test_path_cost() {
        let map: Vec<Vec<u16>> = vec![
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
        ];
        let path: Vec<u16> = vec![0, 1, 2, 3];

        // all paths costs 2 so 3 movements needed, so 2*3 is the cost
        let cost = path_cost(&map, &path);
        assert_eq!(cost, 2 * 3);
    }

    #[test]
    fn test_map_gen() {
        let map = generate_map(10, (25, 40));
        assert!(map.is_ok());
        let map = map.ok().unwrap();

        for i in 0..map.len() {
            for j in 0..map.len() {
                assert_eq!(map[i][j], map[j][i]); // must be symmetric
            }
        }
    }

    #[test]
    fn test_random_path_gen() {
        let map = generate_map(10, (60, 90)).ok().unwrap();
        let path = generate_random_path(&map);
        let path = path;

        let dedupd = path.iter().unique().collect::<Vec<&u16>>();
        for (&e1, &e2) in zip_eq(&dedupd, &path) {
            assert_eq!(*e1, e2);
        }

        assert_eq!(dedupd.len(), path.len());
    }
}

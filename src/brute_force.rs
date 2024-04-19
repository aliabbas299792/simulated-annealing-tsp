use itertools::Itertools;
use log::error;

use super::common::{valid_city_map, path_cost, TSPError};

pub fn brute_force_tsp(intercity_map: &Vec<Vec<u16>>) -> Result<(Vec<u16>, u32), TSPError> {
    if !valid_city_map(&intercity_map) {
        error!("The provided map must be square");
        return Err(TSPError::InvalidMapShape);
    }

    let num_cities = intercity_map.len();
    let initial_path = intercity_map.iter().enumerate().map(|(idx, _)| idx as u16);
    let cost = |p: &Vec<u16>| path_cost(&intercity_map, p);

    let optimal_path = initial_path
        .permutations(num_cities)
        .min_by(|p1, p2| cost(p1).cmp(&cost(p2)))
        .unwrap();

    let optimal_cost = path_cost(&intercity_map, &optimal_path);

    Ok((optimal_path, optimal_cost))
}


mod tests {
    use crate::{brute_force_tsp, path_cost};

    #[test]
    fn test_brute_force_tsp() {
        let map: Vec<Vec<u16>> = vec![
            vec![2, 2, 2, 1],
            vec![1, 2, 2, 2],
            vec![2, 1, 2, 2],
            vec![2, 2, 1, 2],
        ];
        let path: Vec<u16> = vec![0, 3, 2, 1];

        let res = brute_force_tsp(&map);
        assert!(res.is_ok());

        let (optimal_path, optimal_cost) = res.ok().unwrap();

        assert_eq!(optimal_path, path);
        assert_eq!(optimal_cost, path_cost(&map, &path));
    }
}

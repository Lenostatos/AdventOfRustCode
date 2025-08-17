use std::collections::HashMap;

use util::input::lines_of;

fn main() {
    // Build an index of town-distance relationships
    let lines = lines_of("input.txt");

    // let mut towns_and_distances = HashMap::<TownName, Vec<TownAndDistance>>::new();
    let mut towns_and_distances = TownDistanceIndex::new();

    for line in lines {
        let towns_and_distance = line.split_once(" = ").unwrap();
        let distance = towns_and_distance.1.parse::<usize>().unwrap();
        let towns = towns_and_distance.0.split_once(" to ").unwrap();

        towns_and_distances.insert(towns.0, distance, towns.1);
    }

    println!("{:#?}", towns_and_distances);

    // Clone the index for later
    let towns_and_distances_clone = towns_and_distances.clone();
    let towns_and_distances_clone_2 = towns_and_distances.clone();

    // Find shortest path (nearest neighbour approach)

    // ⚠️ Still buggy! The path length found is longer than the longest possible path found with the brute force approach. ⚠️

    // 1) get a random pair of neigbouring towns
    // 2) take one of the towns as the left one and the other as the right one
    // 3) extend the path stepwise to either the left or right, depending on where the smaller distance to the next town can be found.

    let mut left = Some(towns_and_distances.index.keys().next().unwrap().clone());
    let mut right = towns_and_distances
        .pop_nearest_neighbour(left.as_ref().unwrap())
        .and_then(|td| Some(td.0));

    let mut path = vec![left.clone(), right.clone()];

    loop {
        if left.is_none() && right.is_none() {
            break;
        } else if left.is_none() {
            path.push(
                towns_and_distances
                    .pop_nearest_neighbour(right.as_ref().unwrap())
                    .and_then(|td| Some(td.0)),
            );
        } else if right.is_none() {
            path.insert(
                0,
                towns_and_distances
                    .pop_nearest_neighbour(left.as_ref().unwrap())
                    .and_then(|td| Some(td.0)),
            );
        } else {
            let left_distance = towns_and_distances
                .get_nearest_neighbour(left.as_ref().unwrap())
                .and_then(|td| Some(td.1));

            let right_distance = towns_and_distances
                .get_nearest_neighbour(right.as_ref().unwrap())
                .and_then(|td| Some(td.1));

            if left_distance.is_none() && right_distance.is_none() {
                break;
            } else if left_distance.is_none() {
                path.push(
                    towns_and_distances
                        .pop_nearest_neighbour(right.as_ref().unwrap())
                        .and_then(|td| Some(td.0)),
                );
            } else if right_distance.is_none() {
                path.insert(
                    0,
                    towns_and_distances
                        .pop_nearest_neighbour(left.as_ref().unwrap())
                        .and_then(|td| Some(td.0)),
                );
            } else if left_distance.unwrap() <= right_distance.unwrap() {
                path.insert(
                    0,
                    towns_and_distances
                        .pop_nearest_neighbour(left.as_ref().unwrap())
                        .and_then(|td| Some(td.0)),
                );
            } else if right_distance.unwrap() < left_distance.unwrap() {
                path.push(
                    towns_and_distances
                        .pop_nearest_neighbour(right.as_ref().unwrap())
                        .and_then(|td| Some(td.0)),
                );
            } else {
                panic!("This should not happen.");
            }
        }

        left = path.first().unwrap().clone();
        right = path.last().unwrap().clone()
    }

    // Calculate the total route length
    let mut route_len: usize = 0;

    for two_towns in path.as_slice().windows(2) {
        // 1) find town-distances to first town
        // 2) find second town among those
        // 3) get distance
        let distance = towns_and_distances_clone
            .get_distance(
                &two_towns[0].clone().unwrap(),
                &two_towns[1].clone().unwrap(),
            )
            .unwrap();

        route_len += distance;
    }

    println!("{route_len}");

    // Find shortest path (brute force approach)

    // Try all possible paths.

    let towns = towns_and_distances_clone_2
        .index
        .keys()
        .cloned()
        .collect::<Vec<TownName>>();

    let mut town_combinations = SetPermutations::new(towns.len(), towns.len());

    let mut shortest_route_len = usize::MIN;

    loop {
        let town_combination = town_combinations.next();

        if town_combination.is_none() {
            break;
        }

        let mut route_len = 0;

        for two_towns in town_combination.unwrap().as_slice().windows(2) {
            // 1) find town-distances to first town
            // 2) find second town among those
            // 3) get distance
            let distance = towns_and_distances_clone_2
                .get_distance(&towns[two_towns[0]], &towns[two_towns[1]])
                .unwrap();

            route_len += distance;
        }

        shortest_route_len = usize::max(shortest_route_len, route_len);
    }

    println!("{shortest_route_len}");

    // Find shortest path (Held-Karp-Algorithm)

    // Build shortest path piece-wise from shorter to longer sections, re-using shortest paths for specific combinations of selected towns and last town in the respective shortest path.

    /*
     * # 3 Towns
     *
     * 0AB = {AB}{B}
     * 0BA = {AB}{A}
     * 0CA = {AC}{A}
     * 0AC = {AC}{C}
     * 0BC = ...
     * 0CB
     * ...
     * 0DC
     *
     * # 4 Towns
     *
     * min(0BCA / 0CBA) = {ABC}{A}
     * min(0ACB / 0CAB) = {ABC}{B}
     * min(0ABC / 0BAC) = {ABC}{C}
     *
     * min(0BDA / 0DBA) = {ABD}{A}
     * min(0ADB / 0DAB) = {ABD}{B}
     * min(0ABD / 0BAD) = {ABD}{D}
     *
     * min(0CDA / 0DCA) = {ACD}{A}
     * min(0ADC / 0DAC) = {ACD}{C}
     * min(0ACD / 0CAD) = {ACD}{D}
     *
     * min(0CDB / 0DCB) = {BCD}{B}
     * min(0BDC / 0DBC) = {BCD}{C}
     * min(0BCD / 0CBD) = {BCD}{D}
     *
     * # 5 Towns
     *
     * {ABCD}{A} = min({BCD}{*})
     * {ABCD}{B} = min({ACD}{*})
     * {ABCD}{C} = min({ABD}{*})
     * {ABCD}{D} = min({ABC}{*})
     */
}

struct SetPermutations {
    set_size: usize,
    permutation_len: usize,
    permutation_nr: usize,
    permutation_max_nr: usize,
    permutation_nr_skip: usize,
    set_memo: Vec<usize>,
}

impl SetPermutations {
    fn new(set_size: usize, permutation_len: usize) -> Self {
        if set_size < permutation_len {
            panic!("Lengths of set permutations should not exceed set size.");
        }

        let set_memo = (0..set_size).collect::<Vec<usize>>();

        SetPermutations {
            set_size,
            permutation_len,
            permutation_nr: 0,
            permutation_max_nr: factorial(set_size),
            permutation_nr_skip: factorial(set_size - permutation_len),
            set_memo,
        }
    }

    fn next(&mut self) -> Option<Vec<usize>> {
        if self.permutation_nr >= self.permutation_max_nr || self.permutation_len == 0 {
            return None;
        }

        let mut permutation = Vec::with_capacity(self.set_size);

        let mut set = self.set_memo.clone();
        let permutation_encoding = as_factoriadic(self.permutation_nr, self.set_size);

        for p in &permutation_encoding[0..self.permutation_len] {
            permutation.push(set.remove(*p));
        }

        self.permutation_nr += self.permutation_nr_skip;

        Some(permutation)
    }
}

#[test]
fn set_permutations() {
    let mut perms = SetPermutations::new(0, 0);

    assert_eq!(perms.next(), None);
    assert_eq!(perms.next(), None);

    let mut perms = SetPermutations::new(1, 1);

    assert_eq!(perms.next(), Some(vec![0]));
    assert_eq!(perms.next(), None);
    assert_eq!(perms.next(), None);

    let mut perms = SetPermutations::new(2, 2);

    assert_eq!(perms.next(), Some(vec![0, 1]));
    assert_eq!(perms.next(), Some(vec![1, 0]));
    assert_eq!(perms.next(), None);
    assert_eq!(perms.next(), None);

    let mut perms = SetPermutations::new(3, 3);

    assert_eq!(perms.next(), Some(vec![0, 1, 2]));
    assert_eq!(perms.next(), Some(vec![0, 2, 1]));
    assert_eq!(perms.next(), Some(vec![1, 0, 2]));
    assert_eq!(perms.next(), Some(vec![1, 2, 0]));
    assert_eq!(perms.next(), Some(vec![2, 0, 1]));
    assert_eq!(perms.next(), Some(vec![2, 1, 0]));
    assert_eq!(perms.next(), None);
    assert_eq!(perms.next(), None);

    let mut perms = SetPermutations::new(4, 4);

    assert_eq!(perms.next(), Some(vec![0, 1, 2, 3]));
    assert_eq!(perms.next(), Some(vec![0, 1, 3, 2]));
    assert_eq!(perms.next(), Some(vec![0, 2, 1, 3]));
    assert_eq!(perms.next(), Some(vec![0, 2, 3, 1]));
    assert_eq!(perms.next(), Some(vec![0, 3, 1, 2]));
    assert_eq!(perms.next(), Some(vec![0, 3, 2, 1]));

    assert_eq!(perms.next(), Some(vec![1, 0, 2, 3]));
    assert_eq!(perms.next(), Some(vec![1, 0, 3, 2]));
    assert_eq!(perms.next(), Some(vec![1, 2, 0, 3]));
    assert_eq!(perms.next(), Some(vec![1, 2, 3, 0]));
    assert_eq!(perms.next(), Some(vec![1, 3, 0, 2]));
    assert_eq!(perms.next(), Some(vec![1, 3, 2, 0]));

    assert_eq!(perms.next(), Some(vec![2, 0, 1, 3]));
    assert_eq!(perms.next(), Some(vec![2, 0, 3, 1]));
    assert_eq!(perms.next(), Some(vec![2, 1, 0, 3]));
    assert_eq!(perms.next(), Some(vec![2, 1, 3, 0]));
    assert_eq!(perms.next(), Some(vec![2, 3, 0, 1]));
    assert_eq!(perms.next(), Some(vec![2, 3, 1, 0]));

    assert_eq!(perms.next(), Some(vec![3, 0, 1, 2]));
    assert_eq!(perms.next(), Some(vec![3, 0, 2, 1]));
    assert_eq!(perms.next(), Some(vec![3, 1, 0, 2]));
    assert_eq!(perms.next(), Some(vec![3, 1, 2, 0]));
    assert_eq!(perms.next(), Some(vec![3, 2, 0, 1]));
    assert_eq!(perms.next(), Some(vec![3, 2, 1, 0]));

    assert_eq!(perms.next(), None);
    assert_eq!(perms.next(), None);

    let mut perms = SetPermutations::new(1, 0);

    assert_eq!(perms.next(), None);

    let mut perms = SetPermutations::new(2, 1);

    assert_eq!(perms.next(), Some(vec![0]));
    assert_eq!(perms.next(), Some(vec![1]));
    assert_eq!(perms.next(), None);
    assert_eq!(perms.next(), None);

    let mut perms = SetPermutations::new(3, 1);

    assert_eq!(perms.next(), Some(vec![0]));
    assert_eq!(perms.next(), Some(vec![1]));
    assert_eq!(perms.next(), Some(vec![2]));
    assert_eq!(perms.next(), None);
    assert_eq!(perms.next(), None);

    let mut perms = SetPermutations::new(3, 2);

    assert_eq!(perms.next(), Some(vec![0, 1]));
    assert_eq!(perms.next(), Some(vec![0, 2]));
    assert_eq!(perms.next(), Some(vec![1, 0]));
    assert_eq!(perms.next(), Some(vec![1, 2]));
    assert_eq!(perms.next(), Some(vec![2, 0]));
    assert_eq!(perms.next(), Some(vec![2, 1]));
    assert_eq!(perms.next(), None);
    assert_eq!(perms.next(), None);

    let mut perms = SetPermutations::new(4, 1);

    assert_eq!(perms.next(), Some(vec![0]));
    assert_eq!(perms.next(), Some(vec![1]));
    assert_eq!(perms.next(), Some(vec![2]));
    assert_eq!(perms.next(), Some(vec![3]));
    assert_eq!(perms.next(), None);
    assert_eq!(perms.next(), None);

    let mut perms = SetPermutations::new(4, 2);

    assert_eq!(perms.next(), Some(vec![0, 1]));
    assert_eq!(perms.next(), Some(vec![0, 2]));
    assert_eq!(perms.next(), Some(vec![0, 3]));
    assert_eq!(perms.next(), Some(vec![1, 0]));
    assert_eq!(perms.next(), Some(vec![1, 2]));
    assert_eq!(perms.next(), Some(vec![1, 3]));
    assert_eq!(perms.next(), Some(vec![2, 0]));
    assert_eq!(perms.next(), Some(vec![2, 1]));
    assert_eq!(perms.next(), Some(vec![2, 3]));
    assert_eq!(perms.next(), Some(vec![3, 0]));
    assert_eq!(perms.next(), Some(vec![3, 1]));
    assert_eq!(perms.next(), Some(vec![3, 2]));
    assert_eq!(perms.next(), None);
    assert_eq!(perms.next(), None);

    let mut perms = SetPermutations::new(4, 3);

    assert_eq!(perms.next(), Some(vec![0, 1, 2]));
    assert_eq!(perms.next(), Some(vec![0, 1, 3]));
    assert_eq!(perms.next(), Some(vec![0, 2, 1]));
    assert_eq!(perms.next(), Some(vec![0, 2, 3]));
    assert_eq!(perms.next(), Some(vec![0, 3, 1]));
    assert_eq!(perms.next(), Some(vec![0, 3, 2]));

    assert_eq!(perms.next(), Some(vec![1, 0, 2]));
    assert_eq!(perms.next(), Some(vec![1, 0, 3]));
    assert_eq!(perms.next(), Some(vec![1, 2, 0]));
    assert_eq!(perms.next(), Some(vec![1, 2, 3]));
    assert_eq!(perms.next(), Some(vec![1, 3, 0]));
    assert_eq!(perms.next(), Some(vec![1, 3, 2]));

    assert_eq!(perms.next(), Some(vec![2, 0, 1]));
    assert_eq!(perms.next(), Some(vec![2, 0, 3]));
    assert_eq!(perms.next(), Some(vec![2, 1, 0]));
    assert_eq!(perms.next(), Some(vec![2, 1, 3]));
    assert_eq!(perms.next(), Some(vec![2, 3, 0]));
    assert_eq!(perms.next(), Some(vec![2, 3, 1]));

    assert_eq!(perms.next(), Some(vec![3, 0, 1]));
    assert_eq!(perms.next(), Some(vec![3, 0, 2]));
    assert_eq!(perms.next(), Some(vec![3, 1, 0]));
    assert_eq!(perms.next(), Some(vec![3, 1, 2]));
    assert_eq!(perms.next(), Some(vec![3, 2, 0]));
    assert_eq!(perms.next(), Some(vec![3, 2, 1]));

    assert_eq!(perms.next(), None);
    assert_eq!(perms.next(), None);

    let mut perms = SetPermutations::new(10, 1);

    assert_eq!(perms.next(), Some(vec![0]));
    assert_eq!(perms.next(), Some(vec![1]));
    assert_eq!(perms.next(), Some(vec![2]));
    assert_eq!(perms.next(), Some(vec![3]));
    assert_eq!(perms.next(), Some(vec![4]));
    assert_eq!(perms.next(), Some(vec![5]));
    assert_eq!(perms.next(), Some(vec![6]));
    assert_eq!(perms.next(), Some(vec![7]));
    assert_eq!(perms.next(), Some(vec![8]));
    assert_eq!(perms.next(), Some(vec![9]));
    assert_eq!(perms.next(), None);
    assert_eq!(perms.next(), None);
}

#[test]
fn enumerate_rev_iter() {
    let a = ['a', 'b', 'c'];

    let mut iter = a.iter().rev().enumerate();

    assert_eq!(iter.next(), Some((0, &'c')));
    assert_eq!(iter.next(), Some((1, &'b')));
    assert_eq!(iter.next(), Some((2, &'a')));
    assert_eq!(iter.next(), None);
}

/**
 Calculates the factoriadic representation of a number.

 Every decimal integer can be represented by the sum of the multiples of different factorials (0!, 1!, 2!, 3!, 4!, etc.). E.g.:
 * The decimal 1 can be calculated as 1 * 1! + 0 * 0!, which is the factoriadic 1:0.
 * The decimal 4 can be calculated as 2 * 2! + 0 * 1! + 0 * 0!, which is the factoriadic 2:0:0.

 Source: https://en.wikipedia.org/wiki/Factorial_number_system
*/
fn as_factoriadic(n: usize, len: usize) -> Vec<usize> {
    let mut factoriadic = Vec::with_capacity(len);

    let mut radix: usize = 0;
    let mut quotient = n;

    if quotient == 0 {
        return vec![0; len];
    }

    while quotient > 0 {
        radix += 1;

        factoriadic.push(quotient % radix);

        quotient = quotient / radix;
    }

    if len > factoriadic.len() {
        for _ in factoriadic.len()..len {
            factoriadic.push(0);
        }
    }

    assert_eq!(factoriadic.len(), len);

    factoriadic.reverse();

    factoriadic
}

#[test]
fn factoriadic_calculation() {
    assert_eq!(as_factoriadic(0, 1), vec![0]); // 0 * 0!
    assert_eq!(as_factoriadic(1, 2), vec![1, 0]); // 1 * 1! + 0 * 0!
    assert_eq!(as_factoriadic(2, 3), vec![1, 0, 0]); // 1 * 2! + 0 * 1! + 0 * 0!
    assert_eq!(as_factoriadic(3, 3), vec![1, 1, 0]); // 1 * 2! + 1 * 1! + 0 * 0!
    assert_eq!(as_factoriadic(4, 3), vec![2, 0, 0]); // ...
    assert_eq!(as_factoriadic(5, 3), vec![2, 1, 0]);
    assert_eq!(as_factoriadic(6, 4), vec![1, 0, 0, 0]);
    assert_eq!(as_factoriadic(7, 4), vec![1, 0, 1, 0]);
    assert_eq!(as_factoriadic(8, 4), vec![1, 1, 0, 0]);
    assert_eq!(as_factoriadic(9, 4), vec![1, 1, 1, 0]);
    assert_eq!(as_factoriadic(10, 4), vec![1, 2, 0, 0]);
    assert_eq!(as_factoriadic(11, 4), vec![1, 2, 1, 0]);
    assert_eq!(as_factoriadic(12, 4), vec![2, 0, 0, 0]);
    assert_eq!(as_factoriadic(13, 4), vec![2, 0, 1, 0]);
    assert_eq!(as_factoriadic(24, 5), vec![1, 0, 0, 0, 0]);
    assert_eq!(as_factoriadic(120, 6), vec![1, 0, 0, 0, 0, 0]);
    assert_eq!(as_factoriadic(720, 7), vec![1, 0, 0, 0, 0, 0, 0]);

    assert_eq!(as_factoriadic(0, 2), vec![0, 0]);
    assert_eq!(as_factoriadic(0, 3), vec![0, 0, 0]);

    assert_eq!(as_factoriadic(5, 4), vec![0, 2, 1, 0]);
    assert_eq!(as_factoriadic(5, 5), vec![0, 0, 2, 1, 0]);
}

fn factorial(n: usize) -> usize {
    if n <= 10 {
        return vec![1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800][n];
    }

    (1..=n).product()
}

use String as TownName;
use usize as TownDistance;

#[derive(Clone, PartialEq, Debug)]
struct TownAndDistance(TownName, TownDistance);

#[derive(Debug, Clone)]
struct TownDistanceIndex {
    index: HashMap<TownName, HashMap<TownName, TownDistance>>,
}

impl TownDistanceIndex {
    fn new() -> Self {
        Self {
            index: HashMap::<TownName, HashMap<TownName, TownDistance>>::new(),
        }
    }

    fn insert(&mut self, town_1: &str, distance: TownDistance, town_2: &str) {
        self.index
            .entry(town_1.to_string())
            .and_modify(|town_1_distances| {
                town_1_distances.insert(town_2.to_string(), distance);
            })
            .or_insert(HashMap::from([(town_2.to_string(), distance)]));

        self.index
            .entry(town_2.to_string())
            .and_modify(|town_2_distances| {
                town_2_distances.insert(town_1.to_string(), distance);
            })
            .or_insert(HashMap::from([(town_1.to_string(), distance)]));
    }

    fn get_distance(&self, town_1: &str, town_2: &str) -> Option<TownDistance> {
        self.index
            .get(town_1)
            .and_then(|town_1_distances| town_1_distances.get(town_2))
            .copied()
    }

    fn remove_distance(&mut self, town_1: &str, town_2: &str) {
        self.index
            .get_mut(town_1)
            .and_then(|town_1_distances| town_1_distances.remove(town_2));

        self.index
            .get_mut(town_2)
            .and_then(|town_2_distances| town_2_distances.remove(town_1));
    }

    fn get_nearest_neighbour(&self, town: &str) -> Option<TownAndDistance> {
        let town_distances = self.index.get(town);

        match town_distances {
            Some(distances) => distances
                .iter()
                .min_by(|a, b| a.1.cmp(b.1))
                .and_then(|nt| Some(TownAndDistance(nt.0.clone(), *nt.1))),
            None => None,
        }
    }

    fn pop_nearest_neighbour(&mut self, town: &str) -> Option<TownAndDistance> {
        self.get_nearest_neighbour(town)
            .inspect(|nearest_neighbour| self.remove_distance(town, &nearest_neighbour.0))
    }
}

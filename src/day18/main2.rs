use std::collections::{HashSet, HashMap};
use crate::day18::main2::Field::{KEY, PATH, WALL, DOOR};

pub fn main(input: &str) -> u64 {
    //Parse input into hashset
    let mut points: HashMap<(u64, u64), Field> = HashMap::new();
    let mut key_locations: HashMap<char, (u64, u64)> = HashMap::new();
    let mut starts = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            points.insert((x as u64, y as u64), match ch {
                '#' => WALL,
                '.' => PATH,
                'a'..='z' => {
                    key_locations.insert(ch, (x as u64, y as u64));
                    KEY(ch)
                },
                'A'..='Z' => DOOR(ch),
                '@' => {
                    starts.push((x as u64, y as u64));
                    PATH
                }
                _ => panic!("Found unknown char")
            });
        }
    }
    assert_eq!(starts.len(), 4);

    let mut cache: HashMap<(Vec<(u64, u64)>, Vec<char>), u64> = HashMap::new();
    return getDistance(&mut cache, &points, &key_locations, &mut starts, &mut Vec::new());
}

pub fn getDistance(cache: &mut HashMap<(Vec<(u64, u64)>, Vec<char>), u64>, points: &HashMap<(u64, u64), Field>, key_locations: &HashMap<char, (u64, u64)>, currents: &mut Vec<(u64, u64)>, collected_keys: &mut Vec<char>) -> u64 {
    //Check cache
    let mut cache_key = (currents.clone(), collected_keys.clone());
    cache_key.1.sort();
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }

    let mut min_dist = std::u64::MAX;

    //For each robot, consider moves
    for i in 0..4 {
        let current = currents[i];

        //Find reachable keys, breadth first search
        let mut reachable_keys: HashSet<(char, u64)> = HashSet::new();
        let mut reached_points = HashSet::new();
        let mut previous_points = HashSet::new();
        previous_points.insert(current);

        //Do breadth first search
        let mut distance = 0;
        while !previous_points.is_empty() {
            distance += 1;
            let mut next_points = HashSet::new();
            for next in &previous_points {
                reached_points.insert(next.clone());
                for neighbour in neighbours(points, next) {
                    if reached_points.contains(&neighbour) { continue }
                    match points.get(&neighbour).unwrap() {
                        PATH => {
                            next_points.insert(neighbour);
                        },
                        WALL => {},
                        DOOR(ch) => {
                            let lower = ch.to_lowercase().next().unwrap();
                            if collected_keys.contains(&lower) {
                                next_points.insert(neighbour);
                            }
                        }
                        KEY(ch) => {
                            if !collected_keys.contains(ch) {
                                reachable_keys.insert((*ch, distance));
                            }
                            next_points.insert(neighbour);
                        }
                    }
                }
            }
            previous_points = next_points;
        }

        //For all reachable keys, recursive call
        for (key, dist) in reachable_keys {
            if collected_keys.contains(&key) {
                assert!(false);
            }

            //Update collected keys & position
            let key_loc = key_locations.get(&key).unwrap();
            collected_keys.push(key);
            currents[i] = *key_loc;

            let final_dist = getDistance(cache, points, key_locations, currents, collected_keys) + dist;
            min_dist = min_dist.min(final_dist);

            //Revert
            collected_keys.remove(collected_keys.len() - 1);
            currents[i] = current;
        }
    }

    //If nothing is reachable
    if min_dist == std::u64::MAX {
        cache.insert(cache_key, 0);
        return 0;
    }

    //Insert min distance
    cache.insert(cache_key, min_dist);
    return min_dist;
}

pub fn test() {
    let mut a = HashSet::new();
    a.insert(1);
    while !a.is_empty() {
        let mut b = HashSet::new();
        for i in &a{
            b.insert(i + 1);
            b.insert(i + 2);
        }
        a = b;
    }
}

pub enum Field {
    PATH,
    WALL,
    KEY(char),
    DOOR(char)
}

pub fn neighbours(points: &HashMap<(u64, u64), Field>, point: &(u64, u64)) -> HashSet<(u64, u64)> {
    let mut result = HashSet::new();

    if points.contains_key(&(point.0 + 1, point.1)) {
        result.insert((point.0 + 1, point.1));
    }
    if points.contains_key(&(point.0 - 1, point.1)) {
        result.insert((point.0 - 1, point.1));
    }
    if points.contains_key(&(point.0, point.1 + 1)) {
        result.insert((point.0, point.1 + 1));
    }
    if points.contains_key(&(point.0, point.1 - 1)) {
        result.insert((point.0, point.1 - 1));
    }

    return result;
}

#[cfg(test)]
mod test {
    use crate::day18::main2::main;

    #[test]
    fn test_day18_part2_0() {
        let input = "#######
#a.#Cd#
##@#@##
#######
##@#@##
#cB#Ab#
#######";
        let result = main(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_day18_part1_1() {
        let input = "###############
#d.ABC.#.....a#
######@#@######
###############
######@#@######
#b.....#.....c#
###############";
        let result = main(input);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_day18_part1_2() {
        let input = "#############
#DcBa.#.GhKl#
#.###@#@#I###
#e#d#####j#k#
###C#@#@###J#
#fEbA.#.FgHi#
#############";
        let result = main(input);
        assert_eq!(result, 32);
    }

    #[test]
    fn test_day18_part1_3() {
        let input = "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############";
        let result = main(input);
        assert_eq!(result, 72);
    }

    #[test]
    fn test_main_real() {
        let input = include_str!("input2.txt");
        let result = main(input);
        println!("Result: {}", result);
        assert_eq!(result, 1828);
    }
}
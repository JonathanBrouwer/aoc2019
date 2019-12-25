use std::collections::{HashMap, HashSet};

pub fn main(strin: &str, stepcount: u64) -> u64 {
    //Read input
    let mut state: HashMap<(u64, u64, i64), bool> = HashMap::new();
    for (y, line) in strin.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if x == 2 && y == 2 { continue; }
            state.insert((x as u64, y as u64, 0), ch == '#');
        }
    }

    //Loop for stepcount - 1 times, return result after stepcountth
    for _ in 0..(stepcount - 1) {
        step(&mut state);
    }
    let result = step(&mut state);

    //Print field
    let minl = state.iter().map(|(p, v)| p.2).min().unwrap();
    let maxl = state.iter().map(|(p, v)| p.2).max().unwrap();
    for l in minl..=maxl {
        println!("Level {}", l);
        for y in 0..5 {
            for x in 0..5 {
                if *state.get(&(x, y, l)).unwrap_or(&false) {
                    print!("#");
                }else {
                    print!(".");
                }
            }
            println!();
        }
    }
    return result;
}

pub fn step(state: &mut HashMap<(u64, u64, i64), bool>) -> u64 {
    //Remember old state
    let mut old_state = state.clone();

    //Insert new points where needed
    for (point, _val) in state.iter().filter(|(_p, v)| **v) {
        for neighbour in neighbours(point) {
            if !old_state.contains_key(&neighbour) {
                old_state.insert(neighbour, false);
            }
        }
    }

    //For each state, count the neighbours
    let mut active_count = 0;
    for (point, val) in old_state.iter() {
        //Count the neighbours
        let mut neighbour_count = 0;
        for neighbour in neighbours(point) {
            //If neighbour is active, count
            if *old_state.get(&neighbour).unwrap_or(&false) {
                neighbour_count += 1;
            }
        }

        //Get new state
        let active = neighbour_count == 1 || (!*val && neighbour_count == 2);
        if active { active_count += 1; };
        state.insert(*point, active);
    }

    return active_count;
}

pub fn neighbours(point: &(u64, u64, i64)) -> HashSet<(u64, u64, i64)> {
    let x = point.0;
    let y = point.1;
    let l = point.2;
    let mut neighbours = HashSet::new();

    //Get neighbours on same level
    if x != 4 { neighbours.insert((x + 1, y, l)); }
    if x != 0 { neighbours.insert((x - 1, y, l)); }
    if y != 4 { neighbours.insert((x, y + 1, l)); }
    if y != 0 { neighbours.insert((x, y - 1, l)); }
    neighbours.remove(&(2, 2, l));

    //Get neighbours on inner level
    for i in 0..5 {
        //Left
        if x == 1 && y == 2 { neighbours.insert((0, i, l + 1)); }
        //Right
        if x == 3 && y == 2 { neighbours.insert((4, i, l + 1)); }
        //Top
        if x == 2 && y == 1 { neighbours.insert((i, 0, l + 1)); }
        //Bottom
        if x == 2 && y == 3 { neighbours.insert((i, 4, l + 1)); }
    }

    //Get neighbours on outer level
    //Left
    if x == 0 { neighbours.insert((1, 2, l - 1)); }
    //Right
    if x == 4 { neighbours.insert((3, 2, l - 1)); }
    //Top
    if y == 0 { neighbours.insert((2, 1, l - 1)); }
    //Down
    if y == 4 { neighbours.insert((2, 3, l - 1)); }

    assert!(neighbours.len() == 4 || neighbours.len() == 8);
    return neighbours;
}

#[cfg(test)]
mod test {
    use crate::day24::main2::main;

    #[test]
    fn test_day24_part2_0() {
        let input = "....#
#..#.
#.?##
..#..
#....";
        let result = main(input, 10);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_day24_part2_real() {
        let input = include_str!("input.txt");
        let result = main(input, 200);
        println!("Result: {}", result);
        assert_eq!(result, 2109);

        /*
        Wrong:
        1832


        2109
        Too high:
        2110
        */
    }
}
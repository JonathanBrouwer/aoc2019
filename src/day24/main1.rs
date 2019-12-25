use std::collections::{HashMap, HashSet};

pub fn main(strin: &str) -> u64 {
    //Read input
    let mut state = [[false; 5]; 5];
    for (y,line) in strin.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            state[y][x] = ch == '#';
        }
    }

    //Loop until repeat
    let mut seen_states = HashSet::new();
    loop {
        //Next state
        step(&mut state);

        //Calculate bio div
        let bio_div = calculate_biodiversity(&state);
        if seen_states.contains(&bio_div) {
            return bio_div;
        }else{
            seen_states.insert(bio_div);
        }
    }
}

pub fn step(state: &mut [[bool; 5]; 5]) {
    //Make a copy of the state
    let mut old_state = [[false; 5]; 5];
    for (y, row) in state.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            old_state[y][x] = *val;
        }
    }

    //For each state, count the neighbours
    for y in 0..5 {
        for x in 0..5 {
            //Count the neighbours
            let mut neighbour_count = 0;
            if x != 4 && old_state[y][x+1] { neighbour_count += 1; }
            if x != 0 && old_state[y][x-1] { neighbour_count += 1; }
            if y != 4 && old_state[y+1][x] { neighbour_count += 1; }
            if y != 0 && old_state[y-1][x] { neighbour_count += 1; }

            //Get new state
            state[y][x] = neighbour_count == 1 || (!old_state[y][x] && neighbour_count == 2);
        }
    }
}

pub fn calculate_biodiversity(state: &[[bool; 5]; 5]) -> u64 {
    state.iter().flatten().scan(1, |s, &i| {
        *s *= 2;
        if i { Some(*s / 2) } else { Some(0) }
    }).sum()
}

#[cfg(test)]
mod test {
    use crate::day24::main1::{main};

    #[test]
    fn test_day24_part1_0() {
        let input = "....#
#..#.
#..##
..#..
#....";
        let result = main(input);
        assert_eq!(result, 2129920);
    }

    #[test]
    fn test_day24_part1_real() {
        let input = include_str!("input.txt");
        let result = main(input);
        println!("Result: {}", result);
        assert_eq!(result, 12129040);
    }
}
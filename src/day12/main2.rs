use std::cmp::Ordering;
use std::collections::HashSet;
use num::integer::{Integer};

pub fn main(input: &str) -> i64 {
    let mut moons: Vec<Moon> = Vec::new();
    for line in input.lines() {
        let mut pos: [i64; 3] = [0; 3];
        for (i, coord) in line.split(',').enumerate() {
            let coordi = coord.parse::<i64>();
            pos[i] = coordi.unwrap();
        }

        let parts = [
            MoonPart{pos: pos[0], vel: 0},
            MoonPart{pos: pos[1], vel: 0},
            MoonPart{pos: pos[2], vel: 0}
        ];

        moons.push(Moon{parts})
    }

    let mut counts = Vec::new();

    for axis in 0..3 {
        //Get parts for this axis
        let mut parts = Vec::new();
        for moon in moons.clone() {
            parts.push(moon.parts[axis].clone());
        }

        //Keep track of seen states
        let mut seen_states : HashSet<Vec<MoonPart>> = HashSet::new();
        seen_states.insert(parts.clone());

        //Keep simulating states until repeat
        let mut count: i64 = 0;
        loop {
            //Apply gravity
            let parts_old = parts.to_vec();
            for part in &mut parts {
                for gravity_part in &parts_old {
                    match part.pos.cmp(&gravity_part.pos) {
                        Ordering::Less => {
                            part.vel += 1;
                        },
                        Ordering::Equal => {},
                        Ordering::Greater => {
                            part.vel -= 1;
                        }
                    }
                }
            }

            //Apply velocity
            for part in &mut parts {
                part.pos += part.vel;
            }

            //Did it repeat?
            count += 1;
            if seen_states.contains(&parts) {
                break;
            }
            seen_states.insert(parts.clone());
        }

        //Store count
        counts.push(count);
    }

    let mut total: i64 = 1;
    for count in counts {
        total = total.lcm(&count);
    }

    return total;
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Moon {
    parts: [MoonPart; 3]
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MoonPart {
    pos: i64,
    vel: i64
}

#[cfg(test)]
mod test {
    use crate::day12::main2::main;

    #[test]
    fn test_day12_part2_1() {
        let input = "-8,-10,0
5,5,10
2,-7,3
9,-8,-3";
        let result = main(input);
        assert_eq!(result, 4686774924);
    }

    #[test]
    fn test_main_real() {
        let input = include_str!("input.txt");
        let result = main(input);
        println!("Result: {}", result);
        assert_eq!(result, 376203951569712);
    }
}
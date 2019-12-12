use std::cmp::Ordering;

pub fn main(input: &str, stepcount: i64) -> i64 {
    let mut moons: Vec<Moon> = Vec::new();
    for line in input.lines() {
        let mut pos: [i64; 3] = [0; 3];
        for (i, coord) in line.split(',').enumerate() {
            let coordi = coord.parse::<i64>();
            pos[i] = coordi.unwrap();
        }
        let vel = [0; 3];
        moons.push(Moon{pos, vel})
    }

    for _ in 0..stepcount {
        //Change velocity
        let moons_old = moons.to_vec();
        for moon in &mut moons {
            //Apply gravity
            for gravitymoon in &moons_old {
                //For each coordinate, update velocity
                for c in 0..3 {
                    match moon.pos[c].cmp(&gravitymoon.pos[c]) {
                        Ordering::Less => {
                            moon.vel[c] += 1;
                        },
                        Ordering::Equal => {},
                        Ordering::Greater => {
                            moon.vel[c] -= 1;
                        },
                    }
                }
            }

            //Apply velocity
            for c in 0..3 {
                moon.pos[c] += moon.vel[c];
            }
        }
    }

    return energy(&moons);
}

pub fn energy(moons: &Vec<Moon>) -> i64 {
    let mut toten = 0;
    for moon in moons {
        let mut poten = 0;
        let mut kinen = 0;
        for c in 0..3 {
            poten += moon.pos[c].abs();
            kinen += moon.vel[c].abs();
        }
        toten += poten * kinen;
    }
    return toten;
}

#[derive(Clone)]
pub struct Moon {
    pos: [i64; 3],
    vel: [i64; 3]
}

#[cfg(test)]
mod test {
    use crate::day12::main1::main;

    #[test]
    fn test_day12_part2_1() {
        let input = "-1,0,2
2,-10,-7
4,-8,8
3,5,-1";
        let result = main(input, 10);
        assert_eq!(result, 179);
    }

    #[test]
    fn test_day12_part2_2() {
        let input = "-8,-10,0
5,5,10
2,-7,3
9,-8,-3";
        let result = main(input, 100);
        assert_eq!(result, 1940);
    }

    #[test]
    fn test_main_real() {
        let input = include_str!("input.txt");
        let result = main(input, 1000);
        println!("Result: {}", result);
    }
}
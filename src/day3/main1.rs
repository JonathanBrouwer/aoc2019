use std::collections::{HashSet, HashMap};

pub fn main(input: &str) -> i64 {
    let lines: Vec<&str> = input.split('\n').collect();
    let path0 = parse_path(lines[0].trim_end());
    let path1 = parse_path(lines[1].trim_end());
    let inters = path0.intersection(&path1);

    //Find max
    let mut mindist = std::i64::MAX;
    for (x, y) in inters {
        let dist = i64::abs(*x) + i64::abs(*y);
        mindist = i64::min(dist, mindist);
    }

    return mindist;
}

pub fn parse_path(input: &str) -> HashSet<(i64, i64)> {
    let mut curpos: Position = Position { x: 0, y: 0 };

    let mut path = HashSet::new();

    for stepstr in input.split(',') {
        let substr: String = stepstr[1..].to_string();
        let dir = stepstr.chars().nth(0);
        let dis = stepstr[1..].parse().expect("Didn't find integer");

        for i in 1..dis {
            match dir {
                Some('U') => { path.insert((curpos.x, curpos.y + i)); }
                Some('D') => { path.insert((curpos.x, curpos.y - i)); }
                Some('R') => { path.insert((curpos.x + i, curpos.y)); }
                Some('L') => { path.insert((curpos.x - i, curpos.y)); }
                _ => panic!("Found invalid direction!")
            }
        }

        match dir {
            Some('U') => { curpos.y = curpos.y + dis; }
            Some('D') => { curpos.y = curpos.y - dis; }
            Some('R') => { curpos.x = curpos.x + dis; }
            Some('L') => { curpos.x = curpos.x - dis; }
            _ => panic!("Found invalid direction!")
        }
    }
    return path;
}

struct Position {
    x: i64,
    y: i64,
}

#[cfg(test)]
mod test {
    use crate::day3::main1::main;

    #[test]
    fn test_main_real() {
        let input = include_str!("input.txt");
        let result = main(input);
        println!("Result: {}", result);
    }

    #[test]
    fn example1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        let result = main(input);
        assert_eq!(159, result);
    }

    #[test]
    fn example2() {
        let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let result = main(input);
        assert_eq!(135, result);
    }
}
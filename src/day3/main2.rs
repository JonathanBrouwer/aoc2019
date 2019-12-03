use std::collections::{HashSet, HashMap};

pub fn main(input: &str) -> i64 {
    let lines: Vec<&str> = input.split('\n').collect();
    let (path0, dis0) = parse_path(lines[0].trim_end());
    let (path1, dis1) = parse_path(lines[1].trim_end());
    let inters = path0.intersection(&path1);

    //Find max
    let mut mindist = std::i64::MAX;
    for (x, y) in inters {
        let dist0 = dis0.get( &(*x, *y)).expect("Dist0 didnt contain");
        let dist1 = dis1.get( &(*x, *y)).expect("Dist1 didnt contain");
        mindist = i64::min(dist0 + dist1, mindist);
    }

    return mindist;
}

pub fn parse_path(input: &str) -> (HashSet<(i64, i64)>, HashMap<(i64, i64), i64>) {
    let mut curpos: Position = Position { x: 0, y: 0 };

    let mut path = HashSet::new();
    let mut distances: HashMap<(i64, i64), i64> = HashMap::new();
    let mut totaldis = 0;

    for stepstr in input.split(',') {
        let substr: String = stepstr[1..].to_string();
        let dir = stepstr.chars().nth(0);
        let dis = stepstr[1..].parse().expect("Didn't find integer");

        for i in 1..dis {
            let pos: (i64, i64);
            match dir {
                Some('U') => { pos = (curpos.x, curpos.y + i); }
                Some('D') => { pos = (curpos.x, curpos.y - i); }
                Some('R') => { pos = (curpos.x + i, curpos.y); }
                Some('L') => { pos = (curpos.x - i, curpos.y); }
                _ => panic!("Found invalid direction!")
            }
            path.insert(pos);
            if distances.get(&pos).is_none() {
                distances.insert(pos, totaldis + i);
            }
        }

        match dir {
            Some('U') => { curpos.y = curpos.y + dis; }
            Some('D') => { curpos.y = curpos.y - dis; }
            Some('R') => { curpos.x = curpos.x + dis; }
            Some('L') => { curpos.x = curpos.x - dis; }
            _ => panic!("Found invalid direction!")
        }
        totaldis += dis;
    }
    return (path, distances);
}

struct Position {
    x: i64,
    y: i64,
}

#[cfg(test)]
mod test {
    use crate::day3::main2::main;

    #[test]
    fn test_main_real() {
        let input = include_str!("input.txt");
        let result = main(input);
        println!("Result: {}", result);
    }

    #[test]
    fn simple() {
        let input = "R8,U5,L5,D3
U7,R6,D4,L4";
        let result = main(input);
        assert_eq!(30, result);
    }

    #[test]
    fn example1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        let result = main(input);
        assert_eq!(610, result);
    }

    #[test]
    fn example2() {
        let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let result = main(input);
        assert_eq!(410, result);
    }
}
use std::collections::{HashMap, HashSet};
use crate::day20::main2::Tile::{PATH, PORTAL};
use crate::day20::main2::Direction::{N, E, S, W};

pub fn main(strin: &str) -> i64 {
    //Parse input
    let mut tiles: HashMap<(i64, i64), Tile> = HashMap::new();
    let mut loc_portal: HashMap<(i64, i64), char> = HashMap::new();
    for (y, line) in strin.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                ' ' => {}
                '#' => {}
                '.' => {
                    tiles.insert((x as i64, y as i64), PATH);
                }
                'A'..='Z' => {
                    loc_portal.insert((x as i64, y as i64), ch);
                    tiles.insert((x as i64, y as i64), PATH); //Temp insert as path
                }
                _ => panic!()
            };
        }
    }

    //Parse portals
    let mut portal_loc: HashMap<String, HashSet<(i64, i64)>> = HashMap::new();
    for (pos, ch1) in &loc_portal {
        let neighbours = neighbours(&tiles, &pos);
        assert!(neighbours.len() == 1 || neighbours.len() == 2);

        let newpos;
        let newstr;

        if neighbours.len() == 2 {
            let mut iter = neighbours.iter();
            let (apos, adir) = iter.next().unwrap();
            let (bpos, bdir) = iter.next().unwrap();
            if loc_portal.contains_key(apos) {
                newstr = parse_portal(*ch1, *loc_portal.get(apos).unwrap(), *adir);
                newpos = *bpos;
            } else {
                newstr = parse_portal(*ch1, *loc_portal.get(bpos).unwrap(), *bdir);
                newpos = *apos;
            }
        } else {
            let (apos, adir) = neighbours.iter().next().unwrap();
            newpos = (apos.0 + (apos.0 - pos.0), apos.1 + (apos.1 - pos.1));
            newstr = parse_portal(*ch1, *loc_portal.get(apos).unwrap(), *adir);
        }

        if !portal_loc.contains_key(&newstr) {
            portal_loc.insert(newstr.clone(), HashSet::new());
        }
        portal_loc.get_mut(&newstr).unwrap().insert(newpos.clone());
        tiles.insert(newpos.clone(), PORTAL(newstr));
    }
    for (pos, _) in &loc_portal {
        tiles.remove(pos);
    }

    //Get minmax
    let mut minx = std::i64::MAX;
    let mut maxx = std::i64::MIN;
    let mut miny = std::i64::MAX;
    let mut maxy = std::i64::MIN;
    for (pos, tile) in &tiles {
        minx = minx.min(pos.0);
        maxx = maxx.max(pos.0);
        miny = miny.min(pos.1);
        maxy = maxy.max(pos.1);
    }
    let minmax = MinMax { minx, maxx, miny, maxy };

    //Breadth first search
    let mut visited: HashSet<((i64, i64), u64)> = HashSet::new();
    let mut to_visit: HashSet<((i64, i64), u64)> = HashSet::new();

    let start = (*portal_loc.get("AA").unwrap().iter().next().unwrap(), 0);
    let end = (*portal_loc.get("ZZ").unwrap().iter().next().unwrap(), 0);
    to_visit.insert(start);

    let mut distance = 0;
    while !to_visit.is_empty() {
        distance += 1;
        let mut new_to_visit: HashSet<((i64, i64), u64)> = HashSet::new();
        for (pos, depth) in &to_visit {
            //Regular neighbours
            for (nb_pos, _nb_dir) in neighbours(&tiles, &pos) {
                let newpos = (nb_pos, *depth);
                if !visited.contains(&newpos) {
                    visited.insert(newpos.clone());
                    new_to_visit.insert(newpos.clone());
                }
            }

            //Portal neighbours
            match tiles.get(pos).unwrap() {
                PATH => {}
                PORTAL(key) => {
                    for nb_pos in portal_loc.get(key).unwrap().iter() {
                        if *pos == *nb_pos { continue; }
                        let newpos = if is_outer_portal(&minmax, pos) {
                            if *depth == 0 { continue; }
                            (*nb_pos, *depth - 1)
                        } else {
                            (*nb_pos, *depth + 1)
                        };

                        if !visited.contains(&newpos) {
                            visited.insert(newpos.clone());
                            new_to_visit.insert(newpos.clone());
                        }
                    }
                }
            }
        }
        to_visit = new_to_visit;

        if to_visit.contains(&end) {
            return distance;
        }
    }
    panic!("Maze was impossible!");
}

#[derive(Eq, PartialEq, Hash)]
pub enum Tile {
    PATH,
    PORTAL(String),
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

pub struct MinMax {
    minx: i64,
    maxx: i64,
    miny: i64,
    maxy: i64,
}

pub fn is_outer_portal(mm: &MinMax, portal: &(i64, i64)) -> bool {
    return mm.minx == portal.0 || mm.maxx == portal.0 || mm.miny == portal.1 || mm.maxy == portal.1;
}

pub fn parse_portal(ch1: char, ch2: char, dir2: Direction) -> String {
    return match dir2 {
        N | W => {
            let mut s = String::from(ch2.to_string());
            s.push(ch1);
            s
        }
        S | E => {
            let mut s = String::from(ch1.to_string());
            s.push(ch2);
            s
        }
    };
}

pub fn neighbours(points: &HashMap<(i64, i64), Tile>, point: &(i64, i64)) -> HashSet<((i64, i64), Direction)> {
    let mut result = HashSet::new();

    if points.contains_key(&(point.0 + 1, point.1)) {
        result.insert(((point.0 + 1, point.1), E));
    }
    if points.contains_key(&(point.0 - 1, point.1)) {
        result.insert(((point.0 - 1, point.1), W));
    }
    if points.contains_key(&(point.0, point.1 + 1)) {
        result.insert(((point.0, point.1 + 1), S));
    }
    if points.contains_key(&(point.0, point.1 - 1)) {
        result.insert(((point.0, point.1 - 1), N));
    }

    return result;
}

#[cfg(test)]
mod test {
    use crate::day20::main2::main;

    #[test]
    fn test_day16_part1_0() {
        let input = "         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z
";
        let result = main(input);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_day16_part1_1() {
        let input = "             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M                     ";
        let result = main(input);
        assert_eq!(result, 396);
    }

    #[test]
    fn test_day16_part1_2() {
        let input = "         A
         A
  #######.#########
  #######.#.......#
  #######.#########
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C  XX.....XX
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z       ";
        let result = main(input);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_main_real() {
        let input = include_str!("input.txt");
        let result = main(input);
        println!("Result: {}", result);
        assert_eq!(result, 6546);
    }
}
use std::f64::consts::PI;
use std::collections::HashMap;
use std::cmp::Ordering::Equal;

#[derive(PartialEq, Debug)]
pub struct Point {
    x: i64,
    y: i64
}

pub fn main(input: &str, basex: i64, basey: i64, maxcount: i64) -> usize {
    let mut points: Vec<Point> = Vec::new();
    let mut base = Point { x: basex, y: basey };
    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                if x as i64 == base.x && y as i64 == base.y {
                    return;
                }
                points.push(Point {x: x as i64 - base.x, y: y as i64 - base.y});
            }
        });
    });


    // Sort points by angle
    let mut angle_to_points: Vec<(&Point, f64)> = Vec::new();
    for p in &points {
        let angle = angle(&p);
        angle_to_points.push((p, angle));
    }
    angle_to_points.sort_by(|a, b| {
        //First by angle
        a.1.partial_cmp(&b.1).unwrap()
            //Then by distance
            .then((a.0.x*a.0.x + a.0.y*a.0.y).cmp(
            &(b.0.x*b.0.x + b.0.y*b.0.y)))
    });

    //Continue to remove single points
    let mut i = 0;
    let mut count = 0;
    while !angle_to_points.is_empty() {
        //Get point & check count
        let (p, a) = angle_to_points.get(i).expect("Yeet1");
        count += 1;
        if count == maxcount {
            return ((p.x + base.x) * 100 + (p.y + base.y)) as usize;
        }

        //Remove
        let af = *a;
        angle_to_points.remove(i);

        //Skip over points with same angle
        while (angle_to_points.get(i).expect("Yeet2").1 - af).abs() < 0.00000001 {
            i+=1;
            i%=angle_to_points.len();
        }
    }

    panic!("Didn't find maxcount points.")
}

/**
* Gets the clockwise angle of p1  from the top.
*/
pub fn angle(p1: &Point) -> f64 {
    let mut start_point = Point {x: p1.x, y: p1.y + 100000};
    let mut curr_angle = ((p1.y) as f64).atan2((p1.x) as f64);
    let mut curr_top = (1.0 as f64).atan2(0.0 as f64);
    curr_angle += curr_top;
    curr_angle += 4.0 * PI;
    curr_angle %= 2.0 * PI;
    return curr_angle;
}

#[cfg(test)]
mod test {
    use crate::day10::main2::main;

    #[test]
    fn test_day8_part2_1() {
        let input = ".....
.###.
.###.
.###.
.....";
        let result = main(input, 2,2, 7);
        assert_eq!(result, 102);
    }

    #[test]
    fn test_day8_part2_2() {
        let input = ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";
        let result = main(input, 8, 3, 20);
        assert_eq!(result, 203);
    }

    #[test]
    fn test_day8_part2_3() {
        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        let result = main(input, 11, 13, 200);
        assert_eq!(result, 802);
    }

    #[test]
    fn test_main_real() {
        let input = include_str!("input.txt");
        let result = main(input, 22, 19, 200);
        println!("Result: {}", result);
    }
}
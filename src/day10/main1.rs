#[derive(PartialEq, Debug)]
pub struct Point {
    x: usize,
    y: usize
}

pub fn main(input: &str) -> usize {
    let mut points: Vec<Point> = Vec::new();
    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                points.push(Point {x, y});
            }
        });
    });

    // Check which points
    let mut max: usize = 0;
    let mut point: &Point= &Point{x:0,y:0};
    for p1 in &points {
        let mut count: usize = 0;
        'test: for p2 in &points {
            if p1 == p2 {
                continue;
            }
            for test in &points {
                if p1 == test || p2 == test {
                    continue;
                }
                if is_in_line(p1, p2, test) {
                    continue 'test;
                }
            }
            count += 1;
        }
        if(count > max) {
            point = p1;
            max = count;
        }

    }
    println!("{:?}", point);
    return max;
}

//Does test block the view from p1 to p2?
fn is_in_line(a: &Point, b: &Point, target: &Point) -> bool {
    let a1 = (a.x as f64, a.y as f64);
    let b1 = (b.x as f64, b.y as f64);
    let target1 = (target.x as f64, target.y as f64);

    is_in_line_f64(&a1, &b1, &target1)
}

fn is_in_line_f64(a: &(f64, f64), b: &(f64, f64), target: &(f64, f64)) -> bool {

    let dist_1 = dist(a, target);
    let dist_2 = dist(target, b);

    let dist_full = dist(a, b);

    dist_1 + dist_2 - dist_full < 0.00000001
}

fn dist(a: &(f64, f64), b: &(f64, f64)) -> f64 {
    (((a.0 - b.0).powf(2.0) + (a.1 - b.1).powf(2.0)) as f64).sqrt()
}

pub fn points_intersect(p1: &Point, p2: &Point, test: &Point) -> bool {
    let slope_tp1 =
        ((test.y as f64 - p1.y as f64))
        /((test.x as f64-p1.x as f64));
    let slope_p2p1 =
        ((p2.y as f64- p1.y as f64))
            /((p2.x as f64-p1.x as f64));
    if ((slope_p2p1.is_infinite() && slope_tp1.is_infinite())
        || (slope_tp1-slope_p2p1).abs() < 0.000001) {
        return manhatten_dist(test, p1) < manhatten_dist(p2, p1);
    }
    return false;
}

pub fn manhatten_dist(p1: &Point, p2: &Point) -> usize {
    return (p1.x as i64 - p2.x as i64).abs() as usize + (p1.y as i64 - p2.y as i64).abs() as usize;
}




#[cfg(test)]
mod test {
    use crate::day10::main1::main;

    #[test]
    fn test_day8_part1_1() {
        let input = ".#..#
.....
#####
....#
...##";
        let result = main(input);
        assert_eq!(result, 8);
    }
    #[test]
    fn test_day8_part1_2() {
        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        let result = main(input);
        assert_eq!(result, 33);
    }
    #[test]
    fn test_day8_part1_3() {
        let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        let result = main(input);
        assert_eq!(result, 35);
    }
    #[test]
    fn test_day8_part1_4() {
        let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
        let result = main(input);
        assert_eq!(result, 41);
    }
    #[test]
    fn test_day8_part1_5() {
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
        let result = main(input);
        assert_eq!(result, 210);
    }

    #[test]
    fn test_main_real() {
        let input = include_str!("input.txt");
        let result = main(input);
        println!("Result: {}", result);
    }
}
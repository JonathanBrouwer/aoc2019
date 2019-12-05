#[cfg(test)]
mod test {
    use crate::day5::main::{main,run};

    // === Day 2
    #[test]
    fn test_day2_1() {
        let mut program = vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50);
        let _outputs = run(&mut program, vec!());
        assert_eq!(program, vec!(3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50));
    }

    #[test]
    fn test_day2_2() {
        let mut program = vec!(1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,1,19,9,23,1,23,13,27,1,10,27,31,2,31,13,35,1,10,35,39,2,9,39,43,2,43,9,47,1,6,47,51,1,10,51,55,2,55,13,59,1,59,10,63,2,63,13,67,2,67,9,71,1,6,71,75,2,75,9,79,1,79,5,83,2,83,13,87,1,9,87,91,1,13,91,95,1,2,95,99,1,99,6,0,99,2,14,0,0);
        let _outputs = run(&mut program, vec!());
        assert_eq!(program, vec!(655695, 0, 0, 2, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 0, 1, 19, 9, 3, 1, 23, 13, 8, 1, 10, 27, 12, 2, 31, 13, 60, 1, 10, 35, 64, 2, 9, 39, 192, 2, 43, 9, 576, 1, 6, 47, 578, 1, 10, 51, 582, 2, 55, 13, 2910, 1, 59, 10, 2914, 2, 63, 13, 14570, 2, 67, 9, 43710, 1, 6, 71, 43712, 2, 75, 9, 131136, 1, 79, 5, 131137, 2, 83, 13, 655685, 1, 9, 87, 655688, 1, 13, 91, 655693, 1, 2, 95, 655693, 1, 99, 6, 0, 99, 2, 14, 0, 0));
    }

    // === Part 1
    #[test]
    fn test_part1_1() {
        let input = "3,0,4,0,99";
        let outputs = main(input, vec!(17));
        assert_eq!(outputs, vec!(17));
    }

    #[test]
    fn test_part1_2() {
        let input = "1002,4,3,4,33";
        let outputs = main(input, vec!(17));
        assert_eq!(outputs, vec!());
    }

    #[test]
    fn real_part1() {
        let input = include_str!("input.txt");
        let outputs = main(input, vec!(1));
        println!("Output part 1: {:?}", outputs);
        assert_eq!(outputs, vec!(0, 0, 0, 0, 0, 0, 0, 0, 0, 13210611));
    }

    #[test]
    fn test_part2_1() {
        let input = "3,9,8,9,10,9,4,9,99,-1,8";
        let outputs = main(input, vec!(17));
        assert_eq!(outputs, vec!(0));
    }

    #[test]
    fn test_part2_2() {
        let input = "3,9,8,9,10,9,4,9,99,-1,8";
        let outputs = main(input, vec!(8));
        assert_eq!(outputs, vec!(1));
    }

    #[test]
    fn test_part2_3() {
        let input = "3,9,7,9,10,9,4,9,99,-1,8";
        let outputs = main(input, vec!(8));
        assert_eq!(outputs, vec!(0));
    }

    #[test]
    fn test_part2_4() {
        let input = "3,9,7,9,10,9,4,9,99,-1,8";
        let outputs = main(input, vec!(7));
        assert_eq!(outputs, vec!(1));
    }

    #[test]
    fn test_part2_5() {
        let input = "3,3,1108,-1,8,3,4,3,99";
        let outputs = main(input, vec!(7));
        assert_eq!(outputs, vec!(0));
    }

    #[test]
    fn test_part2_6() {
        let input = "3,3,1108,-1,8,3,4,3,99";
        let outputs = main(input, vec!(8));
        assert_eq!(outputs, vec!(1));
    }

    #[test]
    fn test_part2_7() {
        let input = "3,3,1107,-1,8,3,4,3,99";
        let outputs = main(input, vec!(8));
        assert_eq!(outputs, vec!(0));
    }

    #[test]
    fn test_part2_8() {
        let input = "3,3,1107,-1,8,3,4,3,99";
        let outputs = main(input, vec!(7));
        assert_eq!(outputs, vec!(1));
    }

    #[test]
    fn test_part2_9() {
        let input = "3,3,1107,-1,8,3,4,3,99";
        let outputs = main(input, vec!(-99));
        assert_eq!(outputs, vec!(1));
    }

    #[test]
    fn test_part2_10() {
        let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let outputs = main(input, vec!(0));
        assert_eq!(outputs, vec!(0));
    }

    #[test]
    fn test_part2_11() {
        let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let outputs = main(input, vec!(1));
        assert_eq!(outputs, vec!(1));
    }

    #[test]
    fn test_part2_12() {
        let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        let outputs = main(input, vec!(0));
        assert_eq!(outputs, vec!(0));
    }

    #[test]
    fn test_part2_13() {
        let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        let outputs = main(input, vec!(1));
        assert_eq!(outputs, vec!(1));
    }

    #[test]
    fn test_part2_14() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let outputs = main(input, vec!(7));
        assert_eq!(outputs, vec!(999));
    }

    #[test]
    fn test_part2_15() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let outputs = main(input, vec!(8));
        assert_eq!(outputs, vec!(1000));
    }

    #[test]
    fn test_part2_16() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let outputs = main(input, vec!(9));
        assert_eq!(outputs, vec!(1001));
    }

    #[test]
    fn real_part2() {
        let input = include_str!("input.txt");
        let outputs = main(input, vec!(5));
        println!("Output part 2: {:?}", outputs);
        assert_eq!(outputs, vec!(584126));
    }
}
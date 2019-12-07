#[cfg(test)]
mod test {
    use crate::day7::main2::{parse,run};
    use crate::day7::main2::try_settings;

//    // === Day 2 ===
//    #[test]
//    fn test_day2_1() {
//        let mut program = vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50);
//        let _outputs = run(&mut program, vec!());
//        assert_eq!(program, vec!(3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50));
//    }
//
//    #[test]
//    fn test_day2_2() {
//        let mut program = vec!(1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,1,19,9,23,1,23,13,27,1,10,27,31,2,31,13,35,1,10,35,39,2,9,39,43,2,43,9,47,1,6,47,51,1,10,51,55,2,55,13,59,1,59,10,63,2,63,13,67,2,67,9,71,1,6,71,75,2,75,9,79,1,79,5,83,2,83,13,87,1,9,87,91,1,13,91,95,1,2,95,99,1,99,6,0,99,2,14,0,0);
//        let _outputs = run(&mut program, vec!());
//        assert_eq!(program, vec!(655695, 0, 0, 2, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 0, 1, 19, 9, 3, 1, 23, 13, 8, 1, 10, 27, 12, 2, 31, 13, 60, 1, 10, 35, 64, 2, 9, 39, 192, 2, 43, 9, 576, 1, 6, 47, 578, 1, 10, 51, 582, 2, 55, 13, 2910, 1, 59, 10, 2914, 2, 63, 13, 14570, 2, 67, 9, 43710, 1, 6, 71, 43712, 2, 75, 9, 131136, 1, 79, 5, 131137, 2, 83, 13, 655685, 1, 9, 87, 655688, 1, 13, 91, 655693, 1, 2, 95, 655693, 1, 99, 6, 0, 99, 2, 14, 0, 0));
//    }
//
//    // === Day 5 Part 1 ===
//    #[test]
//    fn test_part1_1() {
//        let input = "3,0,4,0,99";
//        let outputs = run(&mut parse(input), vec!(17));
//        assert_eq!(outputs, vec!(17));
//    }
//
//    #[test]
//    fn test_part1_2() {
//        let input = "1002,4,3,4,33";
//        let outputs = run(&mut parse(input), vec!(17));
//        assert_eq!(outputs, vec!());
//    }
//
//    // === Day 5 Part 2 ===
//    #[test]
//    fn test_part2_1() {
//        let input = "3,9,8,9,10,9,4,9,99,-1,8";
//        let outputs = run(&mut parse(input), vec!(17));
//        assert_eq!(outputs, vec!(0));
//    }
//
//    #[test]
//    fn test_part2_2() {
//        let input = "3,9,8,9,10,9,4,9,99,-1,8";
//        let outputs = run(&mut parse(input), vec!(8));
//        assert_eq!(outputs, vec!(1));
//    }
//
//    #[test]
//    fn test_part2_3() {
//        let input = "3,9,7,9,10,9,4,9,99,-1,8";
//        let outputs = run(&mut parse(input), vec!(8));
//        assert_eq!(outputs, vec!(0));
//    }
//
//    #[test]
//    fn test_part2_4() {
//        let input = "3,9,7,9,10,9,4,9,99,-1,8";
//        let outputs = run(&mut parse(input), vec!(7));
//        assert_eq!(outputs, vec!(1));
//    }
//
//    #[test]
//    fn test_part2_5() {
//        let input = "3,3,1108,-1,8,3,4,3,99";
//        let outputs = run(&mut parse(input), vec!(7));
//        assert_eq!(outputs, vec!(0));
//    }
//
//    #[test]
//    fn test_part2_6() {
//        let input = "3,3,1108,-1,8,3,4,3,99";
//        let outputs = run(&mut parse(input), vec!(8));
//        assert_eq!(outputs, vec!(1));
//    }
//
//    #[test]
//    fn test_part2_7() {
//        let input = "3,3,1107,-1,8,3,4,3,99";
//        let outputs = run(&mut parse(input), vec!(8));
//        assert_eq!(outputs, vec!(0));
//    }
//
//    #[test]
//    fn test_part2_8() {
//        let input = "3,3,1107,-1,8,3,4,3,99";
//        let outputs = run(&mut parse(input), vec!(7));
//        assert_eq!(outputs, vec!(1));
//    }
//
//    #[test]
//    fn test_part2_9() {
//        let input = "3,3,1107,-1,8,3,4,3,99";
//        let outputs = run(&mut parse(input), vec!(-99));
//        assert_eq!(outputs, vec!(1));
//    }
//
//    #[test]
//    fn test_part2_10() {
//        let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
//        let outputs = run(&mut parse(input), vec!(0));
//        assert_eq!(outputs, vec!(0));
//    }
//
//    #[test]
//    fn test_part2_11() {
//        let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
//        let outputs = run(&mut parse(input), vec!(1));
//        assert_eq!(outputs, vec!(1));
//    }
//
//    #[test]
//    fn test_part2_12() {
//        let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
//        let outputs = run(&mut parse(input), vec!(0));
//        assert_eq!(outputs, vec!(0));
//    }
//
//    #[test]
//    fn test_part2_13() {
//        let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
//        let outputs = run(&mut parse(input), vec!(1));
//        assert_eq!(outputs, vec!(1));
//    }
//
//    #[test]
//    fn test_part2_14() {
//        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
//        let outputs = run(&mut parse(input), vec!(7));
//        assert_eq!(outputs, vec!(999));
//    }
//
//    #[test]
//    fn test_part2_15() {
//        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
//        let outputs = run(&mut parse(input), vec!(8));
//        assert_eq!(outputs, vec!(1000));
//    }
//
//    #[test]
//    fn test_part2_16() {
//        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
//        let outputs = run(&mut parse(input), vec!(9));
//        assert_eq!(outputs, vec!(1001));
//    }

    // === Day 7 Part 1 ===
//    #[test]
//    fn test_day7_part1_1() {
//        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
//        let output = try_settings(input);
//        assert_eq!(output, 43210);
//    }
//
//    #[test]
//    fn test_day7_part1_2() {
//        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
//        let output = try_settings(input);
//        assert_eq!(output, 54321);
//    }
//
//    #[test]
//    fn test_day7_part1_3() {
//        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
//        let output = try_settings(input);
//        assert_eq!(output, 65210);
//    }
//
//    #[test]
//    fn test_day7_part1_real() {
//        let input = include_str!("input.txt");
//        let output = try_settings(input);
//        assert_eq!(output, 1001);
//    }

    // === Day 7 Part 2 ===
    #[test]
    fn test_day7_part2_1() {
        let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let output = try_settings(input);
        assert_eq!(output, 139629729);
    }

    #[test]
    fn test_day7_part2_2() {
        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let output = try_settings(input);
        assert_eq!(output, 18216);
    }

    #[test]
    fn test_day7_part2_real() {
        let input = include_str!("input.txt");
        let output = try_settings(input);
        assert_eq!(output, 8271623);
    }
}
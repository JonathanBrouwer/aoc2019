#[cfg(test)]
mod test {
    use crate::day15::main2::{parse, run, parse_run, play};

    // === Day 5 Part 1 ===
    #[test]
    fn test_part1_1() {
        let input = "3,0,4,0,99";
        let outputs = parse_run(input, vec!(17));
        assert_eq!(outputs, (vec!(17), true));
    }

    #[test]
    fn test_part1_2() {
        let input = "1002,4,3,4,33";
        let outputs = parse_run(input, vec!(17));
        assert_eq!(outputs, (vec!(), true));
    }

    // === Day 5 Part 2 ===
    #[test]
    fn test_part2_1() {
        let input = "3,9,8,9,10,9,4,9,99,-1,8";
        let outputs = parse_run(input, vec!(17));
        assert_eq!(outputs, (vec!(0), true));
    }

    #[test]
    fn test_part2_2() {
        let input = "3,9,8,9,10,9,4,9,99,-1,8";
        let outputs = parse_run(input, vec!(8));
        assert_eq!(outputs, (vec!(1), true));
    }

    #[test]
    fn test_part2_3() {
        let input = "3,9,7,9,10,9,4,9,99,-1,8";
        let outputs = parse_run(input, vec!(8));
        assert_eq!(outputs, (vec!(0), true));
    }

    #[test]
    fn test_part2_4() {
        let input = "3,9,7,9,10,9,4,9,99,-1,8";
        let outputs = parse_run(input, vec!(7));
        assert_eq!(outputs, (vec!(1), true));
    }

    #[test]
    fn test_part2_5() {
        let input = "3,3,1108,-1,8,3,4,3,99";
        let outputs = parse_run(input, vec!(7));
        assert_eq!(outputs, (vec!(0), true));
    }

    #[test]
    fn test_part2_6() {
        let input = "3,3,1108,-1,8,3,4,3,99";
        let outputs = parse_run(input, vec!(8));
        assert_eq!(outputs, (vec!(1), true));
    }

    #[test]
    fn test_part2_7() {
        let input = "3,3,1107,-1,8,3,4,3,99";
        let outputs = parse_run(input, vec!(8));
        assert_eq!(outputs, (vec!(0), true));
    }

    #[test]
    fn test_part2_8() {
        let input = "3,3,1107,-1,8,3,4,3,99";
        let outputs = parse_run(input, vec!(7));
        assert_eq!(outputs, (vec!(1), true));
    }

    #[test]
    fn test_part2_9() {
        let input = "3,3,1107,-1,8,3,4,3,99";
        let outputs = parse_run(input, vec!(-99));
        assert_eq!(outputs, (vec!(1), true));
    }

    #[test]
    fn test_part2_10() {
        let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let outputs = parse_run(input, vec!(0));
        assert_eq!(outputs, (vec!(0), true));
    }

    #[test]
    fn test_part2_11() {
        let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let outputs = parse_run(input, vec!(1));
        assert_eq!(outputs, (vec!(1), true));
    }

    #[test]
    fn test_part2_12() {
        let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        let outputs = parse_run(input, vec!(0));
        assert_eq!(outputs, (vec!(0), true));
    }

    #[test]
    fn test_part2_13() {
        let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        let outputs = parse_run(input, vec!(1));
        assert_eq!(outputs, (vec!(1), true));
    }

    #[test]
    fn test_part2_14() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let outputs = parse_run(input, vec!(7));
        assert_eq!(outputs, (vec!(999), true));
    }

    #[test]
    fn test_part2_15() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let outputs = parse_run(input, vec!(8));
        assert_eq!(outputs, (vec!(1000), true));
    }

    #[test]
    fn test_part2_16() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let outputs = parse_run(input, vec!(9));
        assert_eq!(outputs, (vec!(1001), true));
    }

    // === Day 7 Part 1 ===
    #[test]
    fn test_day7_part1_1() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let outputs = parse_run(input, vec!());
        assert_eq!(outputs, (vec!(109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99), true));
    }

    #[test]
    fn test_day7_part1_2() {
        let input = "1102,34915192,34915192,7,4,7,99,0";
        let outputs = parse_run(input, vec!());
        assert_eq!(outputs, (vec!(1219070632396864), true));
    }

    #[test]
    fn test_day7_part1_3() {
        let input = "104,1125899906842624,99";
        let outputs = parse_run(input, vec!());
        assert_eq!(outputs, (vec!(1125899906842624), true));
    }

    // === Day 9 Part 1 ===

    #[test]
    fn test_day15_part2_real() {
        let input = include_str!("input.txt");
        let output = play(input);
        println!("Answer: {}", output);
        assert_eq!(output, 358);
    }
}
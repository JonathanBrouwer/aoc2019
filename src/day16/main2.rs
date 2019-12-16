pub fn main(strin: &str) -> u64 {
    //Get the input, times 10000
    let mut input: Vec<u8> = strin.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .cycle()
        .take(strin.len() * 10000).collect();

    //Calculate the message offset
    let mut offset: u64 = 0;
    for i in 0..7 {
        offset *= 10;
        offset += input[i] as u64;
    }

    //Do some nice summing
    //We will assume that offset is after the middle
    //After the middle, it's a triangular matrix of 1s.
    let top = input.len() - 1;
    let mid = input.len() / 2;
    for _ in 0..100 {
        //From middle to end, in reverse, calculate sums
        for i in (mid..top).rev() {
            input[i] = (input[i] + input[i + 1]) % 10;
        }
    }

    //Calculate output
    let mut final_output : u64 = 0;
    for i in 0..8 {
        final_output *= 10;
        final_output += input[(offset + i) as usize] as u64;
    }

    return final_output;
}

#[cfg(test)]
mod test {
    use crate::day16::main2::main;

    #[test]
    fn test_day16_part1_1() {
        let input = "03036732577212944063491565474664";
        let result = main(input);
        assert_eq!(result, 84462026);
    }

    #[test]
    fn test_day16_part1_2() {
        let input = "02935109699940807407585447034323";
        let result = main(input);
        assert_eq!(result, 78725270);
    }

    #[test]
    fn test_day16_part1_3() {
        let input = "03081770884921959731165446850517";
        let result = main(input);
        assert_eq!(result, 53553731);
    }

    #[test]
    fn test_main_real() {
        let input = include_str!("input.txt");
        let result = main(input);
        println!("Result: {}", result);
        assert_eq!(result, 28135104);
    }
}
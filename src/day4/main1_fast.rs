use core::cmp;
use std::time::Instant;

pub fn main() -> usize {
    let now = Instant::now();

    let low = 138307;
    let high = 654504;

    let mut count = 0;
    let mut current: Vec<char> = (low-1).to_string().chars().collect();
    let mut cur_num: usize = low;
    generate_next(&mut current);

    while cur_num < high {
        if is_valid(&mut current) {
            count += 1;
        }
        generate_next(&mut current);
        let cur_str: String = (&current).into_iter().collect();
        cur_num = cur_str.parse::<usize>().expect("Int");
    }

    println!("Time: {}ms", now.elapsed().as_millis());
    return count;
}

pub fn is_valid(chars: &Vec<char>) -> bool {
    //Check if there's a group of 2
    for i in chars {
        if chars.iter().filter(|&n| n == i).count() >= 2 {
            return true;
        }
    }
    return false;
}

pub fn generate_next(chars: &mut Vec<char>) {
    //Increment by one
    for i in (0..(chars.len())).rev() {
        if chars[i] == '9' {
            chars[i] = '0';
        } else {
            chars[i] = std::char::from_u32(chars[i] as u32 + 1).expect("Couldn't convert to ascii.");
            break;
        }
    }

    //Make sure it's non-decreasing
    for i in 0..(chars.len() - 1) {
        chars[i+1] = cmp::max(chars[i], chars[i+1]);
    }
}

#[cfg(test)]
mod test {
    use crate::day4::main1::main;
    use crate::day4::main1::is_valid;

    #[test]
    fn real() {
        let result = main();
        println!("Result: {}", result);
    }

//    #[test]
//    fn test1() {
//        assert!(is_valid(&111111));
//    }
//
//    #[test]
//    fn test2() {
//        assert!(!is_valid(&223450));
//    }
//
//    #[test]
//    fn test3() {
//        assert!(!is_valid(&123789));
//    }
}
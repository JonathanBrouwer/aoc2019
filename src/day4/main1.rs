pub fn main() -> usize {
    let low = 138307;
    let high = 654504;

    return (low..high).filter(is_valid).count() as usize;
}

pub fn is_valid(i: &i32) -> bool {
    let chars: Vec<char> = i.to_string().chars().collect();

    //Check ascending
    for i in chars.windows(2) {
        if i[0] > i[1] {
            return false;
        }
    }

    //Check if there's a group of 2
    for i in &chars {
        if chars.iter().filter(|&n| n == i).count() >= 2 {
            return true;
        }
    }
    return false;
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

    #[test]
    fn test1() {
        assert!(is_valid(&111111));
    }

    #[test]
    fn test2() {
        assert!(!is_valid(&223450));
    }

    #[test]
    fn test3() {
        assert!(!is_valid(&123789));
    }
}
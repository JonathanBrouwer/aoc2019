use std::collections::{HashSet, HashMap};

pub fn main() -> i64 {
    let mut count = 0;
    for i in 138307..654504 {
        if(is_valid(i)) {
            count+=1;
        }
    }
    println! {"{}", count};
    return 0;
}

pub fn is_valid(i: i32) -> bool {
    let mut istr2 = i.to_string();
    let mut istr = istr2.chars();
    let mut curchar = istr.next().expect("No next char");
    let mut charcount: HashMap<char, i32> = HashMap::new();
    charcount.insert(curchar, 1);
    for i in 0..5 {
        let newchar = istr.next().expect("No next char");
        if newchar >= curchar {
            charcount.insert(newchar, *charcount.get(&newchar).get_or_insert(&0) + 1);
            curchar = newchar;
        } else {
            return false;
        }
    }

    for (k, v) in charcount {
        if v >= 2 {
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
        assert!(is_valid(111111));
    }

    #[test]
    fn test2() {
        assert!(!is_valid(223450));
    }

    #[test]
    fn test3() {
        assert!(!is_valid(123789));
    }
}
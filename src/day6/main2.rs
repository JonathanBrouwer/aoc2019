use std::collections::{HashMap, HashSet};

pub fn main(program: &str) -> i64 {
    let mut edgesIn: HashMap<&str, &str> = HashMap::new();
    let mut edgesOut: HashMap<&str, HashSet<&str>> = HashMap::new();

    for st in program.lines() {
        let both: Vec<&str> = st.split(")").collect();
        let inner = both[0];
        let outer = both[1];

        edgesIn.insert(outer, inner);
        let mut old = HashSet::new();
        if edgesOut.contains_key(inner) {
            old = edgesOut.get(inner).expect("Wut2").clone()
        }
        old.insert(outer);
        edgesOut.insert(inner, old.clone());
    }

    let mut hasBeen: HashSet<&str> = HashSet::new();
    return count("YOU", &edgesIn, &edgesOut, &mut hasBeen).expect("Didn't find santa") - 2;
}

pub fn count<'a>(from: &'a str, edgesIn: &HashMap<&'a str, &'a str>, edgesOut: &HashMap<&'a str,
    HashSet<&'a str>>, hasBeen: &mut HashSet<&'a str>) -> Option<i64> {
    hasBeen.insert(from);

    if(from == "SAN") {
        return Option::Some(0);
    }

    let mut out = Option::None;
    if edgesIn.contains_key(from) {
        let to = edgesIn.get(from).expect("Wut");
        if !hasBeen.contains(to) {
            out = out.or(count(to, edgesIn, edgesOut, hasBeen));
        }
    }
    if edgesOut.contains_key(from) {
        for to in edgesOut.get(from).expect("") {
            if !hasBeen.contains(to) {
                out = out.or(count(to, edgesIn, edgesOut, hasBeen));
            }
        }
    }

    if(out.is_some()) {
        return Option::Some(out.unwrap() + 1);
    }
    return out;
}

#[cfg(test)]
mod test {
    use crate::day6::main2::{main};

    #[test]
    fn test_part2_1() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        let output = main(input);
        assert_eq!(output, 4);
    }

    #[test]
    fn real_part2() {
        let input = include_str!("input.txt");
        let outputs = main(input);
        println!("Output part 1: {:?}", outputs);
        assert_eq!(outputs, 481);
    }
}
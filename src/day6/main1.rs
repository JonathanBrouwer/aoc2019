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
            old = edgesOut.get(inner).expect("Wut").clone()
        }
        old.insert(outer);
        edgesOut.insert(inner, old.clone());
    }

    return count("COM", &edgesOut).1;
}

pub fn count(from: &str, edgesOut: &HashMap<&str, HashSet<&str>>) -> (i64, i64) {
    if !edgesOut.contains_key(from) {
        return (1, 0);
    }
    let mut out = (1, 0);
    for to in edgesOut.get(from).expect("") {
        let rtrn = count(to, edgesOut);
        out.0 += rtrn.0;
        out.1 += rtrn.1 + rtrn.0;
    }
    return out;
}

#[cfg(test)]
mod test {
    use crate::day6::main1::{main};

    #[test]
    fn test_part1_1() {
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
K)L";
        let output = main(input);
        assert_eq!(output, 42);
    }

    #[test]
    fn real_part1() {
        let input = include_str!("input.txt");
        let outputs = main(input);
        println!("Output part 1: {:?}", outputs);
        assert_eq!(outputs, 315757);
    }
}
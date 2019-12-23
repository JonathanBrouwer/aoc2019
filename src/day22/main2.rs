use std::collections::{HashMap, HashSet};

pub fn main(strin: &str, deck_size: u128, repeat: u128, position: u128) -> u128 {
    let mut current = LinearModularFunction {
        factor: 1,
        add: 0,
        modulo: deck_size
    };

    //Read in actions
    for action in strin.lines().rev() {
        //Execute the action
        if action.eq("deal into new stack") {
            current = current.then(&deal_new_stack_rv(deck_size));
        } else if action.starts_with("cut ") {
            let line_input: Vec<&str> = action.split(" ").collect();
            let amount = line_input[1].parse::<i128>().unwrap();
            current = current.then(&cut_rv(deck_size, amount));
        } else if action.starts_with("deal with increment ") {
            let line_input: Vec<&str> = action.split(" ").collect();
            let amount = line_input[3].parse::<u128>().unwrap();
            current = current.then(&deal_with_increment_rv(deck_size, amount));
        } else {
            panic!("Invalid instruction");
        }
    }

    current = current.power(repeat);
    return current.apply(position as i128);
}

pub fn deal_new_stack_rv(size: u128) -> LinearModularFunction {
    return LinearModularFunction {
        factor: (size - 1) as i128,
        add: (size - 1) as i128,
        modulo: size,
    }
}

pub fn cut_rv(size: u128, amount: i128) -> LinearModularFunction {
    return LinearModularFunction {
        factor: 1,
        add: modulo(amount, size as i128),
        modulo: size
    };
}

pub fn deal_with_increment_rv(size: u128, amount: u128) -> LinearModularFunction {
    let inv = modinverse::modinverse(amount as i128, size as i128).unwrap();
    return LinearModularFunction {
        factor: inv,
        add: 0,
        modulo: size
    };
}

pub fn modulo(x: i128, m: i128) -> i128 {
    let r = x%m;
    return if r < 0 { r + m } else { r };
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct LinearModularFunction {
    factor: i128,
    add: i128,
    modulo: u128
}

impl LinearModularFunction {
    pub fn apply(&self, to: i128) -> u128 {
        return modulo(self.factor * to as i128 + self.add, self.modulo as i128) as u128;
    }

    pub fn power(&self, to: u128) -> LinearModularFunction {
        let mut iterations = to;
        let mut result = LinearModularFunction {
            factor: 1, add: 0, modulo: self.modulo
        };
        let mut power = self.clone();
        while iterations != 0 {
            if iterations % 2 != 0 {
                result = result.then(&power);
            }
            power = power.then(&power);
            iterations /= 2;
        }
        return result;
    }

    pub fn then(&self, next: &LinearModularFunction) -> LinearModularFunction {
        assert_eq!(self.modulo, next.modulo);

        //c(ax+b) + d mod n
        //cax + cb + d mod n
        return LinearModularFunction {
            factor: modulo(next.factor * self.factor, self.modulo as i128),
            add: modulo(next.factor * self.add + next.add, self.modulo as i128),
            modulo: self.modulo
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day22::main2::{main, deal_new_stack_rv, LinearModularFunction, cut_rv, deal_with_increment_rv};

    #[test]
    fn test_day22_part2_methods() {
        assert_eq!(deal_new_stack_rv(5), LinearModularFunction{factor: 4, add: 4, modulo: 5});
        assert_eq!(cut_rv(5, 2), LinearModularFunction{factor: 1, add: 2, modulo: 5});
        assert_eq!(cut_rv(5, -2), LinearModularFunction{factor: 1, add: 3, modulo: 5});
        assert_eq!(deal_with_increment_rv(5, 2), LinearModularFunction{factor: 3, add: 0, modulo: 5});
    }

    #[test]
    fn test_day22_part2_0() {
        let input = "deal with increment 7
deal into new stack
deal into new stack";
        let result = main(input, 10, 1, 0);
        assert_eq!(result, 0);
        //assert_eq!(result, vec!(0, 3, 6, 9, 2, 5, 8, 1, 4, 7));
    }

    #[test]
    fn test_day22_part2_1() {
        let input = "cut 6
deal with increment 7
deal into new stack";
        let result = main(input, 10, 1, 0);
        assert_eq!(result, 3);
//        assert_eq!(result, vec!(3, 0, 7, 4, 1, 8, 5, 2, 9, 6));
    }

    #[test]
    fn test_day22_part2_2() {
        let input = "deal with increment 7
deal with increment 9
cut -2";
        let result = main(input, 10, 1, 0);
        assert_eq!(result, 6);
//        assert_eq!(result, vec!(6, 3, 0, 7, 4, 1, 8, 5, 2, 9));
    }

    #[test]
    fn test_day22_part2_3() {
        let input = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";
        let result = main(input, 10, 1, 0);
        assert_eq!(result, 9);
//        assert_eq!(result, vec!(9, 2, 5, 8, 1, 4, 7, 0, 3, 6));
    }

    #[test]
    fn test_day22_part2_real() {
        let input = include_str!("input.txt");
        let result = main(input, 119315717514047, 101741582076661, 2020);
        println!("Result: {}", result);
        assert_eq!(result, 77225522112241);

        /*
        Wrong:
        32855988950697
        34651451052212
        49836800999463
        77225522112241
        86653852026288
        */
    }
}
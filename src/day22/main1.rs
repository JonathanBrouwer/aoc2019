use std::collections::{HashMap, HashSet};

pub fn card_at_pos(strin: &str, deck_size: u64, card_at_pos: usize) -> usize {
    let output = main(strin, deck_size);

    for (i, val) in output.iter().enumerate() {
        if *val == card_at_pos as u64 {
            return i;
        }
    }
    panic!("Didn't find card at pos.");
}

pub fn main(strin: &str, deck_size: u64) -> Vec<u64> {
    let mut deck: Vec<u64> = Vec::new();
    for i in 0..deck_size {
        deck.push(i as u64);
    }

    //Read in actions
    for action in strin.lines() {
        //Execute the action
        if action.eq("deal into new stack") {
            deal_new_stack(&mut deck);
        } else if action.starts_with("cut ") {
            let line_input: Vec<&str> = action.split(" ").collect();
            cut(&mut deck, line_input[1].parse::<i64>().unwrap());
        } else if action.starts_with("deal with increment ") {
            let line_input: Vec<&str> = action.split(" ").collect();
            deal_with_increment(&mut deck, line_input[3].parse::<u64>().unwrap());
        } else {
            panic!("Invalid instruction");
        }

        //Assert that action is valid
        let mut map = HashSet::new();
        for card in &deck {
            map.insert(card);
        }
        assert_eq!(map.len(), deck_size as usize);
    }

    return deck;
}

pub fn deal_new_stack(deck: &mut Vec<u64>) {
    deck.reverse();
}

pub fn cut(deck: &mut Vec<u64>, amount: i64) {
    if amount > 0 {
        deck.rotate_left(amount.abs() as usize);
    }else{
        deck.rotate_right(amount.abs() as usize);
    }
}

pub fn deal_with_increment(deck: &mut Vec<u64>, amount: u64) {
    let mut output: Vec<u64> = vec![0; deck.len()];
    for i in 0..deck.len() {
        output[(i * amount as usize) % deck.len()] = deck[i];
    }
    for i in 0..deck.len() {
        deck[i] = output[i];
    }
}

#[cfg(test)]
mod test {
    use crate::day22::main1::{main, card_at_pos};

    #[test]
    fn test_day22_part1_0() {
        let input = "deal with increment 7
deal into new stack
deal into new stack";
        let result = main(input, 10);
        assert_eq!(result, vec!(0, 3, 6, 9, 2, 5, 8, 1, 4, 7));
    }

    #[test]
    fn test_day22_part1_1() {
        let input = "cut 6
deal with increment 7
deal into new stack";
        let result = main(input, 10);
        assert_eq!(result, vec!(3, 0, 7, 4, 1, 8, 5, 2, 9, 6));
    }

    #[test]
    fn test_day22_part1_2() {
        let input = "deal with increment 7
deal with increment 9
cut -2";
        let result = main(input, 10);
        assert_eq!(result, vec!(6, 3, 0, 7, 4, 1, 8, 5, 2, 9));
    }

    #[test]
    fn test_day22_part1_3() {
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
        let result = main(input, 10);
        assert_eq!(result, vec!(9, 2, 5, 8, 1, 4, 7, 0, 3, 6));
    }

    #[test]
    fn test_day22_part1_real() {
        let input = include_str!("input.txt");
        let result = card_at_pos(input, 10007, 2019);
        println!("Result: {}", result);
        assert_eq!(result, 3749);
    }
}
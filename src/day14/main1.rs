use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::BorrowMut;
use std::collections::HashMap;

pub fn findOreNeeded(input: &str, fuel_count: i64) -> i64 {
    //Parse recipes
    let mut recipes: Vec<Recipe> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" => ").collect();

        //Input
        let mut inputs = Vec::new();
        for input in parts[0].split(", ") {
            let parts: Vec<&str> = input.split(" ").collect();
            let count: i64 = parts[0].parse().unwrap();
            inputs.push(Thing{ name: parts[1], count });
        }

        //Output
        let parts: Vec<&str> = parts[1].split(" ").collect();
        let count: i64 = parts[0].parse().unwrap();
        let output = Thing { name: parts[1], count };

        recipes.push(Recipe {inputs, output});
    }

    //Recipe maps has recipe for each output, except ore
    let mut recipes_map: HashMap<&str, &Recipe> = HashMap::new();
    for recipe in &recipes {
        recipes_map.insert(recipe.output.name, &recipe);
    }

    //Depend counts tracks how many recipes will still produce (need) something as input
    let mut depend_counts: HashMap<&str, i64> = HashMap::new();
    depend_counts.insert("ORE", 0);
    for recipe in &recipes {
        depend_counts.insert(recipe.output.name, 0);
    }
    for recipe in &recipes {
        for input in &recipe.inputs {
            depend_counts.insert(input.name, depend_counts.get(input.name).unwrap() + 1);
        }
    }

    //Current counts tracks how much we have of something
    let mut current_counts: HashMap<&str, i64> = HashMap::new();
    current_counts.insert("FUEL", fuel_count);

    //Loop and find roots of the DAG
    while !depend_counts.is_empty() {
        //Does anything still require ore?
        if *depend_counts.get("ORE").unwrap() == 0 {
            return *current_counts.get("ORE").unwrap();
        }

        //Get all recipes which are done
        let done_recipes: Vec<&&Recipe> = depend_counts.iter()
            .filter(|(_k, v)| **v == 0)
            .map(|(k, _v)| recipes_map.get(k).unwrap())
            .collect();

        //Handle each done recipe
        for done in done_recipes {
            //Remove from depend counts, so it's only handled once
            depend_counts.remove(done.output.name);

            //Calculate how many times to use recipe
            let to_create = current_counts.get(done.output.name).unwrap_or(&0);
            let recipe_times = if to_create % done.output.count == 0 {
                to_create / done.output.count
            } else {
                to_create / done.output.count + 1
            };

            //Handle inputs
            for input in &done.inputs {
                //Update current counts
                let mut current = *current_counts.get(input.name).unwrap_or(&0);
                current += recipe_times * input.count;
                current_counts.insert(input.name, current);

                //Decrement depend counts
                depend_counts.insert(input.name, depend_counts.get(input.name).unwrap() - 1);
            }
        }
    }

    return 0;
}

#[derive(Clone)]
pub struct Recipe<'a> {
    inputs: Vec<Thing<'a>>,
    output: Thing<'a>,
}

#[derive(Clone)]
pub struct Thing<'a> {
    name: &'a str,
    count: i64
}

#[cfg(test)]
mod test {
    use crate::day14::main1::findOreNeeded;

    #[test]
    fn test_day8_part1_0() {
        let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        let result = findOreNeeded(input, 1);
        assert_eq!(result, 31);
    }

    #[test]
    fn test_day8_part1_1() {
        let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        let result = findOreNeeded(input, 1);
        assert_eq!(result, 165);
    }

    #[test]
    fn test_day8_part1_2() {
        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let result = findOreNeeded(input, 1);
        assert_eq!(result, 13312);
    }

    #[test]
    fn test_day8_part1_3() {
        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        let result = findOreNeeded(input, 1);
        assert_eq!(result, 180697);
    }

    #[test]
    fn test_day8_part1_4() {
        let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        let result = findOreNeeded(input, 1);
        assert_eq!(result, 2210736);
    }

    #[test]
    fn test_main_real() {
        let input = include_str!("input.txt");
        let result = findOreNeeded(input, 1);
        println!("Result: {}", result);
        assert_eq!(result, 532506);
    }
}
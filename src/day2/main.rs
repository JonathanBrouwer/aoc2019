pub fn main(input: &str) -> usize {
    //Put all numbers in vector
    let mut memory: Vec<usize> = Vec::new();
    for st in input.split(',') {
        let num: usize = st.parse::<usize>().expect("Didn't get a number!");
        memory.push(num);
    }

    //Check all nouns and verbs between 1 and 99
    for noun in 1..=99 {
        for verb in 1..=99 {
            let mut memory_cln = memory.clone();
            memory_cln[1] = noun;
            memory_cln[2] = verb;
            let output = run(memory_cln);
            if output == 19690720 {
                return noun * 100 + verb;
            }
        }
    }

    //Output
    panic!("No output found!");
}

pub fn run(mut memory: Vec<usize>) -> usize {
    //Run program
    let mut current = 0;
    while memory[current] != 99 {
        //Get operator parameters
        let param_a = memory[current + 1];
        let param_b = memory[current + 2];
        let output_index = memory[current + 3];

        match memory[current] {
            1 => {
                memory[output_index] = memory[param_a] + memory[param_b];
                current += 4;
            }
            2 => {
                memory[output_index] = memory[param_a] * memory[param_b];
                current += 4;
            }
            _ => {
                panic!("Invalid op-code.");
            }
        }
    }

    //Output is address 0
    return memory[0];
}


#[cfg(test)]
mod test {
    use crate::day2::main::main;

    #[test]
    fn test_main_real() {
        let input = include_str!("input.txt");
        let result = main(input);
        println!("Result: {}", result);
    }
}
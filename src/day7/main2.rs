use std::collections::HashSet;

pub fn try_settings(program: &str) -> i32 {
    //Parse program
    let mut memory = parse(program);

    let mut max = 0;
    for i1 in 5..=9 {
        for i2 in 5..=9{
            for i3 in 5..=9 {
                for i4 in 5..=9{
                    for i5 in 5..=9{
                        //Check if input is unique
                        let mut set = HashSet::new();
                        set.insert(i1);
                        set.insert(i2);
                        set.insert(i3);
                        set.insert(i4);
                        set.insert(i5);
                        if set.len() != 5 {
                            continue;
                        }

                        //Get memory
                        let mut memory1 = memory.clone();
                        let mut pc1 = 0;
                        run(&mut memory1, &mut pc1, vec!(i1));
                        let mut memory2 = memory.clone();
                        let mut pc2 = 0;
                        run(&mut memory2, &mut pc2, vec!(i2));
                        let mut memory3 = memory.clone();
                        let mut pc3 = 0;
                        run(&mut memory3, &mut pc3, vec!(i3));
                        let mut memory4 = memory.clone();
                        let mut pc4 = 0;
                        run(&mut memory4, &mut pc4, vec!(i4));
                        let mut memory5 = memory.clone();
                        let mut pc5 = 0;
                        run(&mut memory5, &mut pc5, vec!(i5));

                        //Run output
                        let mut max_signal = 0;
                        let mut prev_output = 0;
                        loop {
                            let (output1, halt1) = run(&mut memory1, &mut pc1, vec!(prev_output));
                            let (output2, halt2) = run(&mut memory2, &mut pc2, vec!(output1[0]));
                            let (output3, halt3) = run(&mut memory3, &mut pc3, vec!(output2[0]));
                            let (output4, halt4) = run(&mut memory4, &mut pc4, vec!(output3[0]));
                            let (output5, halt5) = run(&mut memory5, &mut pc5, vec!(output4[0]));

                                prev_output = output5[0];
                                max_signal = max_signal.max(output5[0]);
                                if halt1 || halt2 || halt3 || halt4 || halt5 {
                                    break;
                                }
                        }
                        max = max.max(max_signal);
                    }
                }
            }
        }
    }
    return max;
}

pub fn parse(program: &str) -> Vec<i32> {
    //Put all numbers in vector
    let mut memory: Vec<i32> = Vec::new();
    for st in program.split(',') {
        let num: i32 = st.parse::<i32>().expect("Didn't get a number!");
        memory.push(num);
    }
    return memory;
}

pub fn run(memory: &mut Vec<i32>, program_counter: &mut usize, inputs: Vec<i32>) -> (Vec<i32>,
                                                                                     bool) {
    //Outputs so far
    let mut outputs = vec!();
    let mut input_current = 0;

    //Program infinite loop
    loop {
        //Get the current instruction
        assert!(memory[*program_counter] >= 0);
        let instr = memory[*program_counter] as usize;

        //Match current instruction
        match get_instr(instr) {
            //Add
            //[2] = [0] + [1]
            1 => {
                //Arguments
                let param_a = input_param(&memory, instr, *program_counter, 0);
                let param_b = input_param(&memory, instr, *program_counter, 1);
                let output_index = output_param(&memory, instr, *program_counter, 2);

                //Execute
                memory[output_index] = param_a + param_b;
                *program_counter += 4;
            }
            //Mul
            //[2] = [0] + [1]
            2 => {
                //Arguments
                let param_a = input_param(&memory, instr, *program_counter, 0);
                let param_b = input_param(&memory, instr, *program_counter, 1);
                let output_index = output_param(&memory, instr, *program_counter, 2);

                //Execute
                memory[output_index] = param_a * param_b;
                *program_counter += 4;
            }
            //Get input
            //[0] = [input]
            3 => {
                //Arguments
                let output_index = output_param(&memory, instr, *program_counter, 0);

                //Get next input
                if input_current == inputs.len() {
                    return (outputs, false);
                }

                //Execute
                memory[output_index] = inputs[input_current];
                input_current+=1;
                *program_counter += 2;
            }
            //Output
            //[output] = [0]
            4 => {
                //Arguments
                let param_a = input_param(&memory, instr, *program_counter, 0);

                //Execute
                outputs.push(param_a);
                *program_counter += 2;
            }
            //Jump if true
            //If [0] != 0, jump to [1]
            5 => {
                //Arguments
                let param_a = input_param(&memory, instr, *program_counter, 0);
                let param_b = input_param(&memory, instr, *program_counter, 1);

                //Execute
                if param_a != 0 {
                    assert!(param_b >= 0);
                    *program_counter = param_b as usize;
                }else{
                    *program_counter += 3;
                }
            }
            //Jump if zero
            //If [0] == 0, jump to [1]
            6 => {
                //Arguments
                let param_a = input_param(&memory, instr, *program_counter, 0);
                let param_b = input_param(&memory, instr, *program_counter, 1);

                //Execute
                if param_a == 0 {
                    assert!(param_b >= 0);
                    *program_counter = param_b as usize;
                }else{
                    *program_counter += 3;
                }
            }
            //Less than
            //[2] = 1 if [0] < [1], otherwise 0
            7 => {
                //Arguments
                let param_a = input_param(&memory, instr, *program_counter, 0);
                let param_b = input_param(&memory, instr, *program_counter, 1);
                let output_index = output_param(&memory, instr, *program_counter, 2);

                //Execute
                if param_a < param_b {
                    memory[output_index] = 1;
                }else{
                    memory[output_index] = 0;
                }
               * program_counter += 4;
            }
            //Equals
            //[2] = 1 if [0] == [1], otherwise 0
            8 => {
                //Arguments
                let param_a = input_param(&memory, instr, *program_counter, 0);
                let param_b = input_param(&memory, instr, *program_counter, 1);
                let output_index = output_param(&memory, instr, *program_counter, 2);

                //Execute
                if param_a == param_b {
                    memory[output_index] = 1;
                }else{
                    memory[output_index] = 0;
                }
                *program_counter += 4;
            }
            //Halt
            99 => {
                //Execute
                return (outputs, true);
            }
            _ => {
                panic!("Invalid op-code.");
            }
        }
    }
}

pub fn get_instr(instr: usize) -> usize {
    let digits = number_to_vec(instr);
    return 10*digits[digits.len() - 2] + digits[digits.len() - 1];
}

pub fn input_param(memory: &Vec<i32>, instr: usize, instrindex: usize, paramnum: usize) -> i32 {
    let digits = number_to_vec(instr);
    let input_type = digits[2-paramnum as usize];

    match input_type {
        0 => {
            return memory[memory[instrindex as usize + paramnum as usize + 1] as usize];
        }
        1 => {
            return memory[instrindex as usize + paramnum as usize + 1];
        }
        _ => {
            panic!("Invalid position mode.")
        }
    }
}

pub fn output_param(memory: &Vec<i32>, instr: usize, instrindex: usize, paramnum: usize) -> usize {
    let digits = number_to_vec(instr);
    let input_type = digits[2-paramnum as usize];

    match input_type {
        0 => {
            let loc = memory[instrindex as usize + paramnum as usize + 1];
            assert!(loc >= 0);
            return loc as usize;
        }
        1 => {
            panic!("Found immediate output location.")
        }
        _ => {
            panic!("Invalid position mode.")
        }
    }
}

fn number_to_vec(n: usize) -> Vec<usize> {
    let mut digits = Vec::new();
    let mut n = n;

    //Add digits
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);

    //Push 0
    while digits.len() < 5 {
        digits.push(0);
    }

    digits.reverse();
    digits
}
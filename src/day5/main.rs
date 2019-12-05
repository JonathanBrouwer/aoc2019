pub fn main(program: &str, inputs: Vec<i32>) -> Vec<i32> {
    //Put all numbers in vector
    let mut memory: Vec<i32> = Vec::new();
    for st in program.split(',') {
        let num: i32 = st.parse::<i32>().expect("Didn't get a number!");
        memory.push(num);
    }

    //Run program
    return run(&mut memory, inputs);
}

pub fn run(memory: &mut Vec<i32>, inputs: Vec<i32>) -> Vec<i32> {
    //Keep track of program state
    let mut program_counter: usize = 0;
    let mut input_current: usize = 0;

    //Outputs so far
    let mut outputs = vec!();

    //Program infinite loop
    loop {
        //Get the current instruction
        assert!(memory[program_counter] >= 0);
        let instr = memory[program_counter] as usize;

        //Match current instruction
        match get_instr(instr) {
            //Add
            //[2] = [0] + [1]
            1 => {
                //Arguments
                let param_a = input_param(&memory, instr, program_counter, 0);
                let param_b = input_param(&memory, instr, program_counter, 1);
                let output_index = output_param(&memory, instr, program_counter, 2);

                //Execute
                memory[output_index] = param_a + param_b;
                program_counter += 4;
            }
            //Mul
            //[2] = [0] + [1]
            2 => {
                //Arguments
                let param_a = input_param(&memory, instr, program_counter, 0);
                let param_b = input_param(&memory, instr, program_counter, 1);
                let output_index = output_param(&memory, instr, program_counter, 2);

                //Execute
                memory[output_index] = param_a * param_b;
                program_counter += 4;
            }
            //Get input
            //[0] = [input]
            3 => {
                //Arguments
                let output_index = output_param(&memory, instr, program_counter, 0);

                //Get next input
                let input = inputs[input_current];
                input_current += 1;

                //Execute
                memory[output_index] = input;
                program_counter += 2;
            }
            //Output
            //[output] = [0]
            4 => {
                //Arguments
                let param_a = input_param(&memory, instr, program_counter, 0);

                //Execute
                outputs.push(param_a);
                program_counter += 2;
            }
            //Jump if true
            //If [0] != 0, jump to [1]
            5 => {
                //Arguments
                let param_a = input_param(&memory, instr, program_counter, 0);
                let param_b = input_param(&memory, instr, program_counter, 1);

                //Execute
                if param_a != 0 {
                    assert!(param_b >= 0);
                    program_counter = param_b as usize;
                }else{
                    program_counter += 3;
                }
            }
            //Jump if zero
            //If [0] == 0, jump to [1]
            6 => {
                //Arguments
                let param_a = input_param(&memory, instr, program_counter, 0);
                let param_b = input_param(&memory, instr, program_counter, 1);

                //Execute
                if param_a == 0 {
                    assert!(param_b >= 0);
                    program_counter = param_b as usize;
                }else{
                    program_counter += 3;
                }
            }
            //Less than
            //[2] = 1 if [0] < [1], otherwise 0
            7 => {
                //Arguments
                let param_a = input_param(&memory, instr, program_counter, 0);
                let param_b = input_param(&memory, instr, program_counter, 1);
                let output_index = output_param(&memory, instr, program_counter, 2);

                //Execute
                if param_a < param_b {
                    memory[output_index] = 1;
                }else{
                    memory[output_index] = 0;
                }
                program_counter += 4;
            }
            //Equals
            //[2] = 1 if [0] == [1], otherwise 0
            8 => {
                //Arguments
                let param_a = input_param(&memory, instr, program_counter, 0);
                let param_b = input_param(&memory, instr, program_counter, 1);
                let output_index = output_param(&memory, instr, program_counter, 2);

                //Execute
                if param_a == param_b {
                    memory[output_index] = 1;
                }else{
                    memory[output_index] = 0;
                }
                program_counter += 4;
            }
            //Halt
            99 => {
                //Execute
                return outputs;
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
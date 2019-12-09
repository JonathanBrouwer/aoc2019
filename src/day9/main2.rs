use std::collections::HashSet;

pub fn parse_run(program: &str, inputs: Vec<i64>) -> (Vec<i64>, bool) {
    let mut memory = parse(program);
    let mut program = ProgramState {memory: &mut memory, pc: 0, relativebase: 0};
    return run(&mut program, inputs);
}

pub fn parse(program: &str) -> Vec<i64> {
    //Put all numbers in vector
    let mut memory: Vec<i64> = Vec::new();
    for st in program.split(',') {
        let num: i64 = st.parse::<i64>().expect("Didn't get a number!");
        memory.push(num);
    }
    return memory;
}

pub struct ProgramState<'a> {
    memory: &'a mut Vec<i64>,
    pc: usize,
    relativebase: i64
}

pub fn run(pr: &mut ProgramState, inputs: Vec<i64>) -> (Vec<i64>, bool) {
    //Outputs so far
    let mut outputs = vec!();
    let mut input_current = 0;

    //Program infinite loop
    loop {
        //Get the current instruction
        assert!(pr.memory[pr.pc] >= 0);
        let instr = pr.memory[pr.pc] as usize;

        //Match current instruction
        match get_instr(instr) {
            //Add
            //[2] = [0] + [1]
            1 => {
                //Arguments
                let param_a = input_param(pr, instr, pr.pc, 0);
                let param_b = input_param(pr, instr, pr.pc, 1);
                let output_index = output_param(pr, instr, pr.pc, 2);

                //Execute
                pr.memory[output_index] = param_a + param_b;
                pr.pc += 4;
            }
            //Mul
            //[2] = [0] + [1]
            2 => {
                //Arguments
                let param_a = input_param(pr, instr, pr.pc, 0);
                let param_b = input_param(pr, instr, pr.pc, 1);
                let output_index = output_param(pr, instr, pr.pc, 2);

                //Execute
                pr.memory[output_index] = param_a * param_b;
                pr.pc += 4;
            }
            //Get input
            //[0] = [input]
            3 => {
                //Arguments
                let output_index = output_param(pr, instr, pr.pc, 0);

                //Get next input
                if input_current == inputs.len() {
                    return (outputs, false);
                }

                //Execute
                pr.memory[output_index] = inputs[input_current];
                input_current+=1;
                pr.pc += 2;
            }
            //Output
            //[output] = [0]
            4 => {
                //Arguments
                let param_a = input_param(pr, instr, pr.pc, 0);

                //Execute
                outputs.push(param_a);
                pr.pc += 2;
            }
            //Jump if true
            //If [0] != 0, jump to [1]
            5 => {
                //Arguments
                let param_a = input_param(pr, instr, pr.pc, 0);
                let param_b = input_param(pr, instr, pr.pc, 1);

                //Execute
                if param_a != 0 {
                    assert!(param_b >= 0);
                    pr.pc = param_b as usize;
                }else{
                    pr.pc += 3;
                }
            }
            //Jump if zero
            //If [0] == 0, jump to [1]
            6 => {
                //Arguments
                let param_a = input_param(pr, instr, pr.pc, 0);
                let param_b = input_param(pr, instr, pr.pc, 1);

                //Execute
                if param_a == 0 {
                    assert!(param_b >= 0);
                    pr.pc = param_b as usize;
                }else{
                    pr.pc += 3;
                }
            }
            //Less than
            //[2] = 1 if [0] < [1], otherwise 0
            7 => {
                //Arguments
                let param_a = input_param(pr, instr, pr.pc, 0);
                let param_b = input_param(pr, instr, pr.pc, 1);
                let output_index = output_param(pr, instr, pr.pc, 2);

                //Execute
                if param_a < param_b {
                    pr.memory[output_index] = 1;
                }else{
                    pr.memory[output_index] = 0;
                }
               pr.pc += 4;
            }
            //Equals
            //[2] = 1 if [0] == [1], otherwise 0
            8 => {
                //Arguments
                let param_a = input_param(pr, instr, pr.pc, 0);
                let param_b = input_param(pr, instr, pr.pc, 1);
                let output_index = output_param(pr, instr, pr.pc, 2);

                //Execute
                if param_a == param_b {
                    pr.memory[output_index] = 1;
                }else{
                    pr.memory[output_index] = 0;
                }
                pr.pc += 4;
            }
            //Change relative offset
            9 => {
                //Arguments
                let change = input_param(pr, instr, pr.pc, 0);

                //Execute
                pr.relativebase += change;
                pr.pc += 2;
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

pub fn input_param(pr: &mut ProgramState, instr: usize, instrindex: usize,
                   paramnum: usize)
                   -> i64 {
    let digits = number_to_vec(instr);
    let input_type = digits[2-paramnum];

    match input_type {
        0 => {
            let index = pr.memory[instrindex + paramnum + 1] as usize;
            ensure_size(pr, index);
            return pr.memory[index];
        }
        1 => {
            let index = instrindex + paramnum + 1;
            ensure_size(pr, index);
            return pr.memory[index];
        }
        2 => {
            let index = (instrindex as i64 + paramnum as i64 + 1) as usize;
            ensure_size(pr, index);
            let param = (pr.memory[index] + pr.relativebase) as usize;
            ensure_size(pr, param);
            return pr.memory[param];
        }
        _ => {
            panic!("Invalid position mode.")
        }
    }
}

pub fn output_param(pr: &mut ProgramState, instr: usize, instrindex: usize, paramnum: usize) ->
                                                                                           usize {
    let digits = number_to_vec(instr);
    let input_type = digits[2-paramnum as usize];

    match input_type {
        0 => {
            let index = instrindex as usize + paramnum as usize + 1;
            ensure_size(pr, index);
            let loc = pr.memory[index];
            assert!(loc >= 0);
            ensure_size(pr, loc as usize);
            return loc as usize;
        }
        1 => {
            panic!("Found immediate output location.")
        }
        2 => {
            let index = (instrindex as i64 + paramnum as i64 + 1) as usize;
            ensure_size(pr, index);
            let loc = pr.memory[index] + pr.relativebase;
            assert!(loc >= 0);
            ensure_size(pr, loc as usize);
            return loc as usize;
        }
        _ => {
            panic!("Invalid position mode.")
        }
    }
}

fn ensure_size(program: &mut ProgramState, index: usize) {
    if(program.memory.len() <= index) {
        let newsize = (index+1).max(program.memory.len() * 2);
        program.memory.resize(newsize, 0);
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
use std::collections::{HashSet, HashMap};
use crate::day11::main1::ParamMode::{READ,WRITE};
use crate::day11::main1::Direction::{N,E,S,W};

enum Direction {N, E, S, W}

pub fn paint_robot(program: &str) -> i64 {
    let mut state = ProgramState {memory: &mut parse(program), pc: 0, relativebase: 0};
    let mut map: HashMap<(i64, i64), bool> = HashMap::new();

    let mut curpos: (i64, i64) = (0, 0);
    map.insert(curpos, true); //Start on white
    let mut curdir = N;
    loop {
        let curtile = map.get(&curpos).or(Option::from(&false)).unwrap();
        let curtile_int = if *curtile {1} else {0};
        let (outputs, halt) = run(&mut state, vec!(curtile_int));

        map.insert(curpos, match outputs[0] {
            0 => false,
            1 => true,
            _ => panic!("Invalid color!")
        });

        curdir = match outputs[1] {
            0 => match curdir {
                N => W,
                W => S,
                S => E,
                E => N
            }
            1 => match curdir {
                N => E,
                E => S,
                S => W,
                W => N
            }
            _ => panic!("Invalid direction!")
        };
        curpos = match curdir {
            N => (curpos.0, curpos.1 + 1),
            E => (curpos.0 + 1, curpos.1),
            S => (curpos.0, curpos.1 - 1),
            W => (curpos.0 - 1, curpos.1)
        };

        if halt {
            break;
        }
    }

    let minx = map.keys().map(|k| k.0).min().unwrap();
    let maxx = map.keys().map(|k| k.0).max().unwrap();
    let miny = map.keys().map(|k| k.1).min().unwrap();
    let maxy = map.keys().map(|k| k.1).max().unwrap();
    for y in (miny..=maxy).rev() {
        for x in minx..=maxx {
            if map.contains_key(&(x, y)) {
                if *map.get(&(x, y)).unwrap() {
                    print!("#");
                }else{
                    print!(" ");
                }
            }else{
                print!(" ");
            }
        }
        println!();
    }
    return map.len() as i64;
}

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
                let param_a = param(pr, 0, READ);
                let param_b = param(pr, 1, READ);
                let output_index = param(pr, 2, WRITE) as usize;

                //Execute
                pr.memory[output_index] = param_a + param_b;
                pr.pc += 4;
            }
            //Mul
            //[2] = [0] + [1]
            2 => {
                //Arguments
                let param_a = param(pr, 0, READ);
                let param_b = param(pr, 1, READ);
                let output_index = param(pr, 2, WRITE) as usize;

                //Execute
                pr.memory[output_index] = param_a * param_b;
                pr.pc += 4;
            }
            //Get input
            //[0] = [input]
            3 => {
                //Arguments
                let output_index = param(pr, 0, WRITE) as usize;

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
                let param_a = param(pr, 0, READ);

                //Execute
                outputs.push(param_a);
                pr.pc += 2;
            }
            //Jump if true
            //If [0] != 0, jump to [1]
            5 => {
                //Arguments
                let param_a = param(pr, 0, READ);
                let param_b = param(pr, 1, READ);

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
                let param_a = param(pr, 0, READ);
                let param_b = param(pr, 1, READ);

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
                let param_a = param(pr, 0, READ);
                let param_b = param(pr, 1, READ);
                let output_index = param(pr, 2, WRITE) as usize;

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
                let param_a = param(pr, 0, READ);
                let param_b = param(pr, 1, READ);
                let output_index = param(pr, 2, WRITE) as usize;

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
                let change = param(pr, 0, READ);

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

pub enum ParamMode {
    READ, WRITE
}

pub fn param(pr: &mut ProgramState, paramnum: usize, mode: ParamMode) -> i64 {
    //Read instruction
    let instr = pr.memory[pr.pc];
    assert!(instr >= 0);
    let digits = number_to_vec(instr as usize);
    let input_type = digits[2-paramnum];

    //Obtain parameter location
    let param_loc = pr.pc + paramnum + 1;
    ensure_size(pr, param_loc);
    let final_param_loc: usize;
    match input_type {
        0 => { //POSITION
            final_param_loc = pr.memory[param_loc] as usize;
        }
        1 => { //IMMEDIATE
            final_param_loc = param_loc;
        }
        2 => { //RELATIVE
            final_param_loc = (pr.memory[param_loc] + pr.relativebase as i64) as usize;
        }
        _ => {
            panic!("Invalid position mode.")
        }
    }

    //Return correct value based on read/write
    return match mode {
        READ => {
            ensure_size(pr, final_param_loc);
            pr.memory[final_param_loc]
        }
        WRITE => {
            ensure_size(pr, final_param_loc);
            final_param_loc as i64
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
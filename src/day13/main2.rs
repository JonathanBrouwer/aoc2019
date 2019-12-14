use std::collections::{HashSet, HashMap};
use crate::day13::main2::ParamMode::{READ, WRITE};
use std::cmp::Ordering::Less;
use std::cmp::Ordering;

pub fn play(program: &str) -> i64 {
    //Run program
    let mut memory = parse(program);
    memory[0] = 2;
    let mut program = ProgramState { memory, pc: 0, relativebase: 0 };

    //Ball state
    let mut state = GameState {
        ball_pos: (0, 0),
        paddle_pos: (0, 0),
        score: 0,
    };

    //Initial board
    let (output, _halt) = run(&mut program, vec!());
    let mut tiles: HashMap<(i64, i64), i64> = HashMap::new();
    handle_output(&mut tiles, &mut state, &output);
    print_board(&tiles);

    //Play
    loop {
        let input = match state.ball_pos.0.cmp(&state.paddle_pos.0) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1
        };
        let (output, halt) = run(&mut program, vec!(input));
        handle_output(&mut tiles, &mut state, &output);
        //print_board(&tiles);
        if halt { return state.score; };
    }


    return 0;
}

#[derive(Clone)]
pub struct GameState {
    ball_pos: (i64, i64),
    paddle_pos: (i64, i64),
    score: i64,
}

pub fn handle_output(tiles: &mut HashMap<(i64, i64), i64>, state: &mut GameState, output: &Vec<i64>) {
    for i in (0..output.len()).step_by(3) {
        let x = output[i];
        let y = output[i + 1];
        let val = output[i + 2];

        if x == -1 && y == 0 {
            state.score = val;
        } else {
            tiles.insert((x, y), val);
            if output[i + 2] == 4 {
                state.ball_pos = (x, y);
            } else if output[i + 2] == 3 {
                state.paddle_pos = (x, y);
            }
        }
    }
}

pub fn print_board(tiles: &HashMap<(i64, i64), i64>) {
    //Print board
    for y in 0..=tiles.keys().map(|p| p.1).max().unwrap() {
        for x in 0..=tiles.keys().map(|p| p.0).max().unwrap() {
            if tiles.contains_key(&(x, y)) {
                print!("{}", match tiles.get(&(x, y)).unwrap() {
                    0 => " ",
                    1 => "#",
                    2 => ".",
                    3 => "=",
                    4 => "B",
                    _ => panic!()
                });
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

pub fn parse_run(program: &str, inputs: Vec<i64>) -> (Vec<i64>, bool) {
    let mut memory = parse(program);
    let mut program = ProgramState { memory, pc: 0, relativebase: 0 };
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

#[derive(Clone)]
pub struct ProgramState {
    memory: Vec<i64>,
    pc: usize,
    relativebase: i64,
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
                input_current += 1;
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
                } else {
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
                } else {
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
                } else {
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
                } else {
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
    return 10 * digits[digits.len() - 2] + digits[digits.len() - 1];
}

pub enum ParamMode {
    READ,
    WRITE,
}

pub fn param(pr: &mut ProgramState, paramnum: usize, mode: ParamMode) -> i64 {
    //Read instruction
    let instr = pr.memory[pr.pc];
    assert!(instr >= 0);
    let digits = number_to_vec(instr as usize);
    let input_type = digits[2 - paramnum];

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
    };
}

fn ensure_size(program: &mut ProgramState, index: usize) {
    if (program.memory.len() <= index) {
        let newsize = (index + 1).max(program.memory.len() * 2);
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
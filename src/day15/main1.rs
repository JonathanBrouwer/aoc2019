use std::collections::{HashSet, HashMap, VecDeque};
use crate::day15::main1::ParamMode::{READ,WRITE};
use crate::day15::main1::Direction::{N,E,S,W};

pub enum Direction {
    N, E, S, W
}

pub fn play(program: &str) -> i64 {
    //Run program
    let mut state = ProgramState {memory: parse(program), pc: 0, relativebase: 0};

    return explore(state, (0, 0));
}

pub fn explore(state: ProgramState, loc: (i64, i64)) -> i64 {
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    let mut have_been: HashSet<(i64, i64)> = HashSet::new();
    let mut type_map: HashMap<(i64, i64), i64> = HashMap::new();
    let mut program_map: HashMap<(i64, i64), ProgramState> = HashMap::new();
    let mut distance_map: HashMap<(i64, i64), i64> = HashMap::new();

    //Insert loc
    queue.push_front(loc.clone());
    have_been.insert(loc.clone());
    type_map.insert(loc.clone(), 1);
    program_map.insert(loc.clone(), state);
    distance_map.insert(loc.clone(), 0);

    while queue.len() > 0 {
        let loc: (i64, i64) = queue.pop_back().unwrap();

        have_been.insert(loc.clone());

        //Go through neighbours
        let neighbours = neighbours(loc);
        for (loc_nb, dir) in neighbours {
            //Track have_been
            if have_been.contains(&loc_nb) { continue }
            have_been.insert(loc_nb.clone());

            //Find relevant information from direction
            let dir_go = match dir {
                N => 1,
                S => 2,
                E => 4,
                W => 3
            };

            //Get old state
            let past_state = program_map.get(&loc).unwrap();
            let past_distance = *distance_map.get(&loc).unwrap();

            //Executes
            let mut state_clone = past_state.clone();
            let output = run(&mut state_clone, vec!(dir_go));
            assert_eq!(output.1, false);

            //Update maps
            type_map.insert(loc_nb.clone(), output.0[0]);
            program_map.insert(loc_nb.clone(), state_clone);
            distance_map.insert(loc_nb.clone(), past_distance + 1);

            //Update maps based on output
            match output.0[0] {
                0 => {
                    //Found wall, just do nothing
                    continue
                }
                1 => {
                    //Found path, repeat
                    queue.push_front(loc_nb.clone());
                }
                2 => {
                    //Target found!
                    return past_distance + 1;
                }
                _ => panic!()
            }
        }
    }

    panic!("Found nothing!");
}

pub fn neighbours(from: (i64, i64)) -> Vec<((i64, i64), Direction)> {
    let mut rtrn : Vec<((i64, i64), Direction)> = Vec::new();
    rtrn.push(((from.0, from.1 + 1), N));
    rtrn.push(((from.0, from.1 - 1), S));
    rtrn.push(((from.0 + 1, from.1), E));
    rtrn.push(((from.0 - 1, from.1), W));
    return rtrn;
}

pub fn render_map(have_been: &HashSet<(i64, i64)>, type_map: &HashMap<(i64, i64), i64>) {
    let minx = have_been.iter().map(|p| p.0).min().unwrap();
    let maxx = have_been.iter().map(|p| p.0).max().unwrap();
    let miny = have_been.iter().map(|p| p.1).min().unwrap();
    let maxy = have_been.iter().map(|p| p.1).max().unwrap();

    for y in miny ..= maxy {
        for x in minx ..= maxx {
            if have_been.contains(&(x, y)) {
                match type_map.get(&(x, y)).unwrap() {
                    0 => print!("#"),
                    1 => print!("."),
                    2 => print!("T"),
                    _ => panic!("Invalid typemap")
                }
            }else{
                print!(" ");
            }
        }
        println!();
    }
}

pub fn parse_run(program: &str, inputs: Vec<i64>) -> (Vec<i64>, bool) {
    let mut memory = parse(program);
    let mut program = ProgramState {memory, pc: 0, relativebase: 0};
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
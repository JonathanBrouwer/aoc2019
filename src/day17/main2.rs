use crate::day17::main2::ParamMode::{READ,WRITE};
use crate::day17::main2::Rotation::{N,E,S,W};
use std::slice::Iter;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Rotation {
    N, E, S, W
}

pub fn main(program: &str) -> i64 {
    //Get input as string
    let mut state = ProgramState { memory: parse(program), pc: 0, relativebase: 0 };
    let output = run(&mut state, vec!());

    //Get input as char array
    let parsed: Vec<char> = output.0.iter()
        .map( |i| *i as u8 as char).collect();
    for ch in &parsed {
        print!("{}", ch);
    }
    return run_map(&parsed);
}

pub fn run_map(parsed: &Vec<char>) -> i64 {
    let actions = find_path(parsed);

    let mut main_program = Vec::new();
    let mut routines: Vec<Vec<&str>> = vec!(Vec::new(),Vec::new(),Vec::new());
    find_subroutines(actions.as_slice(), &mut main_program, &mut routines, 0);

    return 0;
}

pub fn yeet(list: &mut Vec<i64>) {
    list.push(0);
    if list.len() < 100 {yeet(list)};
    list.remove(list.len() - 1);
}

pub fn find_subroutines<'a>(path: &[&'a str], main_program: &mut Vec<usize>, routines: &mut Vec<Vec<&'a str>>, cr: usize) -> bool {
    //Is this valid?
    if cr > 3 {
        return false;
    }
    if routines[cr].iter().map(|s| (s.len() + 1)).sum::<usize>() - 1 <= 20 {
        return false;
    }
    if main_program.iter().map(|s| (s.to_string().len() + 1)).sum::<usize>() - 1 <= 20 {
        return false;
    }

    //Are we finished?
    if path.len() == 0 {
        return true;
    }

    let croutine = &mut routines[cr];

    //Not finished :(
    //Options:
    //- We end the current here
    //- We continue the current
    //- Use a previous option from already_found

    //Can we use a previous option?
    if routines[cr].len() == 0 {
        'options: for option in routines.clone() {
            //Enumerate through option
            let mut path_index = 0;
            for (i, thing) in option.iter().enumerate() {
                //Is thing numeric?
                let num_opt = thing.parse::<i64>();
                if num_opt.is_ok() {
                    //Expect num 1s
                    let num = num_opt.unwrap();
                    for _ in 0..num {
                        if path[path_index] != "1" {
                            continue 'options;
                        }
                        path_index += 1;
                    }
                }else{
                    //Not a number, must be equal
                    if path[path_index] != *thing {
                        continue 'options;
                    }
                    path_index += 1;
                }
            }

            //It matched!
            let (first, rest) = path.split_at(path_index);
            if find_subroutines(rest, main_program, routines, cr) { return true }
        }
    }

    //The rest is char-by-char, split off the first char
    let (first, rest) = path.split_first().unwrap();

    //Can we end it here?
    if routines.len() < 3 {
        main_program.push(routines.len());
        if find_subroutines(rest, main_program, routines, cr + 1) { return true }
        routines.remove(routines.len() - 1);
    }

    //Can we add it to the current?
    //Just try, the valid check will catch us if it's too long
    {
        let num_opt = routines[cr].last().unwrap().parse::<i64>();
        if num_opt.is_ok() {
            let old_num = num_opt.unwrap();
            let lastpos = routines[cr].len() - 1;
            routines[cr][lastpos - 1] = (old_num+1).to_string().as_ref();
            if find_subroutines(rest, main_program, routines, cr) { return true }
            //TODO routines[cr][lastpos - 1] = old_num.to_string().as_ref();
        }else{
            routines[cr].push("1");
            if find_subroutines(rest, main_program, routines, cr) { return true }
            routines[cr].remove(routines[cr].len() - 1);
        }
    }

    //Nothing worked
    return false;
}

pub fn find_path(parsed: &Vec<char>) -> Vec<&str> {
    //Get input as 2d array
    let mut map: HashSet<(i64, i64)> = HashSet::new();
    let mut current = (0, 0);
    let mut current_rot = N;

    let mut pos = (0, 0);
    for ch in parsed {
        if *ch == '\n' {
            pos = (0, pos.1 + 1);
        }else{
            match *ch {
                '#' => { map.insert(pos.clone()); }
                '^' | 'v' | '<' | '>' => { map.insert(pos.clone()); current = pos.clone(); }
                _ => {}
            }
            pos = (pos.0 + 1, pos.1);
        }
    }

    //For all points that have 4 neighbours
    let mut actions: Vec<&str> = Vec::new();
    while map.len() > 1 {
        //Find amount of points around current point
        let neighbours = neighbours(&map, &current);

        //Find target position
        let target ;
        match neighbours.len() {
            1 => {
                target = neighbours.iter().next().unwrap();
            }
            2 => {
                target = neighbours.iter().filter(|p| p.1 == current_rot).next().unwrap();
            }
            3 => {
                let directions: Vec<Rotation> = neighbours.iter().map(|p| p.1).collect();
                let correct_dir = if !directions.contains(&N) {
                    S
                }else if !directions.contains(&E) {
                    W
                }else if !directions.contains(&S) {
                    N
                }else if !directions.contains(&W) {
                    E
                } else {
                    panic!()
                };
                target = neighbours.iter().filter(|p| p.1 == correct_dir).next().unwrap();
            }
            _ => panic!()
        }

        //Go to target
        //Find rotation needed
        match (current_rot, target.1) {
            (N, N) | (E, E) | (S, S) | (W, W)
            => {}
            (N, E) | (E, S) | (S, W) | (W, N)
            => actions.push("R"),
            (N, W) | (W, S) | (S, E) | (E, N)
            => actions.push("L"),
            (N, S) | (E, W) | (S, N) | (W, E)
            => panic!()
        }

        //If this is the last time we'll come here, remove the tile
        if neighbours.len() <= 2 {map.remove(&current);}

        //Move forward
        actions.push("1");
        current = target.0.clone();
        current_rot = target.1;
    }

    return actions;
}

pub fn neighbours(map: &HashSet<(i64, i64)>, pos: &(i64, i64)) -> HashSet<((i64, i64), Rotation)> {
    let mut neighbours: HashSet<((i64, i64), Rotation)> = HashSet::new();
    if map.contains(&(pos.0 + 1, pos.1)) {
        neighbours.insert(((pos.0 + 1, pos.1), E));
    }
    if map.contains(&(pos.0 - 1, pos.1)) {
        neighbours.insert(((pos.0 - 1, pos.1), W));
    }
    if map.contains(&(pos.0, pos.1 + 1)) {
        neighbours.insert(((pos.0, pos.1 + 1), S));
    }
    if map.contains(&(pos.0, pos.1 - 1)) {
        neighbours.insert(((pos.0, pos.1 - 1), N));
    }
    return neighbours;
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
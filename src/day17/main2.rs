use crate::day17::main2::ParamMode::{READ,WRITE};
use crate::day17::main2::Rotation::{N,E,S,W};
use std::slice::Iter;
use std::collections::HashSet;
use crate::day17::main2::Step::{R, L, F};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Rotation {
    N, E, S, W
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Step {
    L, R,
    F(u64)
}

pub trait Len {
    fn len(&self) -> usize;
}

impl Len for Step {
    fn len(&self) -> usize {
        match self {
            L | R => 1,
            F(n) => n.to_string().len()
        }
    }
}

impl ToString for Step {
    fn to_string(&self) -> String {
        match self {
            L => String::from("L"),
            R => String::from("R"),
            F(n) => n.to_string()
        }.parse().unwrap()
    }
}

const MAXLENGTH: usize = 21;
pub fn main_str(main: &Vec<usize>) -> Vec<char> {
    let strings = main.iter().map(|c| ((*c as u8 + 'A' as u8) as char).to_string()).collect::<Vec<String>>();
    let result = strings.join(",") + "\n";
    return result.chars().collect();
}

pub fn routine_str(routine: &Vec<Step>) -> Vec<char> {
    let mut strings = routine.iter().map(|step| step.to_string()).collect::<Vec<String>>();
    let result = strings.join(",") + "\n";
    return result.chars().collect();
}

pub fn main(program: &str) -> i64 {
    //Get input as string
    let mut memory = parse(program);
    memory[0] = 2;
    let mut state = ProgramState { memory, pc: 0, relativebase: 0 };
    let output = run(&mut state, vec!());

    //Get input as char array
    let parsed: Vec<char> = output.0.iter()
        .map( |i| *i as u8 as char).collect();
    for ch in &parsed {
        print!("{}", ch);
    }

    //Get answers
    let (routines, main) = find_map(&parsed);
    let main_str: Vec<char> = main_str(&main);
    let routines_str: Vec<Vec<char>> = routines.iter().map(|r| routine_str(r)).collect();
    assert!(main_str.len() <= 21);
    assert!(routines_str[0].len() <= 21);
    assert!(routines_str[1].len() <= 21);
    assert!(routines_str[2].len() <= 21);

    //Feed into robot
    let (o1, h1) = run(&mut state, main_str.iter().map(|c| *c as u8 as i64).collect::<Vec<i64>>());
    assert_eq!(h1, false);
    for i in o1 {
        print!("{}", i as u8 as char);
    }
    println!();

    for i in 0..3 {
        let (o2, h2) = run(&mut state, routines_str[i].iter().map(|c| *c as u8 as i64).collect::<Vec<i64>>());
        assert_eq!(h2, false);
        for i in o2 {
            print!("{}", i as u8 as char);
        }
        println!();
    }
    let (o5, h5) = run(&mut state, vec!('n' as u8 as i64, '\n' as u8 as i64));
    assert_eq!(h5, true);
    for i in &o5 {
        print!("{}", *i as u8 as char);
    }
    println!();

    return o5[0];
}

pub fn find_map(parsed: &Vec<char>) -> (Vec<Vec<Step>>, Vec<usize>) {
    let actions = find_path(parsed);

    let mut main_program = Vec::new();
    let mut routines: Vec<Vec<Step>> = vec!(Vec::new(),Vec::new(),Vec::new());
    let found = find_subroutines(actions.as_slice(), &mut main_program, &mut routines, 0);
    assert!(found);

    println!("M: {:?}", main_program);
    println!("0: {:?}", routines[0]);
    println!("1: {:?}", routines[1]);
    println!("2: {:?}", routines[2]);

    return (routines, main_program);
}

pub fn find_subroutines<'a>(path: &[Step], main_program: &mut Vec<usize>, routines: &mut Vec<Vec<Step>>, cr: usize) -> bool {
    //Is this valid?
    if cr > 3 {
        return false;
    }

    //Are the routines short enough?
    // + 1 at the end is to account for the extra comma counted at the end
    if main_str(main_program).len() > MAXLENGTH {
        return false;
    }
    for routine in routines.iter() {
        if routine_str(routine).len() > MAXLENGTH {
            return false;
        }
    }

    //Are we finished?
    if cr == 3 && path.len() == 0 {
        return true;
    }

    //Not finished :(
    //Options:
    //- We end the current here
    //- We continue the current
    //- Use a previous option from already_found

    //Can we use a previous option?
    if cr == 3 || routines[cr].len() == 0 {
        'options: for (option_index, option) in routines.clone().iter().enumerate() {
            //
            if option.len() == 0 {
                continue;
            }
            //Enumerate through option
            let mut path_index = 0;
            for thing in option.iter() {
                //Match on type
                match *thing {
                    L | R => {
                        //Is path index still valid?
                        if path_index >= path.len() {
                            continue 'options;
                        }
                        //Not a forward, the items must be equal
                        if path[path_index] != *thing {
                            continue 'options;
                        }
                        path_index += 1;
                    }
                    F(num) => {
                        //We expect num F(1)s
                        for _ in 0..num {
                            //Is path index still valid?
                            if path_index >= path.len() {
                                continue 'options;
                            }
                            //Match next F(1)
                            if path[path_index] != F(1) {
                                continue 'options;
                            }
                            path_index += 1;
                        }
                    }
                }
            }

            //It matched!
            let (_first, rest) = path.split_at(path_index);
            main_program.push(option_index);
            if find_subroutines(rest, main_program, routines, cr) { return true }
            main_program.remove(main_program.len() - 1);
        }
    }

    //Can we end the current routine here?
    if cr < 3 {
        main_program.push(cr);
        if find_subroutines(path, main_program, routines, cr + 1) { return true }
        main_program.remove(main_program.len() - 1);
    }

    //Can we add it to the current?
    //Just try, the valid check will catch us if it's too long
    if cr < 3 {
        //General things for later
        let (first, rest) = path.split_first().unwrap();

        match *first {
            L | R => {
                //Add the L or R to the routine
                routines[cr].push(*first);
                if find_subroutines(rest, main_program, routines, cr) { return true }
                let lastpos = routines[cr].len() - 1;
                routines[cr].remove(lastpos);
            }
            F(num) => {
                assert_eq!(num, 1);
                //Check if we can merge this with the last of current routine
                if !routines[cr].is_empty() {
                    let old_lastpos = routines[cr].len() - 1;
                    match *routines[cr].last().unwrap() {
                        L | R => {
                            routines[cr].push(F(num));
                            if find_subroutines(rest, main_program, routines, cr) { return true }
                            routines[cr].remove(old_lastpos + 1);
                        }
                        F(old_num) => {
                            routines[cr][old_lastpos] = F(old_num + num);
                            if find_subroutines(rest, main_program, routines, cr) { return true }
                            routines[cr][old_lastpos] = F(old_num);
                        }
                    }
                }else{
                    routines[cr].push(F(num));
                    if find_subroutines(rest, main_program, routines, cr) { return true }
                    routines[cr].remove(0);
                }
            }
        }
    }
    //Nothing worked
    return false;
}

pub fn find_path(parsed: &Vec<char>) -> Vec<Step> {
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
    let mut actions: Vec<Step> = Vec::new();
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
            => actions.push(R),
            (N, W) | (W, S) | (S, E) | (E, N)
            => actions.push(L),
            (N, S) | (E, W) | (S, N) | (W, E)
            => panic!()
        }

        //If this is the last time we'll come here, remove the tile
        if neighbours.len() <= 2 {map.remove(&current);}

        //Move forward
        actions.push(F(1));
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
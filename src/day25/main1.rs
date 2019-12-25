use std::collections::{HashSet, HashMap, VecDeque};
use crate::day25::main1::ParamMode::{READ,WRITE};
use crate::day25::main1::Input::{MOVE, TAKE, DROP, INV};
use crate::day25::main1::Direction::{NORTH, EAST, SOUTH, WEST};
use text_io::read;

pub fn main(program: &str) {
    let mut droid = Droid { program: ProgramState { memory: parse(program), pc: 0, relativebase: 0}};

    //Door = SWSS
    bfs(&mut droid);

    //Verify that robot got items
    let items = vec!("cake", "easter egg", "mutex", "space law space brochure", "manifold", "hologram", "whirled peas", "loom");
    let (inv, halt) = droid.input(INV);
    for item in items {
        assert!(inv.contains(item));
    }

    //Go to door
    droid.input(MOVE(SOUTH));
    droid.input(MOVE(WEST));
    droid.input(MOVE(SOUTH));
    droid.input(MOVE(SOUTH));

    //Find right item combination
    try_items(&mut droid);

    //Manual mode
    run(&mut droid.program, vec!());
    loop {
        let inp: String = read!("{}\n");
        let action = match inp.as_ref() {
            "n" => MOVE(NORTH),
            "e" => MOVE(EAST),
            "s" => MOVE(SOUTH),
            "w" => MOVE(WEST),
            "t" => {
                println!("Item name?");
                let item= read!("{}\n");
                TAKE(item)
            }
            "d" => {
                println!("Item name?");
                let item= read!("{}\n");
                DROP(item)
            }
            "i" => INV,
            _ => {
                println!("Invalid input.");
                continue
            }
        };
        droid.input(action);
    }
}

pub fn try_items(droid: &mut Droid) {
    let items = vec!("cake", "easter egg", "mutex", "space law space brochure", "manifold", "hologram", "whirled peas", "loom");

    //Drop all items
    for item in &items {
        droid.input(DROP(item.to_string()));
    }

    //For all subsets
    for set in powerset(&items) {
        for item in &set {
            droid.input(TAKE(item.to_string()));
        }
        let (output, halt) = droid.input(MOVE(SOUTH));
        if !output.contains("ejected") {
            println!("Found right item combination");
            return;
        }
        for item in &set {
            droid.input(DROP(item.to_string()));
        }
    }
    println!("Didn't find right combination");
}

fn powerset<'a>(s: &[&'a str]) -> Vec<Vec<&'a str>> {
    let mut subsets: Vec<Vec<&str>> = vec![];
    let empty: Vec<&str> = vec![];
    subsets.push(empty);

    let mut updated: Vec<Vec<&str>> = vec![];

    for ele in s {
        for mut sub in subsets.clone() {
            sub.push(*ele);
            updated.push(sub);
        }
        subsets.append(&mut updated);
    }
    subsets
}

pub fn bfs(droid: &mut Droid) {
    let (mut output, mut halt) = run(&mut droid.program, vec!());
    let mut output_str: String = output.iter().map( |i| *i as u8 as char).collect();
    println!("{}", output_str);

    //Do BFS
    bfs_rec(droid, &output_str, Option::None);
}

pub fn bfs_rec(droid: &mut Droid, output: &String, come_from: Option<Direction>) {
    //Exceptions: Security Checkpoint has no neighbours
    if output.contains("Security Checkpoint") { return }

    //Take items where we are
    //Excluded: infinite loop, giant electromagnet, escape pod, molten lava, photons
    let items = vec!("cake", "easter egg", "mutex", "space law space brochure", "manifold", "hologram", "whirled peas", "loom");
    for item in items {
        if output.contains(item) {
            droid.input(TAKE(item.to_string()));
        }
    }

    //Loop through neighbours
    for nb_dir in neighbours(output) {
        //Don't go back where we just came from
        if let Some(from) = come_from {
            if from.inv() == nb_dir {
                continue;
            }
        }

        //Go there, run recursive, go back
        let (output, halt) = droid.input(MOVE(nb_dir));
        assert_eq!(halt, false);
        bfs_rec(droid, &output, Some(nb_dir));
        droid.input(MOVE(nb_dir.inv()));
    }
}

pub fn neighbours(output_str: &String) -> HashSet<(Direction)> {
    let mut neighbours = HashSet::new();
    if output_str.contains("north\n") {
        neighbours.insert(NORTH);
    }
    if output_str.contains("east\n") {
        neighbours.insert(EAST);
    }
    if output_str.contains("south\n") {
        neighbours.insert(SOUTH);
    }
    if output_str.contains("west\n") {
        neighbours.insert(WEST);
    }
    return neighbours;
}

pub struct Droid {
    program: ProgramState
}

impl Droid {
    pub fn input(&mut self, input: Input) -> (String, bool) {
        let mut pinput: Vec<i64> = input.to_string().chars().map(|c| c as u8 as i64).collect();
        pinput.push('\n' as u8 as i64);
        let (output, halt) = run(&mut self.program, pinput);
        for ci in &output {
            print!("{}", *ci as u8 as char);
        }
        return (output.iter().map( |i| *i as u8 as char).collect(), halt);
    }
}

pub enum Input {
    MOVE(Direction),
    TAKE(String),
    DROP(String),
    INV
}

impl ToString for Input {
    fn to_string(&self) -> String {
        match self {
            MOVE(dir) => dir.to_string(),
            TAKE(item) => format!("take {}", item).to_string(),
            DROP(item) => format!("drop {}", item).to_string(),
            INV => String::from("inv")
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub enum Direction {
    NORTH, EAST, SOUTH, WEST
}

impl Direction {
    pub fn inv(&self) -> Direction {
        match self {
            NORTH => SOUTH,
            SOUTH => NORTH,
            EAST => WEST,
            WEST => EAST
        }
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            NORTH => "north",
            EAST => "east",
            SOUTH => "south",
            WEST => "west"
        }.to_string()
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
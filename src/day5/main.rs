pub fn main(input: &str) {
    //Put all numbers in vector
    let mut memory: Vec<i32> = Vec::new();
    for st in input.split(',') {
        let num: i32 = st.parse::<i32>().expect("Didn't get a number!");
        memory.push(num);
    }

    run(memory);
}

pub fn run(mut memory: Vec<i32>) {
    //Run program
    let mut current: i32 = 0;
    loop {
        //Get operator parameters
        let instr = memory[current as usize] as i32;

        match get_instr(instr) {
            1 => {
                let param_a = read_param(&memory, instr, current, 0);
                let param_b = read_param(&memory, instr, current, 1);
                let output_index = memory[current as usize + 3];
                memory[output_index as usize] = param_a + param_b;
                current += 4;
            }
            2 => {
                let param_a = read_param(&memory, instr, current, 0);
                let param_b = read_param(&memory, instr, current, 1);
                let output_index = memory[current as usize + 3];
                memory[output_index as usize] = param_a * param_b;
                current += 4;
            }
            3 => {
                let input = 5;
                let output_index = memory[current as usize + 1];
                memory[output_index as usize] = input;
                current += 2;
            }
            4 => {
                let param_a = read_param(&memory, instr, current, 0);
                println!("Output: {}", param_a);
                current += 2;
            }
            5 => {
                let param_a = read_param(&memory, instr, current, 0);
                let param_b = read_param(&memory, instr, current, 1);
                if param_a != 0 {
                    current = param_b;
                }else{
                    current += 3;
                }
            }
            6 => {
                let param_a = read_param(&memory, instr, current, 0);
                let param_b = read_param(&memory, instr, current, 1);
                if param_a == 0 {
                    current = param_b;
                }else{
                    current += 3;
                }
            }
            7 => {
                let param_a = read_param(&memory, instr, current, 0);
                let param_b = read_param(&memory, instr, current, 1);
                let output_index = memory[current as usize + 3];
                if param_a < param_b {
                    memory[output_index as usize] = 1;
                }else{
                    memory[output_index as usize] = 0;
                }
                current += 4;
            }
            8 => {
                let param_a = read_param(&memory, instr, current, 0);
                let param_b = read_param(&memory, instr, current, 1);
                let output_index = memory[current as usize + 3];
                if param_a == param_b {
                    memory[output_index as usize] = 1;
                }else{
                    memory[output_index as usize] = 0;
                }
                current += 4;
            }
            99 => {
                return;
            }
            _ => {
                panic!("Invalid op-code.");
            }
        }
    }
}

pub fn get_instr(instr: i32) -> i32 {
    let digits = number_to_vec(instr);
    return 10*digits[digits.len() - 2] + digits[digits.len() - 1];
}

pub fn read_param(memory: &Vec<i32>, instr: i32, instrindex: i32, paramnum: i32) -> i32 {
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

fn number_to_vec(n: i32) -> Vec<i32> {
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

#[cfg(test)]
mod test {
    use crate::day5::main::main;

    #[test]
    fn test_main_real() {
        let input = include_str!("input.txt");
        main(input);
    }

    #[test]
    fn jump_zero_pos() {
        let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        println!("Expect non-zero");
        main(input);
    }

    #[test]
    fn jump_zero_imm() {
        let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        println!("Expect non-zero");
        main(input);
    }

    #[test]
    fn big_test() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        println!("Expect 999");
        main(input);
    }
}
pub fn main(strin: &str) -> String {
    //Create first input
    let mut input: Vec<i64> = Vec::new();
    for ch in strin.chars() {
        input.push(ch.to_digit(10).unwrap() as i64);
    }

    //For each phase
    let pattern = vec!(0, 1, 0, -1);
    let mut output: Vec<i64> = Vec::new();
    for _ in 0..100 {
        //For each position
        for i in 1..=input.len() {
            let mut possum = 0;
            for (j, num) in input.iter().enumerate() {
                let mul_by = pattern[((j+1) / i) % 4];
                possum += *num * mul_by;
            }
            output.push((possum % 10).abs());
        }

        //Do input-output swap
        input = output;
        output = Vec::new();
    }

    let mut final_output = String::new();
    for i in input {
        final_output.push_str(i.to_string().as_str());
    }
    return final_output;
}

#[cfg(test)]
mod test {
    use crate::day16::main1::main;

    #[test]
    fn test_day16_part1_0() {
        let input = "12345678";
        let result = main(input);
        assert_eq!(result, "23845678");
    }

    #[test]
    fn test_day16_part1_1() {
        let input = "80871224585914546619083218645595";
        let result = main(input);
        assert_eq!(result, "24176176480919046114038763195595");
    }

    #[test]
    fn test_day16_part1_2() {
        let input = "19617804207202209144916044189917";
        let result = main(input);
        assert_eq!(result, "73745418557257259149466599639917");
    }

    #[test]
    fn test_day16_part1_3() {
        let input = "69317163492948606335995924319873";
        let result = main(input);
        assert_eq!(result, "52432133292998606880495974869873");
    }

    #[test]
    fn test_main_real() {
        let input = include_str!("input.txt");
        let result = main(input);
        println!("Result: {}", result);
        assert_eq!(result, "77038830653233361255314046818347110691571207860972826750703528036072647137275835157934865244753436827100638642075752850257221737315334180111899482275873821050397765752162740703857084466158829110765095716409457347494374275616497668507627323833234935306896546517713172097671580519699518571036198691652639192629112328924296569895346103757993804590618706801045733244530365266348359432626365639551250687317888261896589020351163534902963636024105155028396916635622607564613260090550294620751980641832094142222897182373851955141758381749266214573978629986756691594740935351826234697302504217334139643483903966326398329406895250802553412058415096652193339331");
    }
}
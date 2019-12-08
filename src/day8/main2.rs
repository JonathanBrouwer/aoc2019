pub fn main(input: &str, width: usize, height: usize) -> Vec<usize> {
    //Put all numbers in vector
    let chars: Vec<char> = input.chars().collect();
    let imgsize = width*height;

    //Images
    let mut image : Vec<usize> = Vec::new();
    for i in 0..imgsize {
        image.push(2);
    }
    for i in 0..(input.len() / imgsize) {
        //Parse digits
        for j in 0..(width*height) {
            //Pixels in image
            let pixel = chars[i*imgsize+j].to_digit(10).expect("No digit") as usize;
            if image[j] == 2 {
                image[j] = pixel;
            }
        }
    }

    return image;
}


#[cfg(test)]
mod test {
    use crate::day8::main2::main;

    #[test]
    fn test_day8_part1_1() {
        let input = "0222112222120000";
        let result = main(input, 2, 2);
        assert_eq!(result, vec!(0, 1, 1, 0));
    }

    #[test]
    fn test_main_real() {
        let input = include_str!("input.txt");
        let result = main(input, 25, 6);
        for y in 0..6 {
            for x in 0..25 {
                let vis = if result[y*25+x] == 1 {
                    '■'
                } else {
                    ' '
                };
                print!("{}", vis);
            }
            println!();
        }
        assert_eq!(result, vec!(0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0));
    }
}
pub fn main(input: &str, width: usize, height: usize) -> usize {
    //Put all numbers in vector
    let chars: Vec<char> = input.chars().collect();
    let imgsize = width*height;

    //Images
    let mut minzero = (std::usize::MAX, 0);
    for i in 0..(input.len() / imgsize) {
        let mut image: [usize; 10] = [0; 10];
        //Parse digits
        for j in 0..(width*height) {
            //Pixels in image
            let pixel = chars[i*imgsize+j].to_digit(10).expect("No digit") as usize;
            image[pixel] += 1;
        }
        //Count
        if image[0] < minzero.0 {
            minzero = (image[0], image[1] * image[2]);
        }
    }

    return minzero.1;
}


#[cfg(test)]
mod test {
    use crate::day8::main1::main;

    #[test]
    fn test_day8_part1_1() {
        let input = "123456789012";
        let result = main(input, 3, 2);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_main_real() {
        let input = include_str!("input.txt");
        let result = main(input, 25, 6);
        println!("Result: {}", result);
    }
}
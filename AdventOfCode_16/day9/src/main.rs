use std::fs;
use std::time::Instant;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let r1 = part1(&text);
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1.len(), t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(&text);
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2.len(), t.elapsed().as_millis());
}

fn part2(input : &str) -> String{
    let mut result = String::new();
    let mut pointer:usize = 0;
    let chars:Vec<char> = input.chars().collect();

    while pointer < input.len(){
        match chars[pointer] {
            '(' => {
                let (length, time, offset) = get_marker(&chars, pointer);
                pointer += offset;

                let repetir:String = part2(&input[pointer..pointer+length as usize]);

                for _ in 0..time {
                    result.push_str(&repetir);
                }

                pointer += length as usize;
            }
            c => {
                result.push(c);
                pointer += 1;
            }
        }
    }
    result
}

fn part1(input : &String) -> String{
    let mut result = String::new();
    let mut pointer:usize = 0;
    let chars:Vec<char> = input.chars().collect::<Vec<char>>();

    while pointer < input.len(){
        match chars[pointer] {
            '(' => {
                let (length, time, offset) = get_marker(&chars, pointer);
                pointer += offset;

                let repetir:String = chars[pointer..pointer+length as usize].iter().collect();
                result.extend(repetir.repeat(time as usize).chars());

                pointer += length as usize;
            }
            c => {
                result.push(c);
                pointer += 1;
            }
        }
    }
    result
}
fn get_marker(chars: &[char] , pointer : usize) -> (u16, u16, usize){
    let end_marker = chars[pointer + 1..].iter().position(|&c| c == ')').unwrap() +1;
    let marker: String = chars[pointer + 1..pointer + end_marker].iter().collect();
    let parts: Vec<u16> = marker.split('x').map(|x| x.parse().unwrap()).collect();
    (parts[0], parts[1], end_marker + 1)
}

#[cfg(test)]
mod test{
    use std::fs;
    use crate::{part1, part2};

    #[test]
    fn test_part1(){
        let text = fs::read_to_string("test.txt").unwrap();
        let tests = gentest(&text);
        for (test, expected) in tests{
            assert_eq!(part1(&test.to_string()), expected);
        }
    }

    #[test]
    fn test_part2(){
        let text = fs::read_to_string("test2.txt").unwrap();
        let tests = gentest(&text);
        for (test, expected) in tests{
            assert_eq!(part2(&test), expected);
        }
    }

    fn gentest(text : &String) -> Vec<(&str, &str)>{
        text.lines().into_iter().map(|x| {
            let separados = x.split("|").collect::<Vec<&str>>();
            (separados[0], separados[1])
        }).collect::<Vec<(&str, &str)>>()
    }

}
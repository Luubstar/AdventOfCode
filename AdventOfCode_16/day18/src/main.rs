use std::fs;
use std::time::Instant;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();
    let input = parse(&text);

    let t = Instant::now();
    let r1 = part1(input.clone(), 40);
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part1(input.clone(), 400000);
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}


fn part1(input : Vec<bool>, length: u32) -> u32{
    let mut last = input;
    let mut count = last.iter().fold(0, |acc, f| if !f {acc +1} else {acc});
    for _ in 0..length-1{
        let mut temp = last.clone();
        temp.insert(0, false);
        temp.push(false);

        let mut temp = temp.windows(3).map(|x|{
            x[0] && !x[1] && !x[2]||
            x[0] && x[1] && !x[2] ||
            !x[0] && x[1] && x[2] ||
            !x[0] && !x[1] && x[2]
        }).collect::<Vec<bool>>();

        std::mem::swap(&mut temp, &mut last);

        count += last.iter().fold(0, |acc, f| if !f {acc +1} else {acc});
    }
    count
}

fn parse(input : &String) -> Vec<bool>{
    input.chars().into_iter().map(|x| if x == '^' {true} else {false}).collect::<Vec<bool>>()
}


#[cfg(test)]
mod test{
    use crate::{parse, part1};

    #[test]
    fn test_part1(){
        assert_eq!(part1(parse(&".^^.^.^^^^".to_string()), 10), 38);
    }
}
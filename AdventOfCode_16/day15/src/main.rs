use std::fs;
use std::time::Instant;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let mut d = parse(&text);
    let r1 = part1(&d);
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    d.push((11, 0));
    let r2 = part1(&d);
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(_input : Vec<(u8,u8)>) -> String{
    unimplemented!();
}

fn part1(input : &Vec<(u8,u8)>) -> usize{
    for i in 0..{
        let mut jumps = true;
        for j in 0..input.len(){
            if (j+1 + i + input[j].1 as usize) % input[j].0 as usize != 0{
                jumps = false;
                break;
            }
        }
        if jumps{return  i;}
    }
    0
}

fn parse(input : &String) -> Vec<(u8,u8)>{
    let mut res = Vec::new();
    for line in input.lines(){
        let espacios = line.split_whitespace().collect::<Vec<&str>>();
        res.push((espacios[3].parse().unwrap(), espacios[11].replace(".","").parse().unwrap()));
    }
    res
}


#[cfg(test)]
mod test{
    use std::fs;
    use crate::{parse, part1};

    #[test]
    fn test_parse(){
        let text = fs::read_to_string("test.txt").unwrap();
        assert_eq!(parse(&text), vec![(5,4), (2,1)]);
    }

    #[test]
    fn test_part1(){
        let text = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(&parse(&text)), 5);
    }
}
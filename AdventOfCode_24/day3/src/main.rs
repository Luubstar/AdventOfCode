use std::fs::{self};
use std::time::Instant;

use regex::Regex;

type Mult = (u16, u16);

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let input = &parse(&text);
    let r1 = part1(&input);
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part1(&parse_2(&text));
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part1(input : &Vec<Mult>) -> u32{
    let mut count:u32 = 0;
    for (v1, v2) in input{
        count += *v1 as u32 * *v2 as u32;
    } 

    count
}

fn parse(input : &str) -> Vec<Mult>{
    let re = Regex::new(r"mul\(\b\d{1,3}\b,\b\d{1,3}\b\)").unwrap();
    let mut res = Vec::new();

    for cap in re.captures_iter(input){
        let values = &cap[0].split(",").collect::<Vec<&str>>();
        let v1 = values[0].split("(").collect::<Vec<&str>>()[1].parse::<u16>().expect("Expected number");
        let v2 = values[1].split(")").collect::<Vec<&str>>()[0].parse::<u16>().expect("Expected number");
        res.push((v1,v2));
    }

    res
}

fn parse_2(input : &str) -> Vec<Mult>{
    let re = Regex::new(r"(?<mul>mul\(\b\d{1,3}\b,\b\d{1,3}\b\))|(?<do>do\(\))|(?<dont>don\'t\(\))").unwrap();
    let mut res = Vec::new();
    let mut reading = true;

    for cap in re.captures_iter(input){
        if cap.name("mul").is_some() && reading{
            let values = &cap[0].split(",").collect::<Vec<&str>>();
            let v1 = values[0].split("(").collect::<Vec<&str>>()[1].parse::<u16>().expect("Expected number");
            let v2 = values[1].split(")").collect::<Vec<&str>>()[0].parse::<u16>().expect("Expected number");
            res.push((v1,v2));
        }
        else if cap.name("do").is_some(){
            reading = true;
        }
        else if cap.name("dont").is_some(){
            reading = false;
        }
    }

    res
}


#[cfg(test)]
mod test{
    use crate::parse;
    use crate::part1;
    use crate::parse_2;

    #[test]
    fn test_parse(){
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(parse(input), vec![(2,4), (5,5), (11,8), (8,5)]);
    }

    #[test]
    fn test_part1(){
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(part1(&parse(input)), 161);
    }

    #[test]
    fn test_part2(){
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(part1(&parse_2(input)), 48);
    }

}
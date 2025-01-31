use std::fs;
use std::time::Instant;
use crate::DIRECTION::{Ascend, Descend};

enum DIRECTION{
    Ascend(),
    Descend(),
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let input = parse(&text);
    let r1 = part1(&input);
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(&input);
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(input : &Vec<Vec<&str>>) -> u32{
    let mut count:u32 = 0;

    for line in input {
        let values = 
            line.iter().map(|x| x.parse::<u32>().expect("Expected number")).collect::<Vec<u32>>();
        if is_safe(&values) {count += 1;}
        else
        {
            for i in 0..values.len(){
                let mut dampened = values.clone();
                dampened.remove(i);
                if is_safe(&dampened){
                    count += 1;
                    break;
                }
            }
        } 
    } 

    count
}

fn part1(input : &Vec<Vec<&str>>) -> u32{
    let mut count:u32 = 0;

    for line in input {
        let values = 
            line.iter().map(|x| x.parse::<u32>().expect("Expected number")).collect::<Vec<u32>>();
                if is_safe(&values) {count += 1;} 
    } 

    count
}

fn is_safe(values : &Vec<u32>) -> bool{
   let dif = *values.get(0).expect("Expected 1º value") as i32 - 
                      *values.get(1).expect("Expected 2º value") as i32;

    if dif.abs() < 1 || dif.abs() > 3 {return  false;}
        
    let direction = if dif > 0 { Ascend() } else { Descend() };

    let mut last_value = values.get(0).expect("Expected 1º value");

    for value in values.iter().skip(1) {
        let dif:i32 = *last_value as i32 - *value as i32;
        
        match direction {
            Ascend() if (dif < 1 || dif > 3) => {return false;},
            Descend() if (dif > -1 || dif < -3) => {return false;},
            _ => {}
        }

        last_value = value;
    }

    return true;
}

fn parse(input : &str) -> Vec<Vec<&str>>{
    input.lines()
         .collect::<Vec<&str>>()
         .iter()
         .map(|x| x.split_whitespace().collect::<Vec<&str>>())
         .collect()
}


#[cfg(test)]
mod test{
    use crate::{parse, part1, part2};

    #[test]
    fn test_parse(){
        let input =    "7 6 4 2 1
                            1 2 7 8 9";
        assert_eq!(parse(input), vec![vec!["7","6","4","2","1"],vec!["1","2","7","8","9"]]);
    }

    #[test]
    fn test_part1(){
        let input ="7 6 4 2 1
                        1 2 7 8 9
                        9 7 6 2 1
                        1 3 2 4 5
                        8 6 4 4 1
                        1 3 6 7 9";
        assert_eq!(part1(&parse(input)), 2);
    }

    #[test]
    fn test_part2(){
        let input ="7 6 4 2 1
                        1 2 7 8 9
                        9 7 6 2 1
                        1 3 2 4 5
                        8 6 4 4 1
                        1 3 6 7 9";
        assert_eq!(part2(&parse(input)), 4);
    }

}
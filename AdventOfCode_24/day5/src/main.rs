use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let input = parse(&text);
    let r1 = part1(&input);
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1.0, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(&mut (input.0, r1.1));
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2((before, lines) : &mut (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)) -> u32{
    let mut value = 0;

    for line in lines.iter_mut(){
        while !is_update_correct(&line, before) {
            let mut past = Vec::new();

            for value in &mut *line{
                let mut moved = false;

                if let Some(anteriores) = before.get(&value){
                    for anterior in anteriores{

                        if let Some(pos) = past.iter().position(|&x| x == *anterior){
                            past.insert(pos, *value);
                            moved = true;
                            break;
                        }
                    }
                }
                if !moved {past.push(*value);}
            }
            *line = past;
        }

        if let Some(&mid) = &line.get(line.len() / 2) {
            value += mid;
        }
    }
    value
}

fn part1((before, updates) : &(HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)) -> (u32, Vec<Vec<u32>>){
    let mut correct = 0;
    let mut incorrect = Vec::new();

    for line in updates{ 
        if !is_update_correct(line, &before) {incorrect.push(line.clone()); continue;}

        if let Some(&mid) = line.get(line.len() / 2) {
            correct += mid;
        }
    } 

    (correct, incorrect)
}

fn is_update_correct(line: &Vec<u32>, before: &HashMap<u32, Vec<u32>>) -> bool{
    let mut past = HashSet::new();
    let mut is_bad = false;

    for &value in line{
        if let Some(anteriores) = before.get(&value){
            anteriores.iter().for_each(|x: &u32| if past.contains(x) {is_bad = true;} );
        }

        if is_bad {return false;}

        past.insert(value);
    }
    return true;
}

fn parse(input : &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>){
    let mut second_part = false;
    let mut before_hash: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates = Vec::new();

    for line in input.lines(){
        if line.is_empty() {second_part = true; continue;}

        if second_part {
            let numbers = line.split(",").map(
                |x| x.parse::<u32>().expect(&format!("{x} should be a number"))
            ).collect::<Vec<u32>>();
            updates.push(numbers);
        }
        else{
            let mut parts = line.split('|');

            if let (Some(x), Some(y)) = (parts.next(), parts.next()) {
                let x = x.parse::<u32>().expect(&format!("'{}' should be a number", x));
                let y = y.parse::<u32>().expect(&format!("'{}' should be a number", y));

                before_hash.entry(x).or_default().push(y);
            } else {
                panic!("Invalid input format: {}", line);
            }; 
        }
    } 

    (before_hash, updates)    
}


#[cfg(test)]
mod test{
    use std::{collections::HashMap, fs};

    use crate::{parse, part1, part2};


    #[test]
    fn test_parse(){
        let test_input = 
        "1|2\n2|3\n\n1,2,3";
        
        let mut before_hash: HashMap<u32, Vec<u32>> = HashMap::new();
        
        before_hash.entry(1).or_insert_with(Vec::new).push(2);
        before_hash.entry(2).or_insert_with(Vec::new).push(3);

        assert_eq!(parse(test_input), (before_hash, vec![vec![1,2,3]]))
    }

    #[test]
    fn test_part1(){
        let input = fs::read_to_string("testInput.txt").unwrap();

        assert_eq!(part1(&parse(&input)).0, 143);
    }

    #[test]
    fn test_part2(){
        let input = fs::read_to_string("testInput.txt").unwrap();
        let parse = parse(&input);
        let p1 = part1(&parse);

        assert_eq!(part2(&mut (parse.0, p1.1)), 123);
    }

}
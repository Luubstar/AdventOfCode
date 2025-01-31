use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

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

fn part2(input : &Vec<(usize, VecDeque<u16>)>) -> usize{
    let mut count = 0;

    for (objective, parts) in input{
        let mut newparts = parts.clone();
        let first = newparts.pop_front().unwrap();
        if complete_checks_part2(*objective, &newparts,first as usize) {count += objective;}
    }

    count
}

fn part1(input : &Vec<(usize, VecDeque<u16>)>) -> usize{
    let mut count = 0;

    for (objective, parts) in input{
        let mut newparts = parts.clone();
        let first = newparts.pop_front().unwrap();
        if complete_checks_part1(*objective, &newparts,first as usize) {count += objective;}
    }

    count
}

fn complete_checks_part2(obj: usize, components: &VecDeque<u16>, carry:usize) -> bool{
    if carry == obj && components.is_empty() {return true;}
    else if carry > obj || components.is_empty() {return false;}

    let mut temp_components = components.clone();
    let head = temp_components.pop_front().unwrap() as usize;

    let mut concat_op = carry.to_string();
    concat_op.push_str(&head.to_string());

    return complete_checks_part2(obj, &temp_components, carry + head) 
           || complete_checks_part2(obj, &temp_components, carry * head)
           || complete_checks_part2(obj, &temp_components, concat_op.parse::<usize>().unwrap());
}

fn complete_checks_part1(obj: usize, components: &VecDeque<u16>, carry:usize) -> bool{
    if carry == obj {return true;}
    else if carry > obj || components.is_empty() {return false;}

    let mut temp_components = components.clone();
    let head = temp_components.pop_front().unwrap() as usize;

    return complete_checks_part1(obj, &temp_components, carry + head) 
           || complete_checks_part1(obj, &temp_components, carry * head);
}

fn parse(input : &str) -> Vec<(usize, VecDeque<u16>)>{
    let mut parts = Vec::new();

    for line in input.lines(){
        let mut numbers = VecDeque::new();
        let mut first_separator = line.split(":");

        if let (Some(objective), Some(values)) = (first_separator.next(), first_separator.next()){
            for num in values.split_whitespace(){
                numbers.push_back(num.parse::<u16>().unwrap())
            }
            parts.push((objective.parse::<usize>().unwrap(), numbers));
        }
    }

    parts
}


#[cfg(test)]
mod test{
    use std::{collections::VecDeque, fs, vec};
    use crate::{parse, part1, part2};

    #[test]
    fn test_parse(){
        let text = fs::read_to_string("inputTestParse.txt").unwrap();
        
        assert_eq!(parse(&text), vec![
            (190, VecDeque::from(vec![10, 19])),
            (3267, VecDeque::from(vec![81, 40, 27])),
            (83, VecDeque::from(vec![17, 5]))
        ]);
    }

    #[test]
    fn test_part1(){
        let text = fs::read_to_string("inputTest.txt").unwrap();
        
        assert_eq!(part1(&parse(&text)), 3749);
    }

    #[test]
    fn test_part2(){
        let text = fs::read_to_string("inputTest.txt").unwrap();
        
        assert_eq!(part2(&parse(&text)), 11387);
    }

}
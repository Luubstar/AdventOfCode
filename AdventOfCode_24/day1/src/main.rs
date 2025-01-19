use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::time::Instant;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let listas = parse(&text);
    let r1 = part1(&mut listas.clone());
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(&mut listas.clone());
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2((left, rigth) : &mut (Vec<&str>, Vec<&str>)) -> String{
    let mut dict_similarity:HashMap<&str, usize> = HashMap::new();
    let mut similarity:usize = 0;

    rigth.iter().for_each(|x| {
        dict_similarity.insert(x,
        x.parse::<usize>().expect("Expected number") * rigth.iter().filter(|y| **y == *x).count());
    });
    left.iter().for_each(|x|
        match dict_similarity.get(x)
        {
            Some(y) => {similarity += y;}
            _ => {}
        }
    );

    String::from(format!("{similarity}"))
}

fn part1((left, rigth) : &mut (Vec<&str>, Vec<&str>)) -> String {
    let mut result: usize = 0;

    let mut left: Vec<usize> = left
        .iter()
        .map(|x| x.parse::<usize>().expect("Failed to parse"))
        .collect();
    let mut right: Vec<usize> = rigth
        .iter()
        .map(|x| x.parse::<usize>().expect("Failed to parse"))
        .collect();

    left.sort();
    right.sort();

    for (x, y) in left.iter().zip(right.iter()) {
        result += x.abs_diff(*y);
    }

    result.to_string()
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut first_elements = Vec::new();
    let mut last_elements = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Some(first), Some(last)) = (parts.first(), parts.last()) {
            first_elements.push(*first);
            last_elements.push(*last);
        } else {
            panic!("Each line must have at least one element");
        }
    }

    (first_elements, last_elements)
}


#[cfg(test)]
mod test{
    use crate::{parse, part1, part2};

    #[test]
    fn test_parse(){
        let entrada = "123 456";

        assert_eq!(parse(entrada), (vec!["123"], vec!["456"]));
    }

    #[test]
    fn test_part1(){
        let entrada = "1 2";

        assert_eq!(part1(&mut parse(entrada)), "1");
    }

    #[test]
    fn test_part2(){
        let entrada = "2 2";
        let entrada2 = "2 2\n1 2";

        assert_eq!(part2(&mut parse(entrada)), "2");
        assert_eq!(part2(&mut parse(entrada2)), "4");
    }

}

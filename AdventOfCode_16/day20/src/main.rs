use std::fs;
use std::ops::{Range};
use std::time::Instant;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let r1 = part1(parse(&text));
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(parse(&text));
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(input : Vec<Range<u32>>) -> u32{
    let mut cuenta = 0;

    for r in input{
        cuenta += r.len()
    }

    u32::MAX - cuenta as u32
}

fn part1(input : Vec<Range<u32>>) -> u32{
    for i in 0..u32::MAX{
        if !input[0].contains(&i){return i;}
    }
    0
}

fn parse(input : &String) -> Vec<Range<u32>>{
    let mut res: Vec<Range<u32>> = Vec::new();
    for linea in input.lines(){
        let r = linea.split("-").collect::<Vec<&str>>();
        let start =  r[0].parse::<u32>().unwrap();
        let mut end = r[1].parse::<u32>().unwrap();
        if r[1].parse::<u32>().unwrap() != u32::MAX{end += 1;}
        let range:Range<u32> = start..end;
        res.push(range);
    }
    compress(res)
}

fn compress(mut input: Vec<Range<u32>>) -> Vec<Range<u32>> {
    if input.is_empty() {
        return input;
    }

    input.sort_by_key(|range| range.start);

    let mut result = Vec::new();
    let mut current = input[0].clone();

    for range in input.into_iter().skip(1) {
        if range.start <= current.end {
            current.end = current.end.max(range.end);
        } else {
            result.push(current);
            current = range;
        }
    }
    result.push(current);
    result.sort_by_key(|r| r.start);
    result
}

#[cfg(test)]
mod test{
    use std::fs;
    use crate::{parse, part1, part2};

    #[test]
    fn test_parse(){
        let test = fs::read_to_string("test.txt").unwrap();
        assert_eq!(parse(&test), vec![0..3, 4..9])
    }

    #[test]
    fn test_part1(){
        let test = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(parse(&test)), 3);
    }

    #[test]
    fn test_part2(){
        let test = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part2(parse(&test)), 2);
    }

}
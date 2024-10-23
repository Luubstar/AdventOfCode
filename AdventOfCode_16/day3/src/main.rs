use std::fs;
use std::time::Instant;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let r1 = solve(parse(&text));
    println!("Resultado de la parte 1: {}\nObtenida en {} ms",r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = solve(parse2(&text));
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn solve(tripletas: Vec<(u16, u16, u16)>) -> usize {
    tripletas.iter().filter(|(x, y, z)| x + y > *z).count()
}


fn parse2(content: &String) -> Vec<(u16, u16, u16)>{
    let v: Vec<&str> = content.lines().collect();
    let mut res: Vec<(u16, u16, u16)> = Vec::new();

    for chunk in v.chunks(3) {

        let mut numbers: Vec<Vec<u16>> = Vec::new();

        for line in chunk {
            let line_numbers: Vec<u16> = line.split_whitespace()
                .filter_map(|num| num.parse::<u16>().ok())
                .collect();
            numbers.push(line_numbers);
        }


        res.push(wrap_triangle(vec![numbers[0][0], numbers[1][0], numbers[2][0]]));
        res.push(wrap_triangle(vec![numbers[0][1], numbers[1][1], numbers[2][1]]));
        res.push(wrap_triangle(vec![numbers[0][2], numbers[1][2], numbers[2][2]]));
    }

    res
}

fn parse(content: &String) -> Vec<(u16, u16, u16)>{
    content.lines().map(|line| {
        let numbers: Vec<u16> = line.split_whitespace()
            .filter_map(|num| num.parse::<u16>().ok())
            .collect();

        wrap_triangle(numbers)
    }).collect()
}

fn wrap_triangle(mut numbers: Vec<u16>) -> (u16, u16, u16){
    let max_num = *numbers.iter().max().unwrap();
    let index = numbers.iter().position(|x| *x == max_num).unwrap();
    numbers.remove(index);
    (numbers[0], numbers[1], max_num)
}

#[cfg(test)]
mod test{
    use crate::{parse, parse2, solve};

    #[test]
    fn test_parse(){
        let r = parse(&"123 123 123".to_string());
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].0, 123);
        assert_eq!(r[0].1, 123);
        assert_eq!(r[0].2, 123);
    }

    #[test]
    fn test_parse2(){
        let r = parse2(&"123 000 000\n123 111 111\n123 222 222".to_string());
        assert_eq!(r.len(), 3);
        assert_eq!(r[0].0, 123);
        assert_eq!(r[0].1, 123);
        assert_eq!(r[0].2, 123);
    }

    #[test]
    fn test_part1(){
        assert_eq!(solve(parse(&"123 123 123".to_string())), 1);
        assert_eq!(solve(parse(&"5 10 25".to_string())), 0);
    }

}
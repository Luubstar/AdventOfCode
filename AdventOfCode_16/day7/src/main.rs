use std::fs;
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

fn part2(input : Vec<(Vec<&str>,Vec<&str>)>) -> u16{
    input.iter().fold(0, |acc, (c, i)|{

        let ssl_c: Vec<String> = c.iter().flat_map(|x| generate_ssl_switched(x)).collect();
        let ssl_i: Vec<String> = i.iter().flat_map(|x| generate_ssl(x)).collect();
        let contains_ssl = ssl_i.iter().any(|ssl| ssl_c.contains(ssl));

        if contains_ssl { acc + 1 } else { acc }
    })
}

fn part1(input : Vec<(Vec<&str>,Vec<&str>)>) -> u16{
    input.iter().fold(0, |acc, (c, i)|
        if c.iter().any(|x| has_abba(x)) && !i.iter().any(|x| has_abba(x))
        {acc + 1} else {acc})
}

fn parse(texto: &str) -> Vec<(Vec<&str>, Vec<&str>)>{
    texto.lines().map(|linea| internal_parse(linea)).collect()
}

fn internal_parse(linea : &str) -> (Vec<&str>, Vec<&str>){
    let mut correct : Vec<&str> = Vec::new();
    let mut incorrect : Vec<&str> = Vec::new();

    let mut parts = linea.split_inclusive(['[', ']']);

    while let Some(part) = parts.next(){
        if part.ends_with('[') {
            correct.push(part.trim_end_matches('['));
            if let Some(inner) = parts.next() {
                incorrect.push(inner.trim_end_matches(']'));
            }
        } else {
            correct.push(part);
        }
    }


    (correct, incorrect)
}

fn has_abba(input : &str) -> bool{
    input.chars().collect::<Vec<char>>()
        .windows(4)
        .map(|ventana|
            if ventana[0] == ventana[3] && ventana[1] == ventana[2] && ventana[0] != ventana[1]
            {true} else {false})
        .any(|x| x)
}

fn generate_ssl_switched(input : &str) -> Vec<String>{
    input.chars().collect::<Vec<char>>()
        .windows(3)
        .filter(|ventana|
            ventana[0] == ventana[2] && ventana[0] != ventana[1])
        .map(|ventana| format!("{}{}{}", ventana[1], ventana[0], ventana[1]))
        .collect()
}
fn generate_ssl(input : &str) -> Vec<String>{
    input.chars().collect::<Vec<char>>()
        .windows(3)
        .filter(|ventana|
            ventana[0] == ventana[2] && ventana[0] != ventana[1])
        .map(|ventana| format!("{}{}{}", ventana[0], ventana[1], ventana[0]))
        .collect()
}

#[cfg(test)]
mod test{
    use std::fs;
    use crate::{parse, part1, part2};

    #[test]
    fn test_parse(){
        assert_eq!(parse(&"abba[mnop]qrst".to_string())[0], (vec!["abba", "qrst"], vec!["mnop"]));
    }

    #[test]
    fn test_part1(){
        let text = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(parse(&text)), 2);
    }

    #[test]
    fn test_part2(){
        let text = fs::read_to_string("test2.txt").unwrap();
        assert_eq!(part2(parse(&text)), 3);
    }

}
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Debug, PartialEq)]
enum Instruccion{
    Cpy(String, String),
    Inc(String),
    Dec(String),
    Jnz(String, isize)
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let r1 = part1(parse(&text));
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(parse(&text));
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(input : Vec<Instruccion>) -> i64{
    let mut hash: HashMap<String, i64> = HashMap::new();
    hash.insert("a".to_string(), 0);
    hash.insert("b".to_string(), 0);
    hash.insert("c".to_string(), 1);
    hash.insert("d".to_string(), 0);
    hash.insert("1".to_string(), 1);
    hash.insert("0".to_string(), 0);

    let mut i = 0;
    while i < input.len(){
        let instruccion = &input[i];
        match instruccion{
            Instruccion::Cpy(v, r) => {
                if hash.contains_key(v){hash.insert(r.clone(), *hash.get(v).unwrap());}
                else{hash.insert(r.clone(), v.parse().unwrap());}
            }
            Instruccion::Inc(r) => {hash.insert(r.clone(), *hash.get(r).unwrap() + 1); }
            Instruccion::Dec(r) => {hash.insert(r.clone(), *hash.get(r).unwrap() - 1); }
            Instruccion::Jnz(r, v) if *hash.get(r).unwrap() != 0 => {
                i = ((i as isize) + *v - 1) as usize}
            _ => {}
        }
        i += 1;
    }

    hash.get("a").unwrap().clone()
}

fn part1(input : Vec<Instruccion>) -> i64{
    let mut hash: HashMap<String, i64> = HashMap::new();
    hash.insert("a".to_string(), 0);
    hash.insert("b".to_string(), 0);
    hash.insert("c".to_string(), 0);
    hash.insert("d".to_string(), 0);
    hash.insert("1".to_string(), 1);
    hash.insert("0".to_string(), 0);

    let mut i = 0;
    while i < input.len(){
        let instruccion = &input[i];
        match instruccion{
            Instruccion::Cpy(v, r) => {
                if hash.contains_key(v){hash.insert(r.clone(), *hash.get(v).unwrap());}
                else{hash.insert(r.clone(), v.parse().unwrap());}
            }
            Instruccion::Inc(r) => {hash.insert(r.clone(), *hash.get(r).unwrap() + 1); }
            Instruccion::Dec(r) => {hash.insert(r.clone(), *hash.get(r).unwrap() - 1); }
            Instruccion::Jnz(r, v) if *hash.get(r).unwrap() != 0 => {
                i = ((i as isize) + *v - 1) as usize}
            _ => {}
        }
        i += 1;
    }

    hash.get("a").unwrap().clone()
}

fn parse(input : &String) -> Vec<Instruccion>{
    let mut res = Vec::new();

    for linea in input.lines(){
        let espacios = linea.split_whitespace().collect::<Vec<&str>>();
        if linea.starts_with("cpy"){
            res.push(Instruccion::Cpy(espacios[1].to_string(), espacios[2].to_string())) }
        else if linea.starts_with("inc"){
            res.push(Instruccion::Inc(espacios[1].to_string()))}
        else if linea.starts_with("dec"){
            res.push(Instruccion::Dec(espacios[1].to_string()))}
        else if linea.starts_with("jnz"){
            res.push(Instruccion::Jnz(espacios[1].to_string(), espacios[2].trim().parse().unwrap()))
        }
    }

    res
}

#[cfg(test)]
mod test{
    use std::fs;
    use crate::{parse, part1, Instruccion};

    #[test]
    fn test_parse(){
        let text = fs::read_to_string("test.txt").unwrap();
        assert_eq!(parse(&text), vec![
            Instruccion::Cpy("41".to_string(), "a".to_string()),
            Instruccion::Inc("a".to_string()),
            Instruccion::Inc("a".to_string()),
            Instruccion::Dec("a".to_string()),
            Instruccion::Jnz("a".to_string(), 2),
            Instruccion::Dec("a".to_string()),
        ])
    }

    #[test]
    fn test_part1(){
        let text = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(parse(&text)), 42);
    }

}
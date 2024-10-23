use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Debug, PartialEq, Clone)]
enum Instruccion{
    Cpy(String, String),
    Inc(String),
    Dec(String),
    Jnz(String, String),
    Tgl(String)
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

fn part2(mut input : Vec<Instruccion>) -> i64{
    let mut hash: HashMap<String, i64> = HashMap::new();
    hash.insert("a".to_string(), 12);
    hash.insert("b".to_string(), 0);
    hash.insert("c".to_string(), 0);
    hash.insert("d".to_string(), 0);

    let mut i = 0;
    while i < input.len(){
        if i + 5 < input.len() && multiplica(&input, i, &mut hash) {i+=6; continue;}
        let instruccion = input.get(i).unwrap().clone();
        match instruccion{
            Instruccion::Cpy(v, r) => {
                if hash.contains_key(&v){hash.insert(r.clone(), *hash.get(&v).unwrap());}
                else{hash.insert(r.clone(), v.parse().unwrap());}
            }
            Instruccion::Inc(r) => {if hash.contains_key(&r) {hash.insert(r.clone(), *hash.get(&r).unwrap() + 1);}}
            Instruccion::Dec(r) => {if hash.contains_key(&r) {hash.insert(r.clone(), *hash.get(&r).unwrap() - 1);}}
            Instruccion::Jnz(r, v) => {
                let value;
                let comparator;

                if hash.contains_key(&r){comparator = *hash.get(&r).unwrap()}
                else if r.parse::<isize>().is_ok(){comparator = r.parse::<i64>().unwrap();}
                else {i+=1; continue;}

                if comparator == 0 {i+=1; continue}

                if hash.contains_key(&v){value = *hash.get(&v).unwrap();}
                else if v.parse::<isize>().is_ok(){value = v.parse::<i64>().unwrap();}
                else{i+=1; continue;}

                i = ((i as isize) + value as isize -1) as usize;
            }
            Instruccion::Tgl(r) =>{
                let editada;
                if hash.contains_key(&r) {
                    editada = input.get_mut(i + *hash.get(&r).unwrap() as usize);
                    if editada.is_none() { i+=1; continue; }
                }
                else if r.parse::<isize>().is_ok() {
                    editada = input.get_mut(i + r.parse::<usize>().unwrap());
                    if editada.is_none() { i+=1; continue; }
                }
                else {i+=1; continue;}
                let editada = editada.unwrap();
                match editada {
                    Instruccion::Cpy(r,v) => {*editada = Instruccion::Jnz(r.clone(), v.clone());}
                    Instruccion::Inc(r) => {*editada = Instruccion::Dec(r.clone());}
                    Instruccion::Dec(r) => {*editada = Instruccion::Inc(r.clone());}
                    Instruccion::Jnz(r, v) => {*editada = Instruccion::Cpy(r.clone(), v.clone());}
                    Instruccion::Tgl(r) => {*editada = Instruccion::Inc(r.clone());}
                }
            }
        }
        i += 1;
    }

    hash.get("a").unwrap().clone()
}

fn multiplica(input: &Vec<Instruccion>, i: usize, hash: &mut HashMap<String, i64>) -> bool{
    if let (
        Instruccion::Cpy(x, w),
        Instruccion::Inc(a),
        Instruccion::Dec(w1),
        Instruccion::Jnz(w2, m),
        Instruccion::Dec(d),
        Instruccion::Jnz(d2, m2)
    ) = (&input[i], &input[i +1], &input[i +2], &input[i +3], &input[i +4], &input[i +5]) {
        if w1 == w2 && *m == "-2" &&
            d == d2 && *m2 == "-5" &&
            w == w1 && a != w && a != w2 {
            let c_val = if let Ok(n) = x.parse::<i64>() { n } else { *hash.get(x).unwrap() };
            let d_val = *hash.get(d).unwrap();
            let a_val = hash.get(a).unwrap().clone();

            let _ = &hash.insert(a.clone(), a_val + d_val * c_val);
            let _ = &hash.insert(w.clone(), 0);
            let _ = &hash.insert(d2.clone(), 0);

            return true
        }
        return false
    }
    false
}

fn part1(mut input: Vec<Instruccion>) -> i64{
    let mut hash: HashMap<String, i64> = HashMap::new();
    hash.insert("a".to_string(), 7);
    hash.insert("b".to_string(), 0);
    hash.insert("c".to_string(), 0);
    hash.insert("d".to_string(), 0);

    let mut i = 0;
    while i < input.len(){
        let instruccion = input.get(i).unwrap().clone();
        match instruccion{
            Instruccion::Cpy(v, r) => {
                if hash.contains_key(&v){hash.insert(r.clone(), *hash.get(&v).unwrap());}
                else{hash.insert(r.clone(), v.parse().unwrap());}
            }
            Instruccion::Inc(r) => {if hash.contains_key(&r) {hash.insert(r.clone(), *hash.get(&r).unwrap() + 1);}}
            Instruccion::Dec(r) => {if hash.contains_key(&r) {hash.insert(r.clone(), *hash.get(&r).unwrap() - 1);}}
            Instruccion::Jnz(r, v) => {
                let value;
                let comparator;

                if hash.contains_key(&r){comparator = *hash.get(&r).unwrap()}
                else if r.parse::<isize>().is_ok(){comparator = r.parse::<i64>().unwrap();}
                else {i+=1; continue;}

                if comparator == 0 {i+=1; continue}

                if hash.contains_key(&v){value = *hash.get(&v).unwrap();}
                else if v.parse::<isize>().is_ok(){value = v.parse::<i64>().unwrap();}
                else{i+=1; continue;}

                i = ((i as isize) + value as isize -1) as usize;
            }
            Instruccion::Tgl(r) =>{
                let editada;
                if hash.contains_key(&r) {
                    editada = input.get_mut(i + *hash.get(&r).unwrap() as usize);
                    if editada.is_none() { i+=1; continue; }
                }
                else if r.parse::<isize>().is_ok() {
                    editada = input.get_mut(i + r.parse::<usize>().unwrap());
                    if editada.is_none() { i+=1; continue; }
                }
                else {i+=1; continue;}
                let editada = editada.unwrap();
                match editada {
                    Instruccion::Cpy(r,v) => {*editada = Instruccion::Jnz(r.clone(), v.clone());}
                    Instruccion::Inc(r) => {*editada = Instruccion::Dec(r.clone());}
                    Instruccion::Dec(r) => {*editada = Instruccion::Inc(r.clone());}
                    Instruccion::Jnz(r, v) => {*editada = Instruccion::Cpy(r.clone(), v.clone());}
                    Instruccion::Tgl(r) => {*editada = Instruccion::Inc(r.clone());}
                }
            }
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
            res.push(Instruccion::Jnz(espacios[1].to_string(), espacios[2].trim().to_string()))}
        else if linea.starts_with("tgl"){
            res.push(Instruccion::Tgl(espacios[1].to_string()))}
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
            Instruccion::Cpy("2".to_string(), "a".to_string()),
            Instruccion::Tgl("a".to_string()),
            Instruccion::Tgl("a".to_string()),
            Instruccion::Tgl("a".to_string()),
            Instruccion::Cpy("1".to_string(), "a".to_string()),
            Instruccion::Dec("a".to_string()),
            Instruccion::Dec("a".to_string()),
        ])
    }

    #[test]
    fn test_part1(){
        let text = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(parse(&text)), 3);
    }

}
use std::fs;
use std::time::Instant;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direcciones{
    N,
    S,
    E,
    O,
}

impl Direcciones{
    fn change (d: Direcciones, pos: i32) -> Direcciones{
        let v = vec![Direcciones::N, Direcciones::E, Direcciones::S, Direcciones::O];

        let i = v.iter().position(|&x| x == d).unwrap();
        let len = v.len() as i32;
        let new_index = (i as i32 + pos).rem_euclid(len);

        v[new_index as usize]
    }
}

#[derive(Debug, PartialEq)]
enum Ordenes{
    L(u32),
    R(u32)
}

struct Posicion{
    pos: (i32, i32),
    direcciones: Direcciones,
}

impl Posicion{
    fn new() -> Posicion{
        Posicion{
            pos: (0,0),
            direcciones: Direcciones::N
        }
    }

    fn advance(&mut self, cant: i32){
        match self.direcciones {
            Direcciones::N => {self.pos = (self.pos.0, self.pos.1 + cant);}
            Direcciones::S => {self.pos = (self.pos.0, self.pos.1 - cant);}
            Direcciones::E => {self.pos = (self.pos.0 + cant, self.pos.1);}
            Direcciones::O => {self.pos = (self.pos.0 - cant, self.pos.1);}
        }
    }

    fn advance2(&mut self, cant: i32, v: &mut Vec<(i32, i32)>) -> Option<(i32, i32)>{
        let (x, y) = self.pos;

        match self.direcciones {
            Direcciones::N => {
                for i in 1..=cant {
                    if v.contains(&(x, y + i)) {return Some((x, y+i))}
                    v.push((x, y + i));
                }
                self.pos = (x, y + cant);
            }
            Direcciones::S => {
                for i in 1..=cant {
                    if v.contains(&(x, y - i)) {return Some((x, y-i))}
                    v.push((x, y - i));
                }
                self.pos = (x, y - cant);
            }
            Direcciones::E => {
                for i in 1..=cant {
                    if v.contains(&(x + i, y)) {return Some((x+i, y))}
                    v.push((x + i, y));
                }
                self.pos = (x + cant, y);
            }
            Direcciones::O => {
                for i in 1..=cant{
                    if v.contains(&(x - i, y)) {return Some((x-i, y))}
                    v.push((x - i, y));
                }
                self.pos = (x - cant, y);
            }
        }
        None
    }
}


fn main() {
    let t = Instant::now();
    let r1 = part1(parse(read("input.txt")));
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1.0.abs() + r1.1.abs(), t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(parse(read("input.txt")));
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2.0.abs() + r2.1.abs(), t.elapsed().as_millis());

}

fn read(path : &str) -> String {
    fs::read_to_string(path).expect(&format!("Fichero no encontrado: {}", path))
}

fn parse(file: String) -> Vec<Ordenes>{
    let v: Vec<Ordenes> = file.trim().split(",").map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x|
        if x.contains("L") {
            Ordenes::L(x[1..].parse().expect("ERROR L"))
        } else {
            Ordenes::R(x[1..].parse().expect("ERROR R"))
        }
    ).collect();
    v
}

fn part1(ordenes: Vec<Ordenes>) -> (i32, i32){
    let mut res = Posicion::new();
    for orden in ordenes{
        if let Ordenes::R(m) = orden{
            res.direcciones = Direcciones::change(res.direcciones, 1);
            res.advance(m as i32);
        }
        else if let Ordenes::L(m) = orden{
            res.direcciones = Direcciones::change(res.direcciones, -1);
            res.advance(m as i32);
        }
    }
    res.pos
}

fn part2(ordenes: Vec<Ordenes>) -> (i32, i32){
    let mut res = Posicion::new();
    let mut posiciones:Vec<(i32, i32)> = vec![(0,0)];
    for orden in ordenes{
        if let Ordenes::R(m) = orden{
            res.direcciones = Direcciones::change(res.direcciones, 1);
            if let Some(t) = res.advance2(m as i32, &mut posiciones){
                return t;
            }
        }
        else if let Ordenes::L(m) = orden{
            res.direcciones = Direcciones::change(res.direcciones, -1);
            if let Some(t) = res.advance2(m as i32, &mut posiciones){
                return t;
            }
        }
    }
    res.pos
}


#[cfg(test)]
mod tests{
    use crate::{parse, part1, part2, Ordenes};

    #[test]
    fn test_parser(){
        let mut v : Vec<Ordenes> = Vec::new();
        v.push(Ordenes::R(2));
        v.push(Ordenes::L(2));
        assert_eq!(parse("R2, L2".to_string()), v);
    }

    #[test]
    fn test_part1(){
        assert_eq!(part1(parse("R2, L2".to_string())), (2,2));
        assert_eq!(part1(parse("R2, R2, R2".to_string())), (0,-2));

        let t = part1(parse("R5, L5, R5, R3".to_string()));
        assert_eq!(t.0 + t.1, 12);
    }

    #[test]
    fn test_part2(){
        let t = part2(parse("R8, R4, R4, R8".to_string()));
        println!("{:#?}", t);
        assert_eq!(t.0.abs() + t.1.abs(), 4)
    }
}
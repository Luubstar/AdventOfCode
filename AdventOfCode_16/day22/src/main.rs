use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Node{
    pos: (u8,u8),
    size: u16,
    used: u16,
    avail: u16
}

impl Node {
    fn new(pos: (u8,u8), size: u16, used: u16, avail: u16) -> Node{
        Node{pos, size, used, avail}
    }
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let r1 = part1(parse(&text));
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1.len(), t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(parse(&text));
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(mut input: Vec<Node>) -> usize{
    input.sort_by(|x, other| x.pos.1.cmp(&other.pos.1));
    let mut steps:u16 = 0;
    let start = input.clone().into_iter().find(|x| x.pos == (input.iter()
    .fold(0, |acc, x| if x.pos.0 > acc {x.pos.0} else {acc}),0)).unwrap();

    let mut empty = input.clone().iter().find(|x| x.avail >= start.used).unwrap().pos;

    let max_x:u16 = input.clone().iter().fold(0, |acc, x| if x.pos.0 > acc {x.pos.0} else {acc}) as u16;
    steps += empty.0 as u16 + empty.1 as u16 + (max_x-1) + (max_x-3)*5 + 1;

    steps as usize
}

fn part1(input : Vec<Node>) -> Vec<(Node, Node)>{
    let mut res: Vec<(Node, Node)> = Vec::new();

    for i in 0..input.len(){
        for j in 0..input.len(){
            let a = input[i];
            let b = input[j];
            if a != b && a.used != 0 && b.avail - a.used <= b.size {res.push((a, b));}
        }
    }
    res
}

fn parse(input : &String) -> Vec<Node>{
    let mut res = Vec::new();
    for node in input.lines().skip(2){
        let espacios = node.split_whitespace().collect::<Vec<&str>>();
        let nombre = espacios[0].replace("/dev/grid/node-", "").replace("x", "")
            .replace("y","");
        let coords = nombre.split("-").collect::<Vec<&str>>();

        res.push(Node::new(
            (coords[0].parse().unwrap(), coords[1].parse().unwrap()),
            espacios[1].replace("T","").parse().unwrap(),
            espacios[2].replace("T","").parse().unwrap(),
            espacios[3].replace("T","").parse().unwrap(),
        ));
    }
    res
}


#[cfg(test)]
mod test{
    use std::fs;
    use crate::{parse, Node};

    #[test]
    fn test_parse(){
        let text = fs::read_to_string("test.txt").unwrap();
        assert_eq!(parse(&text), vec![
            Node::new((0,0), 93, 67, 26),
            Node::new((0,1), 89, 70, 19)
        ]);
    }

    #[test]
    fn test_part1(){
    }

    #[test]
    fn test_part2(){
    }

}
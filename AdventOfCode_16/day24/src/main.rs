use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;
use crate::Tile::{Floor, Wall};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum  Tile{
    Wall,
    Floor(Option<u8>),
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let input = calc_distances(parse(&text));

    let t = Instant::now();
    let r1 = part1(input.clone());
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(input.clone());
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(input: Vec<(u8, Vec<(u8, u16)>)>) -> u16{
    let s = input.iter().find(|n| n.0 == 0).unwrap();
    let mut min = u16::MAX;
    let mut minr = Vec::new();
    for nodo in s.1.clone(){
        let c =calcular_camino(nodo, &input, vec![0, nodo.0]);
        if c.0.is_some() {
            if c.0.unwrap() < min {
                min = c.0.unwrap();
                minr = c.1;
            }
        }
    }
    min
}

fn part1(input: Vec<(u8, Vec<(u8, u16)>)>) -> u16{
    let s = input.iter().find(|n| n.0 == 0).unwrap();
    let mut min = u16::MAX;
    let mut minr = Vec::new();
    for nodo in s.1.clone(){
        let c =calcular_camino(nodo, &input, vec![0, nodo.0]);
        if c.0.is_some() {
            if c.0.unwrap() < min {
                min = c.0.unwrap();
                minr = c.1;
            }
        }
    }
    min
}

fn calcular_camino(nodo: (u8, u16), nodos: &Vec<(u8, Vec<(u8, u16)>)>, visitados: Vec<u8>) -> (Option<u16>, Vec<u8>) {
    if visitados.len() == nodos.len() {
        let n = nodos.iter().find(|n| n.0 == 0).unwrap();
        let cost = n.1.iter().find(|x| x.0 == nodo.0).unwrap().1;
        return (Some(nodo.1 + cost), visitados);
    }

    let distancia = nodo.1;
    let mut min: Option<u16> = None;
    let mut minvi = Vec::new();

    if let Some(actual) = nodos.iter().find(|x| x.0 == nodo.0) {
        for vecino in actual.1.iter() {
            if !visitados.contains(&vecino.0) && vecino.0 != nodo.0 {
                let mut nuevos_visitados = visitados.clone();
                nuevos_visitados.push(vecino.0);

                let (distancia_vecino, camino_vecino) = calcular_camino(*vecino, nodos, nuevos_visitados);


                if let Some(v) = distancia_vecino {
                    let total = v + distancia;
                    if min.is_none() || total < min.unwrap() {
                        min = Some(total);
                        minvi = camino_vecino;
                    }
                }
            }
        }
    }
    (min, minvi)
}


fn calc_distances(input: (Vec<Vec<Tile>>, Vec<(u8, u8, u8)>)) -> Vec<(u8, Vec<(u8, u16)>)> {
    let mut res = Vec::new();
    let nodos = &input.1;
    let tiles = input.0;

    let height = tiles.len() as u8;
    let width = tiles[0].len() as u8;

    for nodo in nodos {
        let num = nodo.0;
        let mut nums: HashMap<(u8, u8, u8), u16> = HashMap::new();
        let mut open: VecDeque<(u8, u8, u16)> = VecDeque::new();
        let mut close: HashSet<(u8, u8)> = HashSet::new();

        let (nx, ny) = (nodo.1, nodo.2);
        open.push_back((nx, ny, 0));

        while !open.is_empty() && nums.len() < input.1.len() {
            let (x, y, cost) = open.pop_front().unwrap();

            if close.contains(&(x, y)) {
                continue;
            }

            close.insert((x, y));

            let tile = &tiles[y as usize][x as usize];

            if tile == &Wall {
                continue;
            }

            if let Floor(Some(n)) = tile {
                if *n != num && (!nums.contains_key(&(*n, x, y)) || nums[&(*n, x, y)] > cost) {
                    nums.insert((*n, x, y), cost);
                }
            }

            let vecinos = vec![
                (x.wrapping_add(1), y),
                (x.wrapping_sub(1), y),
                (x, y.wrapping_add(1)),
                (x, y.wrapping_sub(1)),
            ];

            for (nx, ny) in vecinos {
                if nx < width && ny < height && !close.contains(&(nx, ny)) {
                    open.push_back((nx, ny, cost + 1));
                }
            }
        }

        let mut values: Vec<(u8, u16)> = nums
            .iter()
            .map(|(&(n, _,_), &cost)| (n, cost))
            .collect();

        values.sort_by_key(|&(x, _)| x);

        res.push((num, values));
    }

    res
}

fn parse(input : &String) -> (Vec<Vec<Tile>>, Vec<(u8, u8, u8)>){
    let mut res = Vec::new();
    let mut nodos = Vec::new();
    let mut y:u8 = 0;

    for line in input.lines(){
        let mut l = Vec::new();
        let mut x:u8 = 0;

        for c in line.chars(){
            match c {
                '#' => {l.push(Wall);}
                '.' => {l.push(Floor(None));}
                 n  => {l.push(Floor(Some(n.to_string().parse().unwrap()))); nodos.push((n.to_string().parse().unwrap(), x, y));}
            }
            x += 1;
        }
        res.push(l);
        y += 1;
    }

    (res,nodos)
}


#[cfg(test)]
mod test{
    use std::fs;
    use crate::{calc_distances, parse, part1};
    use crate::Tile::{Floor, Wall};

    #[test]
    fn test_parse(){
        let test = fs::read_to_string("test.txt").unwrap();
        assert_eq!(parse(&test),
        (vec![
            vec![Wall].repeat(11),
            vec![Wall, Floor(Some(0)), Floor(None), Floor(Some(1)), Floor(None),Floor(None),Floor(None),Floor(None),Floor(None), Floor(Some(2)), Wall],
            vec![Wall, Floor(None), Wall, Wall, Wall, Wall, Wall, Wall, Wall, Floor(None), Wall],
            vec![Wall, Floor(Some(4)), Floor(None), Floor(None), Floor(None), Floor(None), Floor(None), Floor(None), Floor(None), Floor(Some(3)), Wall],
            vec![Wall].repeat(11)
        ], vec![(0,1,1),(1,3,1),(2,9,1),(4,1,3),(3,9,3)])
        );
    }

    #[test]
    fn test_distancia(){
        let test = fs::read_to_string("test.txt").unwrap();
        assert_eq!(calc_distances(parse(&test)), vec![
            (0, vec![(1, 2), (2, 8), (3, 10), (4, 2)]), (1, vec![(0, 2), (2, 6), (3, 8), (4, 4)]),
            (2, vec![(0, 8), (1, 6), (3, 2), (4, 10)]), (4, vec![(0, 2), (1, 4), (2, 10), (3, 8)]),
            (3, vec![(0, 10), (1, 8), (2, 2), (4, 8)])]);
    }

    #[test]
    fn test_part1(){
        let test = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(calc_distances(parse(&test))), 14);
    }

    #[test]
    fn test_part2(){
    }

}
use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

type Coord = (u8, u8);

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

fn part2((input, (cols, rows)): &(HashMap<char, Vec<Coord>>, (u8, u8))) -> u32 {
    let mut expected_places: HashSet<Coord> = HashSet::new();

    let cols = *cols as i16;
    let rows = *rows as i16;

    for key in input.keys() {
        if let Some(positions) = input.get(key) {
            for &pos in positions {
                expected_places.insert(pos);
                for &pos2 in positions {
                    if pos == pos2 {continue;}
                    
                    let x = pos2.0 as i16 - pos.0 as i16;
                    let y = pos2.1 as i16 - pos.1 as i16;

                    let mut new_positions = vec![
                        (pos.0 as i16 - x, pos.1 as i16 - y, -1),
                        (pos2.0 as i16 + x, pos2.1 as i16 + y, 1)
                    ];
                    
                    while !new_positions.is_empty() {
                        let mut temp: Vec<(i16,i16,i16)> = Vec::new();
                        for &(xpos, ypos, dir) in &new_positions {
                            if xpos >= 0 && xpos < cols && ypos >= 0 && ypos < rows {
                                expected_places.insert((xpos as u8, ypos as u8));
                                temp.push((xpos + (x * dir), ypos + (y * dir), dir));
                            }
                        }
                        new_positions = temp;
                    }
                }
            }
        }
    }
    expected_places.len() as u32
}

fn part1((input, (cols, rows)): &(HashMap<char, Vec<Coord>>, (u8, u8))) -> u32 {
    let mut expected_places: HashSet<Coord> = HashSet::new();

    let cols = *cols as i16;
    let rows = *rows as i16;

    for key in input.keys() {
        if let Some(positions) = input.get(key) {
            for &pos in positions {
                for &pos2 in positions {
                    if pos == pos2 {continue;}

                    let x = pos2.0 as i16 - pos.0 as i16;
                    let y = pos2.1 as i16 - pos.1 as i16;

                    let new_positions = [
                        (pos.0 as i16 - x, pos.1 as i16 - y),
                        (pos2.0 as i16 + x, pos2.1 as i16 + y)
                    ];

                    for &(xpos, ypos) in &new_positions {
                        if xpos >= 0 && xpos < cols && ypos >= 0 && ypos < rows {
                            expected_places.insert((xpos as u8, ypos as u8));
                        }
                    }
                }
            }
        }
    }
    expected_places.len() as u32
}

fn parse(input : &str) -> (HashMap<char, Vec<Coord>>, (u8, u8)){
    let mut res = HashMap::new();
    let rows = input.lines().count();
    let mut cols = 0;

    for (y, line) in input.lines().enumerate(){
        cols = line.len();
        for(x, c) in line.chars().enumerate(){
            if c == '.' {continue;}

            res.entry(c).or_insert_with(Vec::new).push((x as u8, y as u8));
        }
    }    
    (res, (cols as u8, rows as u8))
}


#[cfg(test)]
mod test{
    use std::{collections::HashMap, fs, vec};

    use crate::{parse, part1, part2};


    #[test]
    fn test_parse(){
        let input = fs::read_to_string("inputTest.txt").unwrap();

        let mut hashmap: HashMap<char, Vec<(u8, u8)>> = HashMap::new();
        let tempvec = vec![(8, 1), (5, 2), (7, 3), (4, 4)];
        hashmap.entry('0').insert_entry(tempvec);

        let tempvec = vec![(6, 5), (8, 8), (9, 9)];
        hashmap.entry('A').insert_entry(tempvec);

        assert_eq!(parse(&input), (hashmap, (12,12)))
    }

    #[test]
    fn test_part1(){
        let input = fs::read_to_string("inputTest.txt").unwrap();
        assert_eq!(part1(&parse(&input)), 14);
    }

    #[test]
    fn test_part2(){
        let input = fs::read_to_string("inputTest.txt").unwrap();
        assert_eq!(part2(&parse(&input)), 34);
    }

}
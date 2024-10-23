use std::fs;
use std::time::Instant;

fn main() {
    let t = Instant::now();
    let r1 = part1(parse(&read("input.txt")));
    println!("Resultado de la parte 1: {}\nObtenida en {} ms",r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(parse(&read("input.txt")));
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());

}

fn read(path : &str) -> String {
    fs::read_to_string(path).expect(&format!("Fichero no encontrado: {}", path))
}

fn parse(file: &String) -> Vec<&str>{
    file.lines().collect()
}

fn part2(lines: Vec<&str>) -> String{
    let keypad = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' '],
    ];

    let mut res = String::new();
    let (mut x, mut y): (u8,u8) = (0,2);

    for line in lines{
        for car in line.chars(){
            match car {
                'U' if y != 0 && valid_position((x, y-1))  => {y -= 1},
                'D' if y < 4  && valid_position((x, y+1))  => {y += 1},
                'L' if x != 0 && valid_position((x-1, y))  => {x -= 1},
                'R' if x < 4  && valid_position((x+1, y))  => {x += 1},
                _ => {}
            }
        }
        res.push(keypad[y as usize][x as usize]);
    }
    res
}

fn valid_position((x,y) : (u8, u8)) -> bool{
    (x.abs_diff(2) + y.abs_diff(2)) <= 2
}

fn part1(lines: Vec<&str>) -> String{
    let mut res = String::new();
    let (mut x, mut y): (u8,u8) = (1,1);

    for line in lines{
        for car in line.chars(){
            match car {
                'U' if y != 0 => {y -= 1},
                'D' if y < 2  => {y += 1},
                'L' if x != 0 => {x -= 1},
                'R' if x < 2  => {x += 1},
                _ => {}
            }
        }
        let value = y * 3 + x + 1;
        res += &*value.to_string();
    }
    res
}

#[cfg(test)]
mod test{
    use crate::{parse, part1, part2};

    #[test]
    fn test_parse(){
        assert_eq!(vec!["UU".to_string(), "DD".to_string()], parse(&"UU\nDD".to_string()));
    }

    #[test]
    fn test_part1(){
        let lines = vec!["ULL", "RRDDD", "LURDL", "UUUUD"];
        assert_eq!("1985", part1(lines));
    }

    #[test]
    fn test_part2(){
        let lines = vec!["ULL", "RRDDD", "LURDL", "UUUUD"];
        assert_eq!("5DB3", part2(lines));
    }
}
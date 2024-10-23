use std::fs;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Instruccion{
    SwapPosition(u8, u8),
    SwapLetter(char, char),
    RotateL(u8),
    RotateR(u8),
    RotateBased(char),
    Reverse(u8,u8),
    Move(u8,u8)
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();
    let mut entrada = parse(&text);
    let t = Instant::now();
    let r1 = part1(entrada.clone(), "abcdefgh".to_string());
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    entrada.reverse();
    let r2 = part2(entrada, "fbgdceah".to_string());
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(input: Vec<Instruccion>, cadena: String) -> String{
    let mut chars = cadena.chars().collect::<Vec<char>>();
    for instruccion in input{
        match instruccion {
            Instruccion::SwapPosition(y, x) => {chars.swap(x as usize, y as usize);}
            Instruccion::SwapLetter(y, x) => {
                for ch in chars.iter_mut() {
                    if *ch == x {
                        *ch = y;
                    } else if *ch == y {
                        *ch = x;
                    }
                }
            }
            Instruccion::RotateL(x) => {chars.rotate_right(x as usize);}
            Instruccion::RotateR(x) => {chars.rotate_left(x as usize);}
            Instruccion::RotateBased(x) => {chars = undo_rotate_based(x, chars); }
            Instruccion::Reverse(x, y) => {
                chars[x as usize..=y as usize].reverse();
            }
            Instruccion::Move(y, x) => {
                let c = chars.remove(x as usize);
                chars.insert(y as usize, c);
            }
        }

    }
    chars.into_iter().collect()
}

fn undo_rotate_based(x: char, mut chars: Vec<char>) -> Vec<char> {
    let original_chars = chars.clone();
    let len = chars.len();

    for _i in 0..len {
        chars.rotate_left(1);

        if rotate_based(x, &chars) == original_chars {
            return chars;
        }
    }

    chars
}


fn rotate_based(x: char, chars: &Vec<char>) -> Vec<char>{
    let mut c = chars.clone();
    let index = c.iter().position(|&ch| ch == x).unwrap();
    let rotate_amount = if index >= 4 { index + 2 } else { index + 1 } % c.len();
    c.rotate_right(rotate_amount);
    c
}

fn part1(input: Vec<Instruccion>, cadena: String) -> String{
    let mut chars = cadena.chars().collect::<Vec<char>>();

    for instruccion in input{
        match instruccion {
            Instruccion::SwapPosition(x, y) => {chars.swap(x as usize, y as usize);}
            Instruccion::SwapLetter(x, y) => {
                for ch in chars.iter_mut() {
                    if *ch == x {
                        *ch = y;
                    } else if *ch == y {
                        *ch = x;
                    }
                }
            }
            Instruccion::RotateL(x) => {chars.rotate_left(x as usize);}
            Instruccion::RotateR(x) => {chars.rotate_right(x as usize);}
            Instruccion::RotateBased(x) => {chars = rotate_based(x, &chars)}
            Instruccion::Reverse(x, y) => {
                chars[x as usize..=y as usize].reverse();
            }
            Instruccion::Move(x, y) => {
                let c = chars.remove(x as usize);
                chars.insert(y as usize, c);
            }
        }
    }
    chars.into_iter().collect()
}

fn parse(input: &String) -> Vec<Instruccion>{
    let mut res:Vec<Instruccion> = Vec::new();
    for line in input.lines(){
        let spaces = line.split_whitespace().collect::<Vec<&str>>();
        match spaces[0] {
            "move" => {
                res.push(Instruccion::Move(spaces[2].parse().unwrap(), spaces[5].parse().unwrap()));
            }
            "reverse" => {
                res.push(Instruccion::Reverse(spaces[2].parse().unwrap(), spaces[4].parse().unwrap()));
            }

            "rotate" => {
                if spaces[1] == "left"{res.push(Instruccion::RotateL(spaces[2].parse().unwrap()));}
                else if spaces[1] == "right" {res.push(Instruccion::RotateR(spaces[2].parse().unwrap()));}
                else{res.push(Instruccion::RotateBased(spaces[6].parse().unwrap()));}
            }
            "swap" => {
                if spaces[1] == "position" {res.push(Instruccion::SwapPosition(spaces[2].parse().unwrap(), spaces[5].parse().unwrap()));}
                else {res.push(Instruccion::SwapLetter(spaces[2].parse().unwrap(), spaces[5].parse().unwrap()));}
            }
            _ => {}
        }
    }
    res
}


#[cfg(test)]
mod test{
    use std::fs;
    use crate::{parse, part1, part2, Instruccion};

    #[test]
    fn test_parse(){
        let text = fs::read_to_string("test.txt").unwrap();
        assert_eq!(parse(&text), vec![
            Instruccion::SwapPosition(4,0),
            Instruccion::SwapLetter('d', 'b'),
            Instruccion::Reverse(0,4),
            Instruccion::RotateL(1),
            Instruccion::Move(1,4),
            Instruccion::Move(3,0),
            Instruccion::RotateBased('b'),
            Instruccion::RotateBased('d'),
        ]);
    }

    #[test]
    fn test_part1(){
        let test = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(parse(&test), "abcde".to_string()), "decab".to_string());
    }

    #[test]
    fn test_part2(){
        let test = fs::read_to_string("test.txt").unwrap();
        let mut entrada = parse(&test);
        entrada.reverse();
        assert_eq!(part2(entrada, "ecabd".to_string()), "abcde".to_string());
    }

}
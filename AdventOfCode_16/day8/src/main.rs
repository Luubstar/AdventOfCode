use std::fs;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
enum Ordenes{
    Rect(u8, u8),
    RotateColumn(u8, u8),
    RotateRow(u8, u8)
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let r =part1(parse(&text), (50, 6));
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", count_lit(&r), t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(r);
    println!("Resultado de la parte 2: \n{}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(mut rect: Vec<Vec<bool>>) -> String{
    rect.iter_mut()
        .map(|x| x.iter_mut().map(|y| if *y {"#"} else {"."}).collect::<String>())
        .collect::<Vec<String>>().join("\n")
}

fn part1(input : Vec<Ordenes>, (x,y) : (u8, u8)) -> Vec<Vec<bool>>{
    let mut rect =vec![vec![false; x as usize]; y as usize];

    for orden in input{
        match orden {
            Ordenes::Rect(x, y) => {
                rect.iter_mut().take(y  as usize).for_each(|line| {
                    line.iter_mut().take(x as usize).for_each(|coord| {
                        *coord = true;
                    });
                });
            }
            Ordenes::RotateColumn(coord, by) => {
                for _ in 0..by {shift_column_down(&mut rect, coord as usize);}
            }
            Ordenes::RotateRow(coord, by) => {
                shift_rigth(&mut rect[coord as usize], by);
            }
        }
    }

    rect
}

fn count_lit(rect : &Vec<Vec<bool>>) -> i32 {
    rect.into_iter()
        .map(|x| x.iter().fold(0, |acc, y| if *y {acc +1} else {acc}))
        .sum()
}

fn shift_rigth(rect : &mut Vec<bool>, by : u8) {
    if rect.is_empty(){return;}
    rect.rotate_right(by as usize);
}

fn shift_column_down(rect: &mut Vec<Vec<bool>>, col_index: usize) {
    if rect.is_empty() || rect[0].is_empty() {
        return;
    }

    let num_rows = rect.len();
    if col_index >= rect[0].len() {
        return;
    }

    let mut carry = rect[0][col_index];

    for row in 1..num_rows {
        std::mem::swap(&mut carry, &mut rect[row][col_index]);
    }

    rect[0][col_index] = carry;
}

fn parse(texto: &String) -> Vec<Ordenes>{
    texto.lines().into_iter().map(|x|
        if x.starts_with("rect"){
            let dimensions : Vec<&str> = x.split(" ").collect::<Vec<&str>>()[1]
                .split("x").collect::<Vec<&str>>();

            Ordenes::Rect(dimensions[0].parse().unwrap(), dimensions[1].parse().unwrap())
        }
        else{
            let spaces:Vec<&str> = x.split(" ").collect::<Vec<&str>>();
            let by = spaces[4].parse().unwrap();
            let coord = spaces[2].split("=").collect::<Vec<&str>>()[1].parse().unwrap();

            if spaces[1].starts_with("c"){
                Ordenes::RotateColumn(coord, by)
            }
            else{
                Ordenes::RotateRow(coord, by)
            }
        }
    ).collect()
}


#[cfg(test)]
mod test{
    use std::fs;
    use crate::{count_lit, parse, part1, part2, Ordenes};

    #[test]
    fn test_parse(){
        let text = fs::read_to_string("test.txt").unwrap();
        let ordenes = vec![Ordenes::Rect(3, 2), Ordenes::RotateColumn(1,1),
                           Ordenes::RotateRow(0,4), Ordenes::RotateColumn(1,1)];

        assert_eq!(parse(&text), ordenes);
    }

    #[test]
    fn test_part1(){
        let text = fs::read_to_string("test.txt").unwrap();
        let r = part1(parse(&text), (7,3));
        assert_eq!(count_lit(&r), 6);
    }

    #[test]
    fn test_part2(){
        let v = vec![vec![true, true, false], vec![true, false, false], vec![false, false, true]];
        assert_eq!(part2(v), "##.\n#..\n..#");
    }

}
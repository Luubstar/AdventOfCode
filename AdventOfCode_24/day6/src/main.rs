use std::collections::HashSet;
use std::fs;
use std::time::Instant;

type Coord = (i16,i16);

#[derive(PartialEq, Eq, Hash, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let input = parse(&text);
    let r1 = part1(&input);
    let mut rtemp = r1.clone();
    rtemp.insert(input.1);
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", rtemp.len(), t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(&input, &r1);
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2((walls, guard, (cols, rows)): &(HashSet<Coord>, Coord, (i16,i16)), posiciones: &HashSet<(i16, i16)>) -> u32{
    // 2. Por cada posición visitada que no sea el inicio
    // 3. Añadir un obstáculo
    // 4. Comprobar si hay un loop
    let mut count = 0;
    for pos in posiciones{
        let mut modified_walls = walls.clone();
        modified_walls.insert(*pos);
        if has_loop(&modified_walls, &guard, *cols, *rows) {count += 1}
    }
    count
}

fn has_loop(walls: &HashSet<Coord>, guard: &Coord, cols: i16, rows: i16) -> bool{
    let mut visited:HashSet<(i16, i16, Direction)> = HashSet::new();
    let mut actual_guard = *guard;
    let mut direction = Direction::UP;

    loop {
        match direction {
            Direction::UP => {
                let pos = (actual_guard.0, actual_guard.1-1);

                if pos.1 < 0 {break;}

                if walls.contains(&pos){
                    direction = Direction::RIGHT;
                }
                else{
                    actual_guard = pos;   
                    if !visited.insert((pos.0, pos.1, direction.clone())){return true;}
                }
            },
            Direction::DOWN => {
                let pos = (actual_guard.0, actual_guard.1+1);
                
                if pos.1 >= rows {break;}
                
                if walls.contains(&pos){
                    direction = Direction::LEFT;
                }
                else{
                    actual_guard = pos;   
                    if !visited.insert((pos.0, pos.1, direction.clone())){return true;}
                }
            },
            Direction::LEFT => {
                let pos = (actual_guard.0-1, actual_guard.1);

                if pos.0 < 0 {break;}

                if walls.contains(&pos){
                    direction = Direction::UP;
                }
                else{
                    actual_guard = pos;   
                    if !visited.insert((pos.0, pos.1, direction.clone())){return true;}
                }
            },
            Direction::RIGHT => {
                let pos = (actual_guard.0+1, actual_guard.1);

                if pos.0 >= cols {break;}
                
                if walls.contains(&pos){
                    direction = Direction::DOWN;
                }
                else{
                    actual_guard = pos;
                    if !visited.insert((pos.0, pos.1, direction.clone())){return true;}
                }
            },
        }
    }
    return false;
}

fn part1((walls, guard, (cols, rows)) : &(HashSet<Coord>, Coord, (i16,i16))) -> HashSet<(i16, i16)>{
    let mut tiles_occupied: HashSet<(i16, i16)> = HashSet::new();
    let mut actual_guard = *guard;
    let mut direction = Direction::UP;

    loop {
        match direction {
            Direction::UP => {
                let pos = (actual_guard.0, actual_guard.1-1);

                if pos.1 < 0 {break;}

                if walls.contains(&pos){
                    direction = Direction::RIGHT;
                }
                else{
                    actual_guard = pos;   
                    tiles_occupied.insert(pos);   
                }
            },
            Direction::DOWN => {
                let pos = (actual_guard.0, actual_guard.1+1);
                
                if pos.1 >= *rows {break;}
                
                if walls.contains(&pos){
                    direction = Direction::LEFT;
                }
                else{
                    actual_guard = pos;   
                    tiles_occupied.insert(pos);   
                }
            },
            Direction::LEFT => {
                let pos = (actual_guard.0-1, actual_guard.1);

                if pos.0 < 0 {break;}

                if walls.contains(&pos){
                    direction = Direction::UP;
                }
                else{
                    actual_guard = pos;   
                    tiles_occupied.insert(pos);   
                }
            },
            Direction::RIGHT => {
                let pos = (actual_guard.0+1, actual_guard.1);

                if pos.0 >= *cols {break;}
                
                if walls.contains(&pos){
                    direction = Direction::DOWN;
                }
                else{
                    actual_guard = pos;
                    tiles_occupied.insert(pos);   
                }
            },
        }
    }

    tiles_occupied
}

fn parse(input : &str) -> (HashSet<Coord>, Coord, (i16, i16)){
    let mut coords = HashSet::new();
    let mut guard = (0,0);
    let rows = input.lines().count();
    let mut cols = 0;

    for (y, line) in input.lines().enumerate(){
        cols = line.len();
        for (x, char) in line.chars().enumerate(){
            if char == '#'{
                coords.insert((x as i16,y as i16));
            }
            else if char == '^'{
                guard = (x as i16,y as i16);
            }
        }
    } 

    (coords, guard, (cols as i16, rows as i16))
}


#[cfg(test)]
mod test{
    use std::{collections::HashSet, fs};

    use crate::{parse, part1, part2};


    #[test]
    fn test_parse(){
        let test = fs::read_to_string("input_test.txt").unwrap();
        let vec = vec![(4,0), (9,1), (2,3), (7,4), (1,6), (8,7), (0,8), (6,9)];
        let mut hashset = HashSet::new();
        for v in vec {hashset.insert(v);}

        assert_eq!(parse(&test), (hashset, (4,6), (10, 10)));

    }

    #[test]
    fn test_part1(){
        let test = fs::read_to_string("input_test.txt").unwrap();
        let input = &parse(&test); 
        let mut rtemp = part1(&input);
        rtemp.insert(input.1);
        assert_eq!(rtemp.len(), 41);
    }

    #[test]
    fn test_part2(){
        let test = fs::read_to_string("input_test.txt").unwrap();
        let input = &parse(&test); 
        let rtemp = part1(&input);

        assert_eq!(part2(&input, &rtemp), 6);
    }

}
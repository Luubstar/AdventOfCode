use std::collections::VecDeque;
use std::fs;
use md5::{Md5, Digest, Md5Core};
use std::time::Instant;
use md5::digest::core_api::CoreWrapper;

const ABLE:[char;5] = ['b', 'c', 'd', 'e', 'f'];
fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let r1 = solve(&text);
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1.first().unwrap(), t.elapsed().as_millis());

    let t = Instant::now();
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r1.last().unwrap().len(), t.elapsed().as_millis());
}

fn solve(input : &String) -> Vec<String>{
    let mut hasher = Md5::new();
    let mut abiertos:VecDeque<(String, (u8,u8))> = VecDeque::new();
    abiertos.push_front(("".to_string(), (0,0)));
    let mut ends = Vec::new();
    while let Some((path, (x,y))) = abiertos.pop_front(){
        if (x,y) == (3,3){ends.push(path); continue;}

        let hex_string = &get_hash(&(input.clone() + &path), &mut hasher).chars().collect::<Vec<char>>()[0..4];
        if ABLE.contains(&hex_string[0]) && y > 0 {abiertos.push_back((path.clone() + "U", (x, y-1)));}
        if ABLE.contains(&hex_string[1]) && y < 3 {abiertos.push_back((path.clone() + "D", (x, y+1)));}
        if ABLE.contains(&hex_string[2]) && x > 0 {abiertos.push_back((path.clone() + "L", (x-1, y)));}
        if ABLE.contains(&hex_string[3]) && x < 3 {abiertos.push_back((path.clone() + "R", (x+1, y)));}
    }
    ends
}

fn get_hash(input: &str, hasher:&mut CoreWrapper<Md5Core>) -> String{
    hasher.update(input.as_bytes());
    let result = hasher.finalize_reset();
    result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>()
}


#[cfg(test)]
mod test{
    use crate::solve;

    #[test]
    fn test_part1(){
        assert_eq!(solve(&"ihgpwlah".to_string()), "DDRRRD");
        assert_eq!(solve(&"kglvqrro".to_string()), "DDUDRLRRUDRD");
        assert_eq!(solve(&"ulqzkmiv".to_string()), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

}
use std::collections::HashMap;
use md5::{Md5, Digest, Md5Core};
use std::time::Instant;
use md5::digest::core_api::CoreWrapper;
use rayon::prelude::*;

fn main() {

    let t = Instant::now();
    let r1 = solve(&"ihaygndm".to_string(), 1);
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = solve(&"ihaygndm".to_string(), 2017);
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn solve(input: &String, times: u16) -> usize {
    let mut index: usize = 0;
    let mut count = 0;
    let mut hasher = Md5::new();
    let salt = input.to_owned();
    let mut generated: HashMap<usize, String> = HashMap::new();

    loop {
        let hex_string = get_hash(index, &generated, &mut hasher, &salt, times);
        generated.insert(index, hex_string.clone());

        if let Some(&c) = hex_string.chars().collect::<Vec<_>>().windows(3)
            .find(|window| window.iter().all(|&ch| ch == window[0]))
            .map(|window| &window[0])
        {
            for t_index in 1..=1000 {
                let iter_string = get_hash(index + t_index, &generated, &mut hasher, &salt, times);
                generated.entry(index + t_index).or_insert_with(|| iter_string.clone());

                if iter_string.chars().collect::<Vec<_>>().windows(5)
                    .any(|window| window.iter().all(|&ch| ch == window[0] && ch == c))
                {
                    count += 1;
                    if count >= 64 {
                        return index;
                    }
                }
            }
        }

        generated.remove(&index);
        index += 1;
    }
}

fn get_hash(index:usize, generated: &HashMap<usize, String>, hasher:&mut CoreWrapper<Md5Core>, salt:&String, times:u16) -> String{
    if !generated.contains_key(&index) {
        let mut hex_string = format!("{salt}{index}");
        (0..times).for_each(|_|{
            hasher.update(hex_string.as_bytes());
            hex_string = format!("{:02x}", hasher.finalize_reset());
        });
        hex_string
    }
    else{generated.get(&index).unwrap().clone() }
}


#[cfg(test)]
mod test{
    use crate::solve;

    #[test]
    fn test_part1(){
        assert_eq!(solve(&"abc".to_string(),1), 22728);
    }

    #[test]
    fn test_part2(){
        assert_eq!(solve(&"abc".to_string(),2017), 22551);
    }

}
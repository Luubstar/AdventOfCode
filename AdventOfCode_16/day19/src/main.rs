use std::fs;
use std::time::Instant;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let r1 = part1(text.parse().unwrap());
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(text.parse().unwrap());
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(input: u32) -> usize {
   let mut index = 1;
    while index*3 < input{
        index *= 3
    }
    (input - index) as usize
}


fn part1(input : u32) -> u32{
    let res = vec![true].repeat(input as usize);
    let mut res: Vec<(bool, u32)>  = res.into_iter().enumerate().map(|(i, x)| (x, i as u32 + 1)).collect();
    while res.len() > 1{
        for i in 1..res.len(){
            if res[i-1].0{
                res[i].0 = false;
            }
        }

        if res[res.len()-1].0{
            res[0].0 = false;
        }

        res = res.iter().filter(|x| x.0 ).map(|x| x.clone()).collect()
    }
    res[0].1
}


#[cfg(test)]
mod test{
    use crate::{part1, part2};

    #[test]
    fn test_part1(){
        assert_eq!(part1(5), 3);
    }

    #[test]
    fn test_part2(){
        assert_eq!(part2(5), 2);
    }

}
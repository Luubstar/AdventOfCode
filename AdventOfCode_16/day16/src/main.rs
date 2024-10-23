use std::time::Instant;

fn main() {
    let input = "01110110101001000";
    let entrada = input.chars().into_iter().map(|x| x == '1').collect::<Vec<bool>>();

    let t = Instant::now();
    let r1 = solve(parse(entrada.clone(), 272));
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = solve(parse(entrada.clone(), 35651584));
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}
fn solve(input : Vec<bool>) -> String{
    let res = input.chunks(2).map(|x| x[0] == x[1])
        .collect::<Vec<bool>>();

    if res.len() % 2 == 0{
        solve(res)
    }
    else{
        res.iter().map(|x| if *x {"1"} else {"0"}).collect::<String>()
    }
}

fn parse(input : Vec<bool>, size: usize) -> Vec<bool>{
    let mut res: Vec<bool> = input;
    while res.len() < size  {
        let mut b = res.clone().iter().rev().map(|x| !x).collect::<Vec<bool>>();
        b.insert(0, false);
        res.append(&mut b);
    }
    res.iter().fold(Vec::new(), |mut acc, x| if acc.len() < size {acc.push(*x); acc} else {acc})
}


#[cfg(test)]
mod test{
    use crate::{parse};

    #[test]
    fn test_parse(){
        let mut res = vec![true].repeat(5);
        res.append(&mut vec![false].repeat(6));
        assert_eq!(parse(vec![true].repeat(5), 11), res);
    }

}
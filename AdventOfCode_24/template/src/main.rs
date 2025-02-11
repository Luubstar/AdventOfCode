use std::fs;
use std::time::Instant;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let input = parse(&text);
    println!("Datos parseados en {} ms", t.elapsed().as_millis());

    let t = Instant::now();
    let r1 = part1(&input);
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(&input);
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part1(_input : &str) -> String{
    String::from("NOT IMPLEMENTED")
}

fn part2(_input : &str) -> String{
    String::from("NOT IMPLEMENTED")
}

fn parse(_input : &str) -> String{
    String::from("NOT IMPLEMENTED")
}


#[cfg(test)]
mod test{

    #[test]
    fn test_parse(){
    }

    #[test]
    fn test_part1(){
    }

    #[test]
    fn test_part2(){
    }

}
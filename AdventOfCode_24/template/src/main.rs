use std::fs;
use std::time::Instant;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let r1 = part1(parse(&text));
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(parse(&text));
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(_input : String) -> String{
    String::from("NOT IMPLEMENTED")
}

fn part1(_input : String) -> String{
    String::from("NOT IMPLEMENTED")
}

fn parse(_input : &String) -> String{
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
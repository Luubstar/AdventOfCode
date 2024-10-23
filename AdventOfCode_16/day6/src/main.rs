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

fn part2(input : Vec<&str>) -> String{
    let mut vec = vec![vec![-1; 26]; input.first().unwrap().len()];

    let base = 'a' as u8;
    for texto in input {
        for (i, caracter) in texto.to_ascii_lowercase().chars().enumerate() {
            let charvalue = (caracter as u8 - base) as usize;
            vec[i][charvalue] += 1;
        }
    }

    vec.into_iter()
        .map(|letra| {
            letra.into_iter()
                .enumerate()
                .filter(|&(_, v)| v != -1)
                .min_by_key(|&(_, count)| count)
                .map(|(i, _)| (i as u8 + base) as char)
                .unwrap()
        })
        .collect()
}

fn part1(input: Vec<&str>) -> String{
    let mut vec = vec![vec![0; 26]; input.first().unwrap().len()];

    let base = 'a' as u8;
    for texto in input {
        for (i, caracter) in texto.to_ascii_lowercase().chars().enumerate() {
            let charvalue = (caracter as u8 - base) as usize;
            vec[i][charvalue] += 1;
        }
    }

    vec.into_iter()
        .map(|letra| {
            letra.into_iter()
                .enumerate()
                .max_by_key(|&(_, count)| count)
                .map(|(i, _)| (i as u8 + base) as char)
                .unwrap()
        })
        .collect()
}

fn parse(texto: &String) -> Vec<&str>{
    texto.lines().into_iter().collect()
}


#[cfg(test)]
mod test{
    use std::fs;
    use crate::{parse, part1, part2};

    #[test]
    fn test_parse(){
        let texto = fs::read_to_string("test.txt").unwrap();
        assert_eq!(parse(&texto), vec!["eedadn", "drvtee", "eandsr", "raavrd", "atevrs", "tsrnev", "sdttsa",
                                       "rasrtv", "nssdts", "ntnada", "svetve", "tesnvt", "vntsnd", "vrdear",
                                       "dvrsen", "enarar"]);
    }

    #[test]
    fn test_part1(){
        let texto = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(parse(&texto)), "easter".to_string());
    }

    #[test]
    fn test_part2(){
        let texto = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part2(parse(&texto)), "advent".to_string());
    }

}
use std::fs;
use std::time::Instant;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let r1 = part1(parse(&text));
    println!("Resultado de la parte 1: {}\nObtenida en {} ms",r1.1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2: Vec<String> = part2(parse2(&text));
    let r2: Vec<String> = r2.into_iter().filter(|x| x.to_ascii_lowercase().contains("pole")).collect();
    println!("Resultado de la parte 2: {:#?}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(input: Vec<(&str, u32)>) -> Vec<String>{
    let mut s = Vec::new();

    for (t, i) in input{
        let base = 'a' as u8;
        let t:String = t.chars().map(|x|{
            if x == '-'{return ' '}
            (((x as u8 - base) as u32 + i).rem_euclid(26) as u8 + base) as char
        }).collect::<Vec<char>>().iter().collect();
        s.push(format!("{} - {}", t, i));
    }
    s
}

fn part1(entradas: Vec<(String, u32, &str)>) -> (Vec<(String, u32)>, u32){
    let mut valids: Vec<(String, u32)> = Vec::new();
    let mut v = 0;
    for (s, i, h) in entradas {
        let mut seen = std::collections::HashSet::new();
        let unique_chars: String = s.chars()
            .filter(|c| seen.insert(*c))
            .collect();

        let slice = &unique_chars[0..std::cmp::min(5, unique_chars.len())];

        if slice == h{v += i;}
        valids.push((s,i));
    }
    (valids, v)
}

fn parse(texto: &String) -> Vec<(String, u32, &str)> {
    texto.lines().map(
        |x| {
            let hash = &x[x.find("[").unwrap() + 1..x.find("]").unwrap()];
            let string: Vec<&str> = x[0..x.find("[").unwrap()].split("-").collect();
            let mut chars:Vec<char> = string[..string.len()-1].join("").chars().collect::<Vec<char>>();
            chars.sort();

            let mut groups: Vec<String> = chars.iter().fold(Vec::new(), |mut acc, &c| {
                if let Some(last) = acc.last_mut() {
                    if last.chars().next().unwrap() == c {
                        last.push(c);
                    } else {
                        acc.push(c.to_string());
                    }
                } else {
                    acc.push(c.to_string());
                }
                acc
            });

            groups.sort_by(|a, b| b.len().cmp(&a.len()));

            (groups.join(""), string.last().unwrap().parse().unwrap(), hash)
        }
    ).collect()
}

fn parse2(texto: &String) -> Vec<(&str, u32)> {
    texto.lines().map(
        |x| {
            let string: &str = &x[0..x.find("[").unwrap()];
            let aux = string.split("-").last().unwrap();
            (&string[..string.rfind("-").unwrap()], aux.parse().unwrap())
        }
    ).collect()
}

#[cfg(test)]
mod test{
    use crate::{parse, parse2, part1, part2};

    #[test]
    fn test_parse(){
        assert_eq!(parse(&"a-b-c-123[123]".to_string()), vec![("abc".to_string(), 123, "123")]);
        assert_eq!(parse(&"a-ba-ca-123[123]".to_string()), vec![("aaabc".to_string(), 123, "123")]);
        assert_eq!(parse(&"a-ba-cadddddd-123[123]".to_string()), vec![("ddddddaaabc".to_string(), 123, "123")]);
    }


    #[test]
    fn test_part1(){
        assert_eq!(part1(parse(&"a-b-c-1[abc]".to_string())).1, 1);
        assert_eq!(part1(parse(&"a-bddddd-c-1[dabc]".to_string())).1, 1);
        assert_eq!(part1(parse(&"a-bddddd-c-1[noes]".to_string())).1, 0);
    }

    #[test]
    fn test_part2(){
        assert_eq!(part2(parse2(&"qzmt-zixmtkozy-ivhz-343[]".to_string())), vec!["very encrypted name - 343"]);
    }

}
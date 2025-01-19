use std::fs;
use std::path::absolute;
use std::time::Instant;
use crate::DIRECTION::{Ascend, Descend, NotSet, Setting};

enum DIRECTION{
    NotSet,
    Setting(i16),
    Ascend(i16),
    Descend(i16),
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let r1 = part1(parse(&text));
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(parse(&text));
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(input : Vec<Vec<&str>>) -> u32{
    let mut safe_reports:u32 = 0;

    input.iter().for_each(|x| {
        let mut direction = NotSet;
        let mut is_safe = true;
        let mut has_failed = false;

        for y in x.iter(){
            let value = y.parse::<i16>().expect(&format!("Expected {y} to be number"));
            match direction {
                NotSet => {direction = Setting(value);}
                Setting(last) => {
                    if last < value {direction = Ascend(value);}
                    else {direction = Descend(value);}

                    let dif = (last - value).abs();
                    if dif > 3 || dif < 1 {
                        if has_failed {
                            is_safe = false;
                            break;
                        }
                        has_failed = true;
                        match direction {
                            NotSet => {}
                            Setting(_) => {direction = Setting(last);}
                            Ascend(_) => {direction = Setting(last);}
                            Descend(_) => {direction = Setting(last);}
                        }
                    }
                }
                Ascend(last) => {
                    let dif = (last - value).abs();
                    if value < last || dif > 3 || dif < 1 {
                        if has_failed {
                            is_safe = false;
                            break;
                        }
                        has_failed = true;
                        match direction {
                            NotSet => {}
                            Setting(_) => {direction = Setting(last);}
                            Ascend(_) => {direction = Setting(last);}
                            Descend(_) => {direction = Setting(last);}
                        }
                        continue;
                    }
                    direction = Ascend(value);
                }
                Descend(last) => {
                    let dif = (last - value).abs();
                    if value > last || dif > 3 || dif < 1 {
                        if has_failed {
                            is_safe = false;
                            break;
                        }
                        has_failed = true;
                        match direction {
                            NotSet => {}
                            Setting(_) => {direction = Setting(last);}
                            Ascend(_) => {direction = Setting(last);}
                            Descend(_) => {direction = Setting(last);}
                        }
                        continue;
                    }
                    direction = Descend(value);
                }
            }
        };
        if is_safe {safe_reports += 1;}
    });

    safe_reports
}

fn part1(input : Vec<Vec<&str>>) -> u32{
    let mut safe_reports:u32 = 0;

    input.iter().for_each(|x| {
        let mut direction = NotSet;
        let mut is_safe = true;

        for y in x.iter(){
            let value = y.parse::<i16>().expect(&format!("Expected {y} to be number"));
            match direction {
                NotSet => {direction = Setting(value);}
                Setting(last) => {
                    if last < value {direction = Ascend(value);}
                    else {direction = Descend(value);}

                    let dif = (last - value).abs();
                    if dif > 3 || dif < 1 {
                        is_safe = false;
                        break;
                    }
                }
                Ascend(last) => {
                    let dif = (last - value).abs();
                    if value < last || dif > 3 || dif < 1 {
                        is_safe = false;
                        break;
                    }
                    direction = Ascend(value);
                }
                Descend(last) => {
                    let dif = (last - value).abs();
                    if value > last || dif > 3 || dif < 1 {
                        is_safe = false;
                        break;
                    }
                    direction = Descend(value);
                }
            }
        };
        if is_safe {safe_reports += 1;}
    });

    safe_reports
}

fn parse(input : &str) -> Vec<Vec<&str>>{
    input.lines()
         .collect::<Vec<&str>>()
         .iter()
         .map(|x| x.split_whitespace().collect::<Vec<&str>>())
         .collect()
}


#[cfg(test)]
mod test{
    use crate::{parse, part1, part2};

    #[test]
    fn test_parse(){
        let input =    "7 6 4 2 1
                            1 2 7 8 9";
        assert_eq!(parse(input), vec![vec!["7","6","4","2","1"],vec!["1","2","7","8","9"]]);
    }

    #[test]
    fn test_part1(){
        let input ="7 6 4 2 1
                        1 2 7 8 9
                        9 7 6 2 1
                        1 3 2 4 5
                        8 6 4 4 1
                        1 3 6 7 9";
        assert_eq!(part1(parse(input)), 2);
    }

    #[test]
    fn test_part2(){
        let input ="7 6 4 2 1
                        1 2 7 8 9
                        9 7 6 2 1
                        1 3 2 4 5
                        8 6 4 4 1
                        1 3 6 7 9";
        assert_eq!(part2(parse(input)), 4);
    }

}
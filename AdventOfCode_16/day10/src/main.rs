use std::fs;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Receptor{
    id: i16,
    low: i16,
    high: i16,
    values: Vec<u8>
}

impl PartialEq for Receptor {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Receptor {
    fn new(id : i16) -> Receptor{
        Receptor{id, low:-1, high:-1, values:Vec::new()}
    }
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let bots = parse(&text);
    let r1 = part1(&mut bots.clone(), (17,61));
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(&mut bots.clone());
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(input: &mut Vec<Receptor>) -> i32{
    loop {
        let mut transfers = Vec::new();

        for bot in &mut *input {
            if bot.id < 0 {
                continue;
            }

            if bot.values.len() == 2 {
                bot.values.sort();

                transfers.push((bot.low, bot.values[0]));
                transfers.push((bot.high, bot.values[1]));

                bot.values.clear();
            }
        }

        if transfers.len() == 0 {break;}
        for (bot_id, value) in transfers {
            input.iter_mut().find(|b| b.id == bot_id).unwrap().values.push(value);
        }
    }
    let o0 = input.iter().find(|b| b.id == -1).unwrap();
    let o1 = input.iter().find(|b| b.id == -2).unwrap();
    let o2 = input.iter().find(|b| b.id == -3).unwrap();

    (o0.values[0] as i32) * (o1.values[0] as i32) * (o2.values[0] as i32)
}

fn part1(input: &mut Vec<Receptor>, compare: (u8, u8)) -> i16 {
    loop {
        let mut transfers = Vec::new();

        for bot in &mut *input {
            if bot.id < 0 {
                continue;
            }

            if bot.values.len() == 2 {
                bot.values.sort();

                if (bot.values[0], bot.values[1]) == compare {
                    return bot.id - 1;
                }

                transfers.push((bot.low, bot.values[0]));
                transfers.push((bot.high, bot.values[1]));

                bot.values.clear();
            }
        }

        for (bot_id, value) in transfers {
            input.iter_mut().find(|b| b.id == bot_id).unwrap().values.push(value);
        }
    }
}

fn parse(input : &String) -> Vec<Receptor>{
    let mut bots: Vec<Receptor> = Vec::new();
    for linea in input.lines(){
        let espacios = linea.split_whitespace().collect::<Vec<&str>>();

        if espacios[0].starts_with("value"){
            let chip = espacios[1].parse().unwrap();
            let bot = get_bot_id(&espacios, 5, &mut bots);
            bot.values.push(chip);
        }
        else{
            let low = get_bot_id(&espacios, 6, &mut bots).id;
            let high = get_bot_id(&espacios, 11, &mut bots).id;
            let bot = get_bot_id(&espacios, 1, &mut bots);
            bot.low = low;
            bot.high = high;
        }
    }
    bots
}

fn get_bot_id<'a>(espacios: &'a Vec<&'a str>, pos: usize, bots: &'a mut Vec<Receptor>) -> &'a mut Receptor {
    let isbot = if espacios[pos - 1] == "bot" {1} else {-1};
    let id = (espacios[pos].parse::<i16>().unwrap() + 1) * isbot;

    if bots.contains(&Receptor::new(id)){
        bots.iter_mut().find(|b| b.id == id).unwrap()
    }
    else{
        bots.push(Receptor::new(id));
        bots.last_mut().unwrap()
    }
}

#[cfg(test)]
mod test{
    use std::fs;
    use crate::{parse, part1, part2, Receptor};

    #[test]
    fn test_parse(){
        let text = fs::read_to_string("test.txt").unwrap();
        assert_eq!(parse(&text), vec![
            Receptor{id: 3, low: 2, high: 1, values:vec![5,2]},
            Receptor{id: 2, low: -2, high: 1, values:vec![3]},
            Receptor{id: 1, low: -3, high: -1, values:vec![]},
            Receptor{id: -2, low: -1, high: -1, values:vec![]},
            Receptor{id: -3, low: -1, high: -1, values:vec![]},
            Receptor{id: -1, low: -1, high: -1, values:vec![]},

        ])
    }

    #[test]
    fn test_part1(){
        let text = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(&mut parse(&text), (2, 5)), 2);
    }

    #[test]
    fn test_part2(){
        let text = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part2(&mut parse(&text)), 3*5*2);
    }

}
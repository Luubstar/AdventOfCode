use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{fs};
use std::time::Instant;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Item{
    elemento: String,
    pos: u8,
    generador: bool
}

impl Item{
    fn new(elemento: String, pos: u8, generador: bool) -> Item{
        Item{
            elemento, pos, generador
        }
    }
}

#[derive(Clone, Debug, Hash)]
struct State{
    pasos: usize,
    ascensor: u8,
    f_score: usize,
    items: Vec<Item>,
    signature: Vec<(u8,u8)>
}

impl State {
    fn heuristic(&self) -> usize {
        self.items.iter().map(|item| (item.pos as isize - 4).abs() as usize).sum()
    }

    fn f_score(&self) -> usize {
        self.pasos + self.heuristic()
    }

    fn gen_signature(&mut self){
        self.signature = vec![(0, 0); 4];
        for item in &self.items {
            let pos_index = (item.pos - 1) as usize; // Convertir a índice 0
            if item.generador {
                self.signature[pos_index].0 += 1; // Contar generadores
            } else {
                self.signature[pos_index].1 += 1; // Contar microchips
            }
        }
    }
}

// Para mantener el estado en la cola de prioridad
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_score().cmp(&other.f_score()).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl PartialEq for State{
    fn eq(&self, other: &Self) -> bool {
        self.signature == other.signature && self.ascensor == other.ascensor
    }
}

impl Eq for State{}

fn main() {
    let text = fs::read_to_string("input_1.txt").unwrap();

    let t = Instant::now();
    let r1 = part1_a(parse(&text));
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1.expect(" NO SE ENCONTRÓ RESULTADO "), t.elapsed().as_millis());

    let text = fs::read_to_string("input_2.txt").unwrap();
    let t = Instant::now();
    let r1 = part1_a(parse(&text));
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r1.expect(" NO SE ENCONTRÓ RESULTADO "), t.elapsed().as_millis());

}

fn part1_a(input: (u8, Vec<Item>)) -> Option<usize>{
    let mut open: BinaryHeap<State> = BinaryHeap::new();
    let mut close: HashSet<State> = HashSet::new();
    let mut parent_map: HashMap<(u8, Vec<Item>), Option<(u8, Vec<Item>)>> = HashMap::new();
    let mut g_score: HashMap<(u8, Vec<Item>), usize> = HashMap::new();

    let mut initial_state = State { ascensor: input.0, pasos: 0, items: input.1.clone(), f_score: usize::MAX, signature: Vec::new() };
    initial_state.gen_signature();
    open.push(initial_state.clone());
    parent_map.insert((input.0, input.1.clone()), None);
    g_score.insert((input.0, input.1.clone()), 0);

    while let Some(state) = open.pop() {
        if goal(&(state.ascensor, state.items.clone())) {
            println!("{:?}", state);
            return Some(state.pasos);
        }

        close.insert(state.clone());

        let new_states = generate_new_states(&(state.ascensor, state.items.clone()));
        for new_state in new_states {
            let mut check = State { pasos: 0, ascensor: new_state.0, items: new_state.1.clone(), f_score: 0, signature: Vec::new() };
            check.gen_signature();
            if close.contains(&check) {
                continue;
            }

            let new_g = g_score[&(state.ascensor, state.items.clone())] + 1; // O costo de mover a un nuevo estado
            let state_tuple = (new_state.0, new_state.1.clone());

            if !open.iter().any(|s| s.ascensor == new_state.0 && s.items == new_state.1.clone()) || new_g < *g_score.get(&state_tuple).unwrap_or(&usize::MAX) {
                parent_map.insert(state_tuple.clone(), Some((state.ascensor, state.items.clone())));
                g_score.insert(state_tuple.clone(), new_g);

                let mut state_with_score = State{pasos: new_g, ascensor:new_state.0, f_score:0, items:new_state.1.clone(), signature: Vec::new()};
                state_with_score.gen_signature();
                state_with_score.f_score = state_with_score.f_score(); // Aquí, asegúrate de calcular el f_score correcto
                open.push(state_with_score);
            }
        }
    }
    None
}

fn generate_new_states(state: &(u8, Vec<Item>)) -> Vec<(u8, Vec<Item>)> {
    let mut new_states = Vec::new();
    let current_floor = state.0;
    let items_on_floor: Vec<usize> = state.1.iter()
        .enumerate()
        .filter(|(_, i)|i.pos == current_floor)
        .map(|(i, _)| i)
        .collect();

    for i in 0..items_on_floor.len() {
        for &new_floor in &[current_floor + 1, current_floor - 1] {
            if new_floor >= 1 && new_floor <= 4 {
                let mut new_state = state.clone();
                move_item(&mut new_state, items_on_floor[i], new_floor);
                if possible(&new_state) {
                    new_state.0 = new_floor;
                    new_states.push(new_state);
                }
            }
        }

        for j in i+1..items_on_floor.len() {
            for &new_floor in &[current_floor + 1, current_floor - 1] {
                if new_floor >= 1 && new_floor <= 4 {
                    let mut new_state = state.clone();
                    move_item(&mut new_state, items_on_floor[i], new_floor);
                    move_item(&mut new_state, items_on_floor[j], new_floor);
                    if possible(&new_state) {
                        new_state.0 = new_floor;
                        new_states.push(new_state);
                    }
                }
            }
        }
    }
    new_states
}

fn move_item(state: &mut (u8, Vec<Item>), item_index: usize, new_floor: u8) {
    let item = &mut state.1[item_index];
    if item.pos == state.0 {
        item.pos = new_floor;
    }
}

fn goal(state: &(u8, Vec<Item>)) -> bool{
    state.0 == 4 && state.1.iter().all(|i|  i.pos == 4)
}

fn possible(state: &(u8, Vec<Item>)) -> bool{
    for i in &state.1 {
        let c = &state.1.clone();
        let items = c.iter().filter(|x| x.pos == i.pos).collect::<Vec<&Item>>();
        if items.iter().any(|x| x.elemento != i.elemento && x.generador && !i.generador
            && !items.iter().any(|y| y.elemento == i.elemento && y.generador)
        ){
            return false;
        }
    }
    true
}



fn part1(input: (u8, Vec<Item>)) -> Option<usize> {
    let mut open: VecDeque<(u8, Vec<Item>)> = VecDeque::new();
    let mut close: HashSet<(u8, Vec<Item>)> = HashSet::new();
    let mut parent_map: HashMap<(u8, Vec<Item>), Option<(u8, Vec<Item>)>> = HashMap::new();

    open.push_back(input.clone());
    parent_map.insert(input, None);

    while let Some(state) = open.pop_front(){
        if goal(&state) {
            println!("{:?}", state);
            let mut path = 0;
            let mut step = Some(state.clone());
            while let Some(node) = step {
                path += 1;
                step = parent_map[&node].clone();
            }
            return Some((path as usize) -1)
        }
        close.insert(state.clone());

        let new_states = generate_new_states(&state);
        for new_state in new_states {
            if !close.contains(&new_state) && !open.contains(&new_state){
                parent_map.insert(new_state.clone(), Some(state.clone()));
                open.push_back(new_state);
            }
        }
    }
    None
}
fn parse(input : &String) -> (u8, Vec<Item>){
    let mut res: Vec<Item> = Vec::new();
    let mut i:u8 = 1;
    for line in input.lines(){
        let espacios = line.split_whitespace().collect::<Vec<&str>>();

        if espacios[4] == "a"{extract_content(&mut res, i, espacios);}

        i += 1;
    }
    (1, res)
}

fn extract_content(res: &mut Vec<Item>, piso: u8, espacios: Vec<&str>) {
    let mut pos = 5;
    while pos < espacios.len(){
        let element;
        let generador;
        if espacios[pos].contains("-"){
            element = espacios[pos].split("-").collect::<Vec<&str>>()[0];
            generador = false;
        }
        else{
            element = espacios[pos];
            generador = true;
        }

        res.push(Item::new(element.to_string(), piso, generador));
        pos += 3;
        if pos < espacios.len() && espacios[pos] == "a"{pos+=1;}
    }
}

#[cfg(test)]
mod test{
    use std::fs;
    use crate::{generate_new_states, parse, part1, part1_a, Item};

    #[test]
    fn test_parse(){
        let text = fs::read_to_string("test.txt").unwrap();
        assert_eq!(parse(&text), (1, vec![
            Item::new("hydrogen".to_string(), 1, false),
            Item::new("lithium".to_string(), 1, false),
            Item::new("hydrogen".to_string(), 2, true),
            Item::new("lithium".to_string(), 3, true),

        ]));
    }

    #[test]
    fn test_gen_posiciones(){
        let text = fs::read_to_string("test.txt").unwrap();

        assert_eq!(generate_new_states(&(2, vec![Item::new("".to_string(), 2, false)])),
                   vec![
                       (3, vec![Item::new("".to_string(), 3, false)]),
                       (1, vec![Item::new("".to_string(), 1, false)])]);

        assert_eq!(generate_new_states(&(2, vec![Item::new("l".to_string(), 2, false),
        Item::new("h".to_string(), 1, true)])),
                   vec![
                       (3, vec![Item::new("l".to_string(), 3, false),Item::new("h".to_string(), 1, true)])]);


        assert_eq!(generate_new_states(&(2, vec![Item::new("l".to_string(), 2, false),
                                                 Item::new("h".to_string(), 2, false)])),
                   vec![
                       (3, vec![Item::new("l".to_string(), 3, false),Item::new("h".to_string(), 2, false)]),
                       (1, vec![Item::new("l".to_string(), 1, false),Item::new("h".to_string(), 2, false)]),
                       (3, vec![Item::new("l".to_string(), 3, false),Item::new("h".to_string(), 3, false)]),
                       (1, vec![Item::new("l".to_string(), 1, false),Item::new("h".to_string(), 1, false)]),
                       (3, vec![Item::new("l".to_string(), 2, false),Item::new("h".to_string(), 3, false)]),
                       (1, vec![Item::new("l".to_string(), 2, false),Item::new("h".to_string(), 1, false)]),
                   ]
        );

        assert_eq!(generate_new_states(&(1, vec![
            Item::new("l".to_string(), 2, false),Item::new("l".to_string(), 2, true),
            Item::new("h".to_string(), 1, false),Item::new("h".to_string(), 1, true)])),
                   vec![(2, vec![
                       Item::new("l".to_string(), 2, false),Item::new("l".to_string(), 2, true),
                       Item::new("h".to_string(), 2, false),Item::new("h".to_string(), 2, true)]),
                        (2, vec![
                            Item::new("l".to_string(), 2, false),Item::new("l".to_string(), 2, true),
                            Item::new("h".to_string(), 1, false),Item::new("h".to_string(), 2, true)]),
                   ]
        )
    }

    #[test]
    fn test_part1(){
        let text = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1_a(parse(&text)), Some(11));
    }

}
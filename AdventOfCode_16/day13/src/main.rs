use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::time::Instant;

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
struct Node{
    position: (usize, usize),
    distance: usize,
    cost: usize
}

impl Node{
    fn get_orthogonal(self, goal: (usize, usize)) -> Vec<Node>{
        let mut res = Vec::new();
        let (x,y) = self.position;

        if x > 0 {res.push(Node::new((x-1, y), self.distance +1, goal));}
        if y > 0 {res.push(Node::new((x, y-1), self.distance +1, goal));}

        res.push(Node::new((x+1, y), self.distance +1, goal));
        res.push(Node::new((x, y+1), self.distance +1, goal));

        res
    }

    fn new(position: (usize, usize), distance: usize, goal: (usize, usize)) -> Node{
        let cost = Node::heuristic(position, goal);
        Node{position, distance, cost:cost as usize + distance}
    }

    fn heuristic(position: (usize, usize), goal: (usize, usize)) -> isize{
        (position.0 as isize - goal.0 as isize).abs()
            + (position.1 as isize - goal.1 as isize).abs()
    }

}

impl PartialOrd for Node{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node{
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

fn main() {
    let t = Instant::now();
    let r1 = part1(1350, (31,39));
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(1350);
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(input: u16) -> usize {
    let mut unvisited: VecDeque<Node> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let start = Node::new((1,1), 0, (0,0));
    unvisited.push_front(start);

    loop{
        let mut step:VecDeque<Node> = VecDeque::new();
        while let Some(nodo) = unvisited.pop_front() {
            let posibles_nodos = nodo.get_orthogonal((0,0));
            for new_nodo in posibles_nodos {
                if !visited.contains(&new_nodo.position) && posible(new_nodo.position, input) {
                    step.push_back(new_nodo);
                }
            }
            visited.insert(nodo.position);
        }
        let mut step = step.iter().filter(|x| x.distance < 51).map(|x| x.clone()).collect::<VecDeque<Node>>();
        if step.is_empty(){return visited.len();}
        std::mem::swap(&mut step, &mut unvisited);
    }
}


fn part1(input : u16, dest: (usize, usize)) -> usize{
    let mut unvisited: BinaryHeap<Node> = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let start = Node::new((1,1), 0, dest);
    unvisited.push(start);

    while let Some(nodo) = unvisited.pop(){
        if nodo.position == dest{
            return nodo.distance;
        }

        let posibles_nodos = nodo.get_orthogonal(dest);

        for new_nodo in posibles_nodos{
            if !visited.contains(&new_nodo.position) && posible(new_nodo.position, input){
                unvisited.push(new_nodo);
            }
        }

        visited.insert(nodo.position);
    }
    0
}

fn posible((x,y): (usize, usize), seed: u16) -> bool{
    let value = x*x + 3*x + 2*x*y + y + y*y;
    let total = value + seed as usize;
    total.count_ones() %2 == 0
}


#[cfg(test)]
mod test{
    use crate::part1;

    #[test]
    fn test_part1(){
        assert_eq!(part1(10, (7,4)), 11);
    }

    #[test]
    fn test_part2(){
        unimplemented!();
    }

}
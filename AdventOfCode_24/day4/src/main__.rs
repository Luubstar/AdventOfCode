use std::fs;
use std::time::Instant;

type Grid = Vec<Vec<char>>;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let input = parse(&text);
    let r1 = part1(&input, "XMAS");
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(&input, "MAS");
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(input : &Grid, word : &str) -> u32{
    let reverse_word = &word.chars().into_iter().rev().collect::<String>();
    let (dleft, drigth) = get_diagonals(&input);
    count_xmas(&dleft, &drigth, &word, &reverse_word) 
}

fn part1(input : &Grid, word: &str) -> u32{
    let mut count = 0;
    let transposed = transpose(&input);
    let (dleft, drigth) = get_diagonals(&input);
    let reverse_word = &word.chars().into_iter().rev().collect::<String>();

    count += find_in_line(&input, &word);
    count += find_in_line(&input, &reverse_word); 
    count += find_in_line(&transposed, &word);
    count += find_in_line(&transposed, &reverse_word);
    count += find_in_line(&dleft, word);
    count += find_in_line(&dleft, &reverse_word);
    count += find_in_line(&drigth, word);
    count += find_in_line(&dleft, &reverse_word);

    count
}

fn show(input : &Grid){
    println!("----");
    for line in input{
        println!("{:?}", line.iter().collect::<String>());
    }
    println!("----");
}

fn find_in_line(grid :&Grid, word: &str) -> u32{
    let mut count = 0;
    for line in grid{
        count += line.iter().collect::<String>().matches(word).count() as u32;
    }
    count
}   

fn get_diagonals(grid: &Grid) -> (Grid, Grid){
    let rows = grid.len();
    let cols = grid.first().unwrap().len();
    let mut diag_lr = vec![Vec::new(); rows + cols - 1];
    let mut diag_rl = vec![Vec::new(); rows + cols - 1];

    for (i, row) in grid.iter().enumerate(){
        for (j, c) in row.iter().enumerate(){
            diag_lr[i+j].push(*c);
            diag_rl[rows-1 + j - i].push(*c);
        }
    }

    (diag_lr, diag_rl)
}

fn transpose(grid : &Grid) -> Grid{
    let cols = grid.first().unwrap().len();
    let rows = grid.len();
    let mut new_grid = Vec::with_capacity(cols);

    for i in 0..grid[0].len(){
        let mut new_line = Vec::with_capacity(rows);

        for line in grid{
            new_line.push(line.get(i).unwrap().clone());
        }
        new_grid.push(new_line)
    }    

    new_grid
}

fn find_positions(input : &Grid, word: &str) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    for (i,line) in input.iter().enumerate(){
        let line_str:String = line.iter().collect();
        for (j, _) in line_str.match_indices(word){
            positions.push((i,j));
        }
    }
    
    positions
} 

fn count_xmas(diag_lr: &Grid, diag_rl: &Grid, word: &str, reverse : &str) -> u32 {
    let mut mas_lr = find_positions(diag_lr, word);
    let mut mas_rl = find_positions(diag_rl, word);

    mas_lr.append(&mut find_positions(diag_lr, reverse));
    mas_rl.append(&mut find_positions(diag_rl, reverse));
    let mut count = 0;

    for &(i_lr, j_lr) in &mas_lr {
        for &(i_rl, j_rl) in &mas_rl {
            if i_lr + 1 == i_rl + 1 && j_lr + 1 == j_rl + 1 {
                count += 1;

                dbg!(i_lr+1, j_lr+1);
            }
        }
    }

    count
}

fn parse(input : &str) -> Grid{
    let mut res = Vec::new();
    for line in input.lines(){
        res.push(line.chars().collect::<Vec<char>>());
    }
    res
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
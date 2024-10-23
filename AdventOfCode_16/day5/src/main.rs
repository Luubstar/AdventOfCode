use std::sync::{Arc, Mutex};
use crossbeam::thread;
use std::time::Instant;
use md5::{Md5, Digest};

fn main() {
    let t = Instant::now();
    let r1 = part1("reyedfim".to_string());
    println!("Resultado de la parte 1: {}\nObtenida en {} ms",r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2("reyedfim".to_string());
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(input : String, threads : i32) -> String{
    // CHATGPT me ayudó con esta función tras el resultado. Así se hace multihilo. Un dolor de cabeza
    let res = Arc::new(Mutex::new(vec![String::from(""); 8]));  // Vector compartido protegido por Mutex
    let mut index = 0;

    // Creamos un pool de hilos usando crossbeam
    thread::scope(|s| {
        // Número de hilos que quieres usar (ajústalo según tu caso)
        let num_threads = threads;

        // Lanzamos varios hilos para procesar en paralelo
        for _ in 0..num_threads {
            let res = Arc::clone(&res); // Clonamos la referencia al vector compartido
            let string = input.clone();
            s.spawn(move |_| {
                let mut hasher = Md5::new();
                loop {
                    let idx = {
                        // Bloque para tomar un índice y luego liberar el Mutex
                        let res_guard = res.lock().unwrap();
                        if res_guard.iter().all(|x| x != "") {
                            break; // Si ya está completo, salimos del bucle
                        }
                        let curr_index = index;
                        index += 1;
                        curr_index
                    };

                    hasher.update(format!("{}{idx}", string).as_bytes());
                    let result = hasher.finalize_reset();
                    let hex_string = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

                    if hex_string.starts_with("00000") {
                        if let Ok(pos) = hex_string.chars().nth(5).unwrap().to_string().parse::<usize>() {
                            if pos < 8 {
                                let char = hex_string.chars().nth(6).unwrap().to_string();
                                let mut res_guard = res.lock().unwrap(); // Volvemos a bloquear el acceso al vector
                                if res_guard[pos].is_empty() {
                                    res_guard[pos] = char;
                                }
                            }
                        }
                    }
                }
            });
        }
    }).unwrap(); // Terminamos el pool de hilos y esperamos a que todos terminen

    // Devolvemos el resultado una vez que el vector esté completo
    let res_guard = res.lock().unwrap();
    res_guard.join("")
}

fn part1(input : String) -> String{
    let mut res = String::new();
    let mut hasher = Md5::new();
    let mut index = 0;

    while res.len() < 8{
        hasher.update(format!("{input}{index}").as_bytes());
        let result = hasher.finalize_reset();
        let hex_string = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

        if hex_string.starts_with("00000") {
            res.push(hex_string.chars().nth(5).unwrap());
        }

        index += 1;
    }

    res
}

#[cfg(test)]
mod test{
    use crate::{part1, part2};

    /*#[test]
    fn test_part1(){
        assert_eq!(part1("abc".to_string()), "18f47a30".to_string());
    }

    #[test]
    fn test_part2(){
        assert_eq!(part2("abc".to_string()), "05ace8e3".to_string());
    }*/
}
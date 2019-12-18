mod pipe;
mod loader;

use pipe::Pipe;
use loader::load_pipes;
use std::env;


fn main() {
    let mut field = vec![vec![]];
    let mut pipes: Vec<Pipe> = Vec::new();

    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];


    load_pipes(file_name, &mut pipes, &mut field);
    pipes.sort_by(|a,b| b.total_len.cmp(&a.total_len).reverse());

    for mut pipe in pipes {
        println!("{}",pipe.to_string());
        if !pipe.find_path(&mut field)
        {
            println!("Could not solve pipe number {}", pipe.pipe_number);
        }
    }
    
    for line in field {
        for space in line {
            if space==0 {
                print!(" ");
            } else {
                print!("{}",space )
            }
        }
        println!("");
    }

}


extern crate cat_wannabe;

use std::fs::File;
use std::env;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    // pega argumentos
    // para cada argumento abre arquivo
    // para cada arquivo printa conteúdo

    if let Some(file_name) = args.get(1) {
        match File::open(file_name) {
            Ok(mut file) => {
                let mut file_content = String::new();

                match file.read_to_string(&mut file_content) {
                   Ok(_) => print!("{}", file_content),
                   Err(err) => println!("{}", err),
                };
            },
            Err(err) => println!("{}", err),
        }
    } else {
        println!("Should pass file name as second argument");
    }
}

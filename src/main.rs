use std::fs::{OpenOptions};
use std::io::prelude::*;
use std::fs::{File};
use std::path::Path;

//important note about rust file handling
// https://github.com/lukaslueg/built/issues/14

//Rust only reads files in the root directory of the project, ie: not in src, but in the folder itself,so if it's blah/src, it will try to read the file in blah, unless specified.

fn count_lines(arquivo: &mut File) -> i32 {
    let mut count = 0;
    let mut content = String::new();
    arquivo.read_to_string(&mut content).unwrap();
    for c in content.chars() {
        if c == '\n' {
            count += 1;
        }
    }
    count
}


fn main() {
    //note on File::open -- it is a read-only mode, so don't expect to write a file with it.
    let file_path = Path::new("novoteste.txt");
    if !file_path.exists() {
        File::create(&file_path).expect("Could not create file");
    }

    let mut arquivo = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&file_path)
            .expect("File could not be opened.");
    let mut content = String::new();

    match arquivo.read_to_string(&mut content) {
        Ok(_) => println!("{}", content),
        Err(e)=> panic!("Arquivo não pôde ser aberto, erro {}", e),
    }
    //instead, you must use File::create so that it  creates a new file, and if you want, you can copy the contents here.
    //The Rust Programming Language book suggests using a buffer to copy.


    let new_content = "Você falou pra ela que eu sou louco e canto mal";
    match arquivo.write_all(new_content.as_bytes()) {
        Ok(_) => println!("{}", new_content),
        Err(e) => panic!("Não pude escrever para o arquivo {:?}", e)
    }

    let counter = count_lines(&mut arquivo);

    println!("The number of lines is {}", counter);

}

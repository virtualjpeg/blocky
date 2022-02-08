use read_input::prelude::*;


fn main() {
    let input = input::<String>().get();

    let mut block = String::from("/");
    let mut whitespace = String::from("");
    let subblock = String::from("//");

    for i in 0..input.len() {
        whitespace.push_str("  ");
    }

    for i in 0..(whitespace.len() * 2) + input.len() + 3 { 
        block.push_str(&"/".to_string());
    }
    
    println!("{0}\n{1}{2}{3}{4}{5}\n{6}", block, subblock, whitespace, input, whitespace, subblock, block);
}
mod layout;

use std::collections::HashMap;
use std::io;

fn main() {
    println!("KADABRA!");
    println!("Please enter your text...");
    let mut incorrect_text = String::new();
    io::stdin()
        .read_line(&mut incorrect_text)
        .expect("Failed to read line");
    //println!("Your incorrect text is {incorrect_text}");

    let mut result = String::with_capacity(incorrect_text.len());

    // //Eng to ru
    // for ch in incorrect_text.trim_end().chars() {
    //     let new_ch = if let Some(key) = SimbolToKeysEng.get(&ch) {
    //         *KeysToSimbolRu.get(&key).unwrap()
    //     } else {
    //         ch
    //     };
    //     result.push(new_ch);
    // }
    // println!("Ru is {}", result);
}

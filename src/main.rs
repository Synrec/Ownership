use std::io;

fn main() {
    let mut call_of_dead = String::new();
    let mut call_of_dread = String::new();
    loop{
        println!{"Type some words here!"}
        call_of_dead.clear();
        io::stdin()
            .read_line(&mut call_of_dead)
            .expect("Failed to read input");
        let cow = String::from(call_of_dead.trim().replace(" ", ""));
        if cow == "exit" {
            println!("Exiting...");
            break;
        }
        let num: usize = get_string_data(&cow);
        println!("The text: {} has {} characters.", call_of_dead.trim(), num);
        println!("The text: {} is valid", cow);
        println!("Check first word length? (y = yes)");
        call_of_dread.clear();
        io::stdin()
            .read_line(&mut call_of_dread)
            .expect("Failed to read input");
        if call_of_dread.trim() == "y" {
            let index = get_first_word(&call_of_dead);
            let first_word = &call_of_dead.trim()[0..=index];
            println!("The first word is {}", first_word);
        }
        println!("Continue? (n = no)");
        call_of_dread.clear();
        io::stdin()
            .read_line(&mut call_of_dread)
            .expect("Failed to read input");
        if call_of_dread.trim() == "n"{
            break;
        }
    }
    println!("Thanks for trying this out!");
}

fn get_string_data(text: &String) -> usize{
    text.len()
}

fn get_first_word(text: &String) -> usize{
    let text_bytes = text.as_bytes();
    for(i, &item) in text_bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    text.len()
}
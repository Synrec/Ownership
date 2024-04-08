use std::io;

fn main() {
    loop{
        println!{"Type some words here!"}
        let mut call_of_dead = String::new();
        io::stdin()
            .read_line(&mut call_of_dead)
            .expect("Failed to read input");
        let cow = String::from(call_of_dead.trim().replace(" ", ""));
        if cow == "exit" {
            println!("Exiting...");
            break;
        }
        let num: usize = get_string_data(&cow);
        println!("The text: {} has {} characters.", call_of_dead, num);
        println!("The text: {} is valid", cow);
        let mut call_of_dread = String::new();
        println!("Continue? (n = no)");
        io::stdin()
            .read_line(&mut call_of_dread)
            .expect("Failed to read input");
        let call_of_dread = String::from(call_of_dread.trim());
        if call_of_dread == "n"{
            break;
        }
    }
    println!("Thanks for trying this out!");
}

fn get_string_data(text: &String) -> usize{
    text.len()
}
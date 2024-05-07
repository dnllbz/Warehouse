use crate::input::input_with_prompt;
use crate::warehouse::Warehouse;

mod crypt;
mod input;
mod warehouse;

const PATH: &str = "warehouse";

fn main() {
    let mut operation = String::new();
    loop {
        input_with_prompt(&mut operation, "Input Operation [ADD/GET/EXIT]: ");
        match operation.to_uppercase().trim() {
            "ADD" => add(),
            "GET" => get(),
            "EXIT" => { println!("Goodbye!"); break },
            _ => continue
        }
        println!();
    }
}

fn add() {
    let mut service = String::new();
    let mut password = String::new();
    let mut key = String::new();

    input_with_prompt(&mut service, "Enter Service: ");
    input_with_prompt(&mut password, "Enter Password: ");
    input_with_prompt(&mut key, "Enter WareKEY: ");

    Warehouse::add_service(PATH, &service, &password, &key);
}

fn get() {
    let mut service = String::new();
    let mut key = String::new();

    input_with_prompt(&mut service, "Enter Service: ");
    input_with_prompt(&mut key, "Enter WareKEY: ");

    Warehouse::get_service(PATH, &service, &key);
}
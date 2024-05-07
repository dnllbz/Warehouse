use crate::crypt::Crypt;
use crate::input::print_input;
use crate::warehouse::Warehouse;

mod crypt;
mod input;
mod warehouse;

const PATH: &str = "warehouse";

fn main() {
    loop {
        let mut operation = String::new();
        print_input(&mut operation, "Input Operation [ADD/GET/EXIT]: ");

        match operation.to_uppercase().trim() {
            "ADD" => add(),
            "GET" => get(),
            "EXIT" => { println!("Bye-bye!"); break },
            _ => continue
        }
    }
}

fn add() {
    let mut service = String::new();
    let mut password = String::new();
    let mut key = String::new();

    print_input(&mut service, "Input Service: ");
    print_input(&mut password, "Input Password: ");
    print_input(&mut key, "Input WareKEY: ");

    Warehouse::add_service(PATH, &service, &password, &key);
}

fn get() {
    let mut service = String::new();
    let mut key = String::new();

    print_input(&mut service, "Input Service: ");
    print_input(&mut key, "Input WareKEY: ");

    let c_service = Crypt::crypt(service.trim(), key.trim());
    let map = Warehouse::readmap(PATH);

    if map.contains_key(&c_service) {
        let password = Crypt::decrypt(&map[&c_service], key.trim());
        println!("Password for {}: {}", service.trim(), password);
    } else {
        println!("Service not found");
    }
}

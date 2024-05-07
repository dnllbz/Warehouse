use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::crypt::{crypt, decrypt};
use crate::input::input_with_prompt;

pub struct Warehouse {}

impl Warehouse {
    pub fn readpairs(path: &str) -> Vec<(String, String)> {
        if !Path::new(path).exists() {
            drop(File::create(path).unwrap());
            return Vec::new();
        }

        fs::read_to_string(path).unwrap().lines().map(|x| {
            let pair = x.split_once(':').unwrap();
            (pair.0.to_string(), pair.1.to_string())
        }).collect()
    }

    pub fn readmap(path: &str) -> HashMap<String, String> {
        Self::readpairs(path).into_iter().collect()
    }

    pub fn add_service(path: &str, service: &str, password: &str, key: &str) {
        if !Path::new(path).exists() {
            drop(File::create(path).unwrap());
        }

        let mut file = OpenOptions::new()
            .append(true)
            .open(path)
            .unwrap();

        let c_service = crypt(service.trim(), key.trim());
        let c_password = crypt(password.trim(), key.trim());
        let map = Warehouse::readmap(path);

        if map.contains_key(&c_service) {
            let mut choice = String::new();
            loop {
                input_with_prompt(&mut choice, "Service already written, rewrite? [Y/N]: ");
                match choice.to_uppercase().trim() {
                    "Y" => break,
                    "N" => return,
                    _ => continue
                }
            }
        }
        writeln!(&mut file, "{}:{}", c_service, c_password).unwrap();
        println!("Successfully added service!")
    }

    pub fn get_service(path: &str, service: &str, key: &str) {
        let c_service = crypt(service.trim(), key.trim());
        let map = Warehouse::readmap(path);

        if map.contains_key(&c_service) {
            let password = decrypt(&map[&c_service], key.trim());
            println!("Password for {}: {}", service.trim(), password);
        } else {
            println!("Service not found");
        }
    }
}
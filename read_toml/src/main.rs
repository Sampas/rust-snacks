#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;
use std::io;

#[derive(Debug, Deserialize)]
struct Server {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Etc {
    arr: [[u8; 3]; 2],
    foo: String,
    pi: f64,
}

#[derive(Debug, Deserialize)]
struct Config {
    g_var: u16,
    servers: HashMap<String, Server>,
    etc: Etc,
}

fn read_config(filename: &str) -> std::io::Result<Config> {
    let content = std::fs::read_to_string(filename)?;
    Ok(toml::from_str(&content)?)
}

fn main() -> io::Result<()> {
    let filename = "example.toml";
    let config = read_config(filename)?;
    println!("Configuration: {:#?}", config);
    Ok(())
}

use crate::address::Address;
use std::{
    env,
    io::{Error, ErrorKind},
};

fn help() {
    eprintln!(
        "usage:
minecraft_connector <url> [port]\n"
    );
}

pub fn get_args() -> Result<Address, std::io::Error> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => Ok(Address {
            url: args[1].to_string(),
            port: 25565,
        }),
        3 => Ok(Address{
            url: args[2].to_string(),
            port: 25565
        }),
        _ => {
            help();
            return Err(Error::new(ErrorKind::Other, "Bad Args"))
        }
    }
}

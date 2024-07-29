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
        3 => {
            let port = match args[2].parse::<u16>() {
                Ok(p) => p,
                Err(_) => {
                    help();
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Port must be a valid number",
                    ));
                }
            };
            Ok(Address {
                url: args[1].to_string(),
                port,
            })
        }
        _ => {
            help();
            return Err(Error::new(ErrorKind::Other, "Bad Args"));
        }
    }
}

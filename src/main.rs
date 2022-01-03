use std::{io::Error, net::TcpStream};

mod address;
mod args;
mod buffer;
mod messages;
mod server_status;

use args::get_args;
use messages::send_message;
use server_status::ServerStatus;

// https://wiki.vg/Server_List_Ping
fn main() -> Result<(), Error> {
    let address = get_args()?;

    let adr_str = format!("{}:{}", address.url, &address.port.to_string());
    let stream = TcpStream::connect(&adr_str)?;

    let buff = send_message(&stream, &address)?;
    let utf8_json_string = String::from_utf8_lossy(&buff[..]);
    let v: ServerStatus = serde_json::from_str(&utf8_json_string)?;

    println!();
    println!("Server: {}", v.description.text);
    println!("Players: {}/{}", v.players.online, v.players.max);
    println!();
    // println!("Version:");
    // println!("\tName: {}", v.version.name);
    // println!("\tProtocol: {}", v.version.protocol);
    Ok(())
}

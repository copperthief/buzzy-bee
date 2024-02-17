use std::{net::{TcpListener, TcpStream}, io::Write, io::Read, io::BufReader, io::BufRead};

pub fn request(content : String) {
    let mut connection = TcpStream::connect("your_ip").unwrap();
    connection.write_all(content.as_bytes());
}

pub fn expect() -> String {
    println!("Expecting data from server...");
    let mut s = String::from("");
    let mut connection = TcpStream::connect("your_ip").unwrap();
    let mut speak = "speak".to_owned();
    connection.write_all((speak + "\n").as_bytes());
    let mut reader = BufReader::new(connection);
    reader.read_line(&mut s);
    println!("Recieved data from server: {}", s);
    s
}

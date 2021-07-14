use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

const MAX_LIST: usize = 8192;

fn make_directory(param: &str) -> String {
    match fs::create_dir_all(param) {
        Ok(_) => String::from("Success"),
        Err(err) => err.to_string(),
    }
}

fn get_file_list() -> String {
    let mut listing = String::with_capacity(MAX_LIST);

    for file in fs::read_dir(".").unwrap() {
        let entry = file.unwrap().path().display().to_string();
        listing.push_str(entry.as_str());
    }
    listing
}

fn erase_file(param: &str) -> String {
    match fs::remove_file(param) {
        Ok(_) => String::from("Success"),
        Err(err) => err.to_string(),
    }
}

fn handle_req(mut conn: TcpStream) {
    println!("start handle_req");
    let mut reqbytes = [0; 512];
    let mut response = String::with_capacity(MAX_LIST);

    match conn.write(b"> ") {
        Ok(_) => (),
        Err(err) => println!("Received an error on write! {}", err),
    };

    println!("waiting something to read");
    let requestsize = conn.read(&mut reqbytes);
    let req = String::from_utf8_lossy(&reqbytes);
    let size = requestsize.unwrap();
    let mut request: String = String::from_utf8(reqbytes[..size].to_vec()).unwrap();
    if size > 0 {
        println!("Received: {}", request);
        let mut params = request.split_whitespace();
        let command = params.next().unwrap();
        match command {
            "flist" => {
                println!("flist has been executed!");
                response = get_file_list();
            }
            "ferase" => {
                println!("ferase has been executed!");
                response = erase_file(params.next().unwrap());
            }
            "md" => {
                println!("md has been executed!");
                response = make_directory(params.next().unwrap());
            }
            _ => response = String::from("Unacceptable command"),
        };
        match conn.write(&response.as_bytes()) {
            Ok(_) => (),
            Err(err) => println!("Received an error on write! {}", err),
        };
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3333")?;

    println!("Waiting a request!");
    for stream in listener.incoming() {
        println!("A request has come in!");
        handle_req(stream?);
        println!("waiting");
    }

    Ok(())
}

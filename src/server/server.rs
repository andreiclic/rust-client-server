use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;
use json::object;

fn create_response(client_id: u32, timestamp: u64, request_type: u32, request_body: String) -> Option<String> {
    let response_body;
    
    match request_type {
        0 => {
            response_body = request_body;
        },
        1 => {
            response_body = String::from("config info");
        },
        _ => {
            println!("Unsupported Request-Type ({})", request_type);
            return None;
        }
    }
    
    let response_data = object!{
        "Client-ID": client_id,
        "Timestamp": timestamp,
        "Request-Type": request_type,
        "Response-Body": response_body
    };

    Some(response_data.dump())
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 100]; // using 100 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            if size > 0 {
                let str_data = from_utf8(&data[0..size]).expect("err");
                let parsed_data = json::parse(str_data).unwrap();
                
                let response = create_response(
                    parsed_data["Client-ID"].as_u32().unwrap(), 
                    parsed_data["Timestamp"].as_u64().unwrap(),
                    parsed_data["Request-Type"].as_u32().unwrap(),
                    parsed_data["Request-Body"].to_string());

                match response {
                    Some(resp) => {
                        println!("Sending response {}", resp);
                        stream.write_all(resp.as_bytes()).unwrap();
                    },
                    None => {}
                }
            }

            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();

            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}

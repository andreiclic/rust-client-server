use std::net::{TcpStream};
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::str::from_utf8;
use std::fs;
use std::io;
use json::object;

const CONFIG_FILE: &str = "../../test/client-config.json";

fn init_client() -> (u32, String) {
    match fs::read_to_string(CONFIG_FILE) {
        Ok(config_file_data) => {
            println!("Read {}", CONFIG_FILE);

            let parsed_data = json::parse(&config_file_data).unwrap();

            let client_id: u32 = parsed_data["Client-ID"].as_u32().unwrap();
            let server_info: String = parsed_data["Server"].to_string();
            
            (client_id, server_info)
        },
        Err(e) => {
            println!("Failed to read {}: {}", CONFIG_FILE, e);
            
            (0, String::new())
        }
    }
}

fn create_request(client_id: u32) -> String {
    print!("Enter Request-Type: ");
    io::stdout().flush().unwrap();
    let mut request_type = String::new();
    io::stdin().read_line(&mut request_type).unwrap();
    let request_type: u32 = request_type.trim().parse().unwrap();

    print!("Enter Request-Body: ");
    io::stdout().flush().unwrap();
    let mut request_body = String::new();
    io::stdin().read_line(&mut request_body).unwrap();
    request_body = request_body.trim().to_owned();

    let timestamp: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let request_data = object!{
        "Client-ID": client_id,
        "Timestamp": timestamp,
        "Request-Type": request_type,
        "Request-Body": request_body
    };

    request_data.dump()
}

fn receive_response(mut stream: TcpStream) {
    let mut data = [0 as u8; 100]; // using 100 byte buffer
    match stream.read(&mut data) {
        Ok(size) => {
            if size > 0 {
                let str_data = from_utf8(&data[0..size]).expect("err");
                let parsed_response = json::parse(str_data).unwrap();

                println!("response {}", parsed_response);
            }
        },
        Err(_) => {}
    }
}

fn main() {
    let (client_id, server_info) = init_client();

    let request = create_request(client_id);

    println!("request {}", request);

    match TcpStream::connect(server_info) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            stream.write_all(request.as_bytes()).unwrap();
            println!("Sent request, waiting for response...");

            receive_response(stream);
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    println!("Terminated.");
}

use std::net::{TcpStream};
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
// use std::str::from_utf8;
use std::fs;
use json::object;

const CONFIG_FILE: &str = "../../test/client-config.json";

fn init_client() -> (u32, String) {
    match fs::read_to_string(CONFIG_FILE) {
        Ok(config_file_data) => {
            println!("Read {}", CONFIG_FILE);

            let parsed_data = json::parse(&config_file_data).unwrap();

            let client_id: u32 = parsed_data["Client-ID"].to_string().parse().expect("Client-ID is not a number!");
            let server_info: String = parsed_data["Server"].to_string();
            
            (client_id, server_info)
        },
        Err(e) => {
            println!("Failed to read {}: {}", CONFIG_FILE, e);
            
            (0, String::new())
        }
    }
}

fn create_request(client_id: u32, request_type: u32, request_body: String) -> String {
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

fn main() {
    let (client_id, server_info) = init_client();

    let request = create_request(client_id, 0, String::from("hello"));

    println!("request {}", request);

    match TcpStream::connect(server_info) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            stream.write_all(request.as_bytes()).unwrap();
            println!("Sent Hello, awaiting reply...");

            // let mut data = [0 as u8; 6]; // using 6 byte buffer
            // match stream.read_exact(&mut data) {
            //     Ok(_) => {
            //         if &data == msg {
            //             println!("Reply is ok!");
            //         } else {
            //             let text = from_utf8(&data).unwrap();
            //             println!("Unexpected reply: {}", text);
            //         }
            //     },
            //     Err(e) => {
            //         println!("Failed to receive data: {}", e);
            //     }
            // }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    println!("Terminated.");
}

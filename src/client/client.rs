use std::net::{TcpStream};
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::str::from_utf8;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::env;
use json::object;

struct ClientParams {
    client_id: u32,
    config_server: String,
    request_server: String,
    log_file: String,
}

fn log(log_file: &str, string: &str) {
    let timestamp: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .try_into()
        .unwrap();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file)
        .unwrap();

    if let Err(e) = writeln!(file, "[{}] {}", timestamp.to_string(), string) {
        println!("Failed to write to {}: {}", log_file, e);
    }
}

fn init_client(config_id: &str) -> ClientParams {
    let config_file: String = format!("../../test/client-config{}.json", config_id);

    match fs::read_to_string(&config_file) {
        Ok(config_file_data) => {
            println!("Read config info from {}", config_file);

            let parsed_data = json::parse(&config_file_data).unwrap();

            let client_params = ClientParams {
                client_id: parsed_data["Client-ID"].as_u32().unwrap(),
                config_server: parsed_data["Config-Server"].to_string(),
                request_server: parsed_data["Config-Server"].to_string(),
                log_file: parsed_data["Log-File"].to_string(),
            };
            
            println!("Log file: {}", client_params.log_file);

            client_params
        },
        Err(e) => {
            println!("Failed to read from {}: {}", config_file, e);
            
            ClientParams {
                client_id: 0, 
                config_server: String::new(), 
                request_server: String::new(), 
                log_file: String::new()
            }
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
        .as_millis()
        .try_into()
        .unwrap();

    let request_data = object!{
        "Client-ID": client_id,
        "Timestamp": timestamp,
        "Request-Type": request_type,
        "Request-Body": request_body
    };

    request_data.dump()
}

fn receive_response(mut stream: TcpStream, client_params: &ClientParams) {
    let mut data = [0 as u8; 100]; // using 100 byte buffer
    match stream.read(&mut data) {
        Ok(size) => {
            if size > 0 {
                let str_data = from_utf8(&data[0..size]).expect("err");
                let parsed_response = json::parse(str_data).unwrap();
                
                log(&client_params.log_file, &format!("Received response {}", parsed_response));

                let current_timestamp: u64 = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
                    .try_into()
                    .unwrap();

                let request_timestamp: u64 = parsed_response["Timestamp"].as_u64().unwrap();
                let rtt: u64 = current_timestamp - request_timestamp;

                log(&client_params.log_file, &format!("The previous request took {}ms", rtt));
            }
        },
        Err(_) => {}
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Please provide a client config id as command line argument!");
        return;
    }

    let client_params = init_client(&args[1]);

    log(&client_params.log_file, &format!("client{} finished initialization", client_params.client_id));

    loop {
        let request = create_request(client_params.client_id);

        log(&client_params.log_file, &format!("Sending request {}", request));

        match TcpStream::connect(client_params.request_server.clone()) {
            Ok(mut stream) => {
                log(&client_params.log_file, &format!("Successfully connected to server on port 3333"));

                stream.write_all(request.as_bytes()).unwrap();
                println!("Sent request, waiting for response...");

                receive_response(stream, &client_params);
            },
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    }
}

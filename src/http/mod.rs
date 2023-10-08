use std::io::{BufReader, prelude::*};
use std::time::Duration;
use std::net::TcpStream;
use std::thread;

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = match buf_reader.lines().next() {
        Some(line) => match line {
            Ok(line) => line,
            Err(err) => {
                println!("Failed to read line: {}", err);
                return;
            }
        },
        None => {
            println!("No Http request line present");
            return;
        }
    };

    let (status_code, message) = route_request(&request_line[..]);
    let length = message.len();
    let response = format!("HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n{}", status_code, length, message);
    match stream.write_all(response.as_bytes()) {
        Err(err) => {
            println!("Failed to write the response: {}", err);
        },
        Ok(_) => {
            println!("Successfully written all data");
        }
    };
}

// fn validate_http_request_line(request_line: &str) -> bool {
    
// }


fn route_request(request_line: &str) -> (i32, &str) {
    let (method, path) = parse_request_line(request_line);
    match method {
        HttpMethod::GET => match path {
            "/" => (200, "Hello world"),
            "/sleep" => {
                thread::sleep(Duration::from_secs(5));
                (200, "Slept for 5 sec")
            },
            _ => (404, "Not Found")
        },
        HttpMethod::POST => (404, "Not Found")
    }
}


enum HttpMethod {
    GET,
    POST
}

fn parse_request_line(request_line: &str) -> (HttpMethod, &str) {
    let req: Vec<&str> = request_line.split(" ").collect();
    let method = parse_method(req[0]);    
    return (method, req[1]); 
}

fn parse_method(method_str: &str) -> HttpMethod {
    match method_str {
        "GET" => HttpMethod::GET,
        "POST" => HttpMethod::POST,
        _ => panic!("Invalid method"),
    }
}

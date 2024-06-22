use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
};
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Anurag's HTTP Server!");
    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                
                handle_connection(_stream).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }





        }
    }


}



fn handle_connection(mut stream: TcpStream) -> Result<()> {
    println!("Connection established!");
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let lines: Vec<&str> = request.split("\r\n").collect();
    let tokens: Vec<&str> = lines[0].split(" ").collect();
    match tokens[0] {
        "GET" => {
            if tokens[1] == "/" {
                stream.write(b"HTTP/1.1 200 OK\r\n\r\n")?;
            } else if tokens[1].starts_with("/echo/") {
                let response = tokens[1].replace("/echo/", "");
                stream.write(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", response.len(), response).as_bytes())?;
            } else {
                stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n")?;
            }
        }
        _ => {
            println!("Unknown method: {}", tokens[0])
        }
    }
    Ok(())
}
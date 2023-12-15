//Basic Rust webserver by Caleb Wentworth
// created for COS 350 @ usm
//Included modules
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, 
    thread, fs, string,
    };
//handels 
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader.lines().next().unwrap().unwrap();
    println!("Request: {:#?}", http_request);
    
    let request = http_request.as_str();
    
    //matching code for later expansion for resource locating, just put html files in the project directory.
    
    //Like haskell guards, this returns different responses based on the resource
    // logic for finding resources will come at some point, as of now this is hard coded, as I wanted to understand
    //rust functions, basic networking, and threads a bit.
    let response = match request{
        "GET / HTTP/1.1" => serve("site.html"),
        "GET /secret.zip HTTP/1.1" => download("secret.zip"),
        "GET /servus.zip" => download("servus.zip"),
        _ => serve("404.html"),//in all other cases serve 404.html
    };
    //sends the response
    stream.write_all(&response).unwrap();
}

fn serve(resource: &str) -> Vec<u8> {
    let status = "HTTP/1.1 200 OK\r\n";
    let page = fs::read_to_string(resource).unwrap();
    let length = format!("Content-Length: {}\r\n\r\n", page.len());

    let mut response = Vec::new();
    response.extend(status.as_bytes());
    response.extend(length.as_bytes());
    response.extend(page.as_bytes());

    response
}

fn download(resource: &str) -> Vec<u8> {
    let status = "HTTP/1.1 200 OK\r\n";
    let content_disposition = format!("Content-Disposition: attachment; filename={}\r\n", resource);
    let content_type = "Content-Type: application/zip\r\n"; // for ZIP files
    let file_content = fs::read(resource).unwrap();
    let length = format!("Content-Length: {}\r\n\r\n", file_content.len());

    let mut response = Vec::new();
    response.extend(status.as_bytes());
    response.extend(content_disposition.as_bytes());
    response.extend(content_type.as_bytes());
    response.extend(length.as_bytes());
    response.extend(file_content);

    response
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });

        println!("Connection established!");
    }

}


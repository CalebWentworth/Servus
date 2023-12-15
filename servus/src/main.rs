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
        _ => serve("404.html"),
    };
    //sends the response
    stream.write_all(response.as_bytes()).unwrap();
}

fn serve(resource: &str) -> String{
    let status = "HTTP/1.1 200 OK";
    let page = fs::read_to_string(resource).unwrap();
    let length = page.len();

    let response = format!(
        "{status}\r\nContent-Length: {length}\r\n\r\n{page}"
        );
    return response;
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


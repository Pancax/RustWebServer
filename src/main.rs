use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

fn main() {
    //makes a listener that is a tcp listener and binds it to this port
    //binds works like a new function that returns a tcplistener object
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //binds returns a Result<T,E> - the binding may fail.
    //unwrap just tells rust to stop the program if an error happens

    //incoming method returns an iterator that gives a sequence of TcpStream objects
    //TcpStreams will read from itself to see what the cient send and
    //then allow us to write our response to the stream
    for stream in listener.incoming(){
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get=b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
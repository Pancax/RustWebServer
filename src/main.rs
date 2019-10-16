use std::net::{TcpListener, TcpStream};
use std::io::Read;

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

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
# Multi-Threaded Web Server with Rust

This Project was carried out as a part of the [Rust language course](https://doc.rust-lang.org/book/title-page.html).

## Building a Single Thread Server

The web-server utilizes two principle protocols for client-server interaction, namely the *Hypertext Transfer Protocol (HTTP)* and the *Transmission Control Protocol (TCP)*. In both protocols a client initiates requests and a server listens to the requests and provides a response to the client. 

### Listening to the TCP connection

The `std::net::TcpListener` submodule listens to incoming TCP streams at `127.0.0.1:7878`. By using the `incoming()` function we generate an iterator which has individual TCP stream which represents a single connection from client to server. A connection is a full request and response process in which :  
    *    A client connects to the server.  
    *    The server generates a response.
    *    Finally, the server closes the connection.

```
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}

``` 
Inspecting `TcpStream` shows us what the client sent

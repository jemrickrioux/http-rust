use crate::http::{Request,Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Write,Read};
use std::net::TcpListener;


pub struct Server {
    addr: String,
    //ip: (i64,i64,i64, i64),
    //port: i64
}

fn arr(_a: &[u8; 5]) {

}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }
    pub fn run(self) {
        println!("The server is listening on: {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream,_)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(bytes) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let reponse = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(StatusCode::Ok, Some("<h1>It works !!!</h1>".to_string()))
                                },
                                Err(e)=> {
                                    println!("Failed to parse a request, {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                },
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("failed to send response: {}", e)
                            }
                        }
                        Err(e) => {println!("We failed to read from connection: {}", e)},
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }


    }
} 


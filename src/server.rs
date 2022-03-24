#[allow(unused_variables)]
use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
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

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                },
                                Err(e)=> println!("Failed to parse a request, {}", e)
                            }

                        },
                        Err(e) => {println!("We failed to read from connection: {}", e)},
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }


    }
} 


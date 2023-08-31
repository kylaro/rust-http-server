use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;

pub struct Server {
    address: String,
}

impl Server{
    pub fn new(address: String) -> Self {
        Self {
            address,
        }
    }
    
    pub fn run(self) {
        println!("Attempting to listen on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        println!("Successful bind on {}", self.address);

        loop {
            match listener.accept() {
                Ok((mut stream, address)) => {
                    println!("OK");
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Received a request:\n{}", String::from_utf8_lossy(&buffer));
                            
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => { dbg!(request);},
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }

                        },
                        Err(e) => {println!("Failed to read from connection: {}", e);},
                    }
                    
                },
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                },
            }
            
        }



    }
}

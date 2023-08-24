use std::net::TcpListener;

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


    }
}

fn main() {
    println!("Hello, world!");
    
    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..];    
    let string_borrow: &str = &string;
    let string_literal = "1234";

    dbg!(string_borrow);

    dbg!(&string);
    dbg!(string_slice);
    
    // let server = Server::new("127.0.0.1:8080");
    // server.run();
}


struct Server {
    address: String,
}

impl Server{
    fn new(address: String) -> Self {
        Self {
            address,
        }
    }
    
    fn run(self) {
        
    }

}
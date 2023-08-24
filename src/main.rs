fn main() {
    println!("Hello, world!");

    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;

    
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
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
        println!("Listening on {}", self.address)
    }
}

enum Method{
    GET, 
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

/* Request
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/

/* Response
HTTP/1.1 200 OK\r\n
HEADERS \r\n
BODY
*/
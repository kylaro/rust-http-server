use super::method::Method;


pub struct Request {
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
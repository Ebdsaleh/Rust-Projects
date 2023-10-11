use crate::http::{ParseError,Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
        
    
  
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse a request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    
    ip_with_port:String,
    
}   

impl Server {
    pub fn new(ip_with_port: String) -> Self {
        // the Self key word is an Alias for the name of the Struct
        Self{
            ip_with_port
        }
    }
    pub fn run(self, mut handler: impl Handler) { 
        // using self like this means that this function will take ownership of the struct
        // if don't want the function to take ownership we can use &self or &mut self if we are modifying our struct.
        println!("Server is listening on: {}", self.ip_with_port);
        let listener = TcpListener::bind(&self.ip_with_port).unwrap();

        // infinate loop
        loop {

            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e);
                                    handler.handle_bad_request(&e)
                                    
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e)

                            }
                            

                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(err) => println!("Failed to establish a connection: {}", err),
                
            }
            let result = listener.accept();
            if result.is_err(){
                continue;
            }
            

        }
       
    }
}

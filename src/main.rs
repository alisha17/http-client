extern crate hyper;
use std::io::Read;
use hyper::Client;

fn main() {
    // GET
    fn get_contents(url: &str) -> hyper::Result<String> {
        //Impl:type Result<T> = Result<T, Error>; Result type which can return hyper errors
        let client = Client::new(); //creates a new client request
        let mut response = client.get(url).send()?; //send completes writing request and returns Response
        let mut buf = String::new();
        // try!(response.read_to_string(&mut buf));
        response.read_to_string(&mut buf)?; //Reads till EOF and returns Result
        Ok(buf)
    }

    println!("{}",
             get_contents("http://www.tutorialspoint.com/http/http_requests").unwrap()); // A whole mess of HTML

}

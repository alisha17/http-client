extern crate hyper;
use std::io::Read;
use hyper::Client;
use hyper::Url;

fn main() {

    //CHECK IF VALID URL
    fn check_url(url: &str) -> Result<hyper::Url, &str> {
        let url = match Url::parse(url) {
            Ok(url) => url,
            Err(_) => panic!("Uh oh. Wrong url"),
        };
        Ok(url)
    }

    // GET
    fn get_contents(url: hyper::Url) -> hyper::Result<String> {
        //Impl:type Result<T> = Result<T, Error>; Result type which can return hyper errors
        let client = Client::new(); //creates a new client request
        let mut response = client.get(url).send()?; //send completes writing request and returns Response
        let mut buf = String::new();
        // try!(response.read_to_string(&mut buf));
        response.read_to_string(&mut buf)?; //Reads till EOF and returns Result
        Ok(buf)
    }

    let url = check_url("http://www.tutorialspoint.com/http/http_requests");
    println!("{}", get_contents(url.unwrap()).unwrap());
    //println!("{:?}",get_contents("http://www.tutorialspoint.com/http/http_requests"));

}

// TO DO: check for empty url, modularise code

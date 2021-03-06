extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "yo!")))
    }

	println!("On 3000");
	
    Iron::new(hello_world).http("localhost:3000").unwrap();    
}

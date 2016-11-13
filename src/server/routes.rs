extern crate iron;
extern crate router;

use self::iron::status;
use self::iron::{Iron, Request, Response, IronResult};

pub fn say_hello(req: &mut Request) -> IronResult<Response> {
    println!("Running send_hello handler, URL path: {}", req.url.path().join("/"));
    Ok(Response::with((status::Ok, "This request was routed!")))
}

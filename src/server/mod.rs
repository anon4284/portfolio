extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;

use self::iron::prelude::*;
use self::iron::status;
use self::staticfile::Static;
use self::mount::Mount;
use self::router::Router;

use std::path::Path;

mod routes;


pub fn start() {
    let mut router = Router::new();
    router.get("/hello", routes::say_hello, "hello");
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("public")));
    mount.mount("/api/", router);
    let server = Iron::new(mount).http("localhost:3000").unwrap();
}

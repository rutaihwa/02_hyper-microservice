use hyper::{Body, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

fn main() {
    // Server address
    let addr = ([127,0,0,1], 8080).into();
    
    // Server instance
    let builder = Server::bind(&addr);

    let server = builder.serve(|| {
        service_fn_ok(|_| {
            Response::new(Body::from("This is where it begins"))
        })
    });

    let server = server.map_err(drop);

    hyper::rt::run(server);
}

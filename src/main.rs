use hyper::{Body, Request, Response, Server, Error, Method, StatusCode};
use futures::{Future, future};
use hyper::service::service_fn;

fn main() {
    // Server address
    let addr = ([127,0,0,1], 8080).into();

    // Server instance
    let builder = Server::bind(&addr);

    // Building a server
    let server = builder.serve(|| service_fn(
        microservice_handler
    ));

    // Dealing with errors
    let server = server.map_err(drop);

    // Running the server
    hyper::rt::run(server);
}

// Microservice hander
fn microservice_handler (req: Request<Body>) ->
    impl Future<Item = Response<Body>, Error=Error> {
        {
            match(req.method(), req.uri().path()) {
                (&Method::GET, "/") => {
                    future::ok(
                        Response::new(INDEX.into())
                    )
                },
                _ => {
                    let response = Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::empty())
                        .unwrap();

                    future::ok(response)
                },
            }
        }
    }

// Index
const INDEX: &'static str = r#"
<!doctype html>
<html>
  <head>
    <title>02: Hyper microservice</title>
  </head>
  <body>
    <h3>This is where it all starts.</h3>
  </body>
</html>
"#;

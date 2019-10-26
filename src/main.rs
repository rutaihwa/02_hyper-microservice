use futures::{future, Future};
use hyper::service::service_fn;
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use slab::Slab;
use std::sync::{Arc, Mutex};

// User data
type UserId = u64;
struct UserData;
type UserDb = Arc<Mutex<Slab<UserData>>>;

fn main() {
    // Server address
    let addr = ([127, 0, 0, 1], 8080).into();

    // Server instance
    let builder = Server::bind(&addr);

    // Shared state
    let user_db = Arc::new(Mutex::new(Slab::new()));

    // Building a server
    let server = builder.serve(move || {
        let user_db = user_db.clone();
        service_fn(move |req| microservice_handler(req, &user_db))
    });

    // Dealing with errors
    let server = server.map_err(drop);

    // Running the server
    hyper::rt::run(server);
}

// Microservice hander
fn microservice_handler(
    req: Request<Body>,
    user_db: &UserDb,
) -> impl Future<Item = Response<Body>, Error = Error> {
    {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") => future::ok(Response::new(INDEX.into())),
            _ => {
                let response = Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap();

                future::ok(response)
            }
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

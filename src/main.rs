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
            (method, path) if path.starts_with(USER_PATH) => {
                unimplemented!();
            }
            _ => {
                let response = response_with_code(StatusCode::NOT_FOUND);

                future::ok(response)
            }
        }
    }
}

// Helper
// response_with_code - creates empty responses
fn response_with_code(status_code: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(Body::empty())
        .unwrap()
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

// User path
const USER_PATH: &str = "/user/";

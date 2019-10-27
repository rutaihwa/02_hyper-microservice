use futures::{future, Future};
use hyper::service::service_fn;
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use slab::Slab;
use std::fmt;
use std::sync::{Arc, Mutex};

// User data
type UserId = u64;
struct UserData;
type UserDb = Arc<Mutex<Slab<UserData>>>;

// Display for UserData
impl fmt::Display for UserData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("{}")
    }
}

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
        let response = {
            match (req.method(), req.uri().path()) {
                // / - index
                (&Method::GET, "/") => Response::new(INDEX.into()),
                // /user - Users
                (method, path) if path.starts_with(USER_PATH) => {
                    let user_id = path
                        .trim_start_matches(USER_PATH)
                        .parse::<UserId>()
                        .ok()
                        .map(|x| x as usize);

                    let mut users = user_db.lock().unwrap();

                    match (method, user_id) {
                        // POST /user/
                        (&Method::POST, None) => {
                            let id = users.insert(UserData);
                            Response::new(id.to_string().into())
                        }
                        // POST /user/<id>
                        (&Method::POST, Some(_)) => response_with_code(StatusCode::BAD_REQUEST),
                        // GET /user/<id>
                        (&Method::GET, Some(id)) => {
                            if let Some(data) = users.get(id) {
                                Response::new(data.to_string().into())
                            } else {
                                response_with_code(StatusCode::NOT_FOUND)
                            }
                        }
                        // PUT updating data
                        (&Method::PUT, Some(id)) => {
                            if let Some(user) = users.get_mut(id) {
                                *user = UserData;
                                response_with_code(StatusCode::OK)
                            } else {
                                response_with_code(StatusCode::NOT_FOUND)
                            }
                        }
                        // DELETE - deleting data
                        (&Method::DELETE, Some(id)) => {
                            if users.contains(id) {
                                users.remove(id);
                                response_with_code(StatusCode::OK)
                            } else {
                                response_with_code(StatusCode::NOT_FOUND)
                            }
                        }

                        // Methods not allowed
                        _ => response_with_code(StatusCode::METHOD_NOT_ALLOWED),
                    }
                }
                // Rest
                _ => response_with_code(StatusCode::NOT_FOUND),
            }
        };
        future::ok(response)
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

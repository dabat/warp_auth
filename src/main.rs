use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;
use warp::Filter;
/*
    source article here: https://blog.joco.dev/posts/warp_auth_server_tutorial
    notes:
    1.  he writes this using warp 0.2 and tokio 0.2, both have upgraded since.
        i tried running this on warp 0.3 and tokio 1.4, and it would not compile with
        error[E0601]: `main` function not found in crate `warp_auth`
        could not find anything about this missing main fn error
    things to learn more about:
    1.  Arc
    2.  Mutex
    3.  Warp API
    4.  function return `impl`
*/

#[tokio::main]
async fn main() {
    // in-memory database
    let db = Arc::new(Mutex::new(HashMap::<String, User>::new()));
    let db = warp::any().map(move || Arc::clone(&db));
    // thread-safe count variable/state
    let count = Arc::new(Mutex::new(0));
    let count = warp::any().map(move || Arc::clone(&count));

    // routes
    let count = warp::path("count").and(count.clone()).and_then(counter);
    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(db.clone())
        .and_then(register);
    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and(db.clone())
        .and_then(login);
    let logout = warp::path("logout").map(|| "hello from logout");
    let routes = register.or(login).or(logout).or(count);
    let routes = warp::path("api").and(routes);

    // start the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn counter(db: Arc<Mutex<u8>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut counter = db.lock().await;
    *counter += 1;
    Ok(counter.to_string())
}

#[derive(Deserialize)]
struct User {
    username: String,
    password: String,
}

async fn register(
    new_user: User,
    db: Arc<Mutex<HashMap<String, User>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut users = db.lock().await;
    if users.contains_key(&new_user.username) {
        return Ok(StatusCode::BAD_REQUEST);
    }
    users.insert(new_user.username.clone(), new_user);
    Ok(StatusCode::CREATED)
}

async fn login(
    credentials: User,
    db: Arc<Mutex<HashMap<String, User>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let users = db.lock().await;
    match users.get(&credentials.username) {
        None => Ok(StatusCode::BAD_REQUEST),
        Some(user) => {
            if credentials.password == user.password {
                Ok(StatusCode::OK)
            } else {
                Ok(StatusCode::UNAUTHORIZED)
            }
        }
    }
}

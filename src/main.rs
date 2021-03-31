use argon2::{self, Config};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;
use warp::Filter;

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
    let list = warp::get()
        .and(warp::path("list"))
        .and(db.clone())
        .and_then(list);
    let routes = register.or(login).or(logout).or(count).or(list);
    let routes = warp::path("api").and(routes);

    // start the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn list(db: Arc<Mutex<HashMap<String, User>>>) -> Result<impl warp::Reply, warp::Rejection> {
    let users = db.lock().await;
    let users: Vec<User> = users.values().map(|user| user.clone()).collect();
    Ok(warp::reply::json(&users))
}

async fn counter(db: Arc<Mutex<u8>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut counter = db.lock().await;
    *counter += 1;
    Ok(counter.to_string())
}

#[derive(Clone, Deserialize, Serialize)]
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
    let user_hashed = User {
        username: new_user.username,
        password: hash(new_user.password.as_bytes()),
    };
    users.insert(user_hashed.username.clone(), user_hashed);
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
            if verify(&user.password, credentials.password.as_bytes()) {
                Ok(StatusCode::OK)
            } else {
                Ok(StatusCode::UNAUTHORIZED)
            }
        }
    }
}

pub fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}

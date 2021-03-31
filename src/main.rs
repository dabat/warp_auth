use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;
/*
    source article here: https://blog.joco.dev/posts/warp_auth_server_tutorial
    notes:
    1.  he writes this using warp 0.2 and tokio 0.2, both have upgraded since.
        i tried running this on warp 0.3 and tokio 1.4, and it would not compile with
        error[E0601]: `main` function not found in crate `warp_auth`
        could not find anything about this missing main fn error
*/

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(0));
    let db = warp::any().map(move || Arc::clone(&db));
    let count = warp::path("count").and(db.clone()).and_then(counter);
    let register = warp::path("register").map(|| "Hellow from register");
    let login = warp::path("login").map(|| "hello from login");
    let logout = warp::path("logout").map(|| "hello from logout");
    let routes = register.or(login).or(logout).or(count);
    let routes = warp::path("api").and(routes);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn counter(db: Arc<Mutex<u8>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut counter = db.lock().await;
    *counter += 1;
    Ok(counter.to_string())
}

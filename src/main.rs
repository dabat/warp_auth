#![deny(warnings)]
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
    let register = warp::path("register").map(|| "Hellow from register");
    let login = warp::path("login").map(|| "hello from login");
    let logout = warp::path("logout").map(|| "hello from logout");
    let routes = register.or(login).or(logout);
    let routes = warp::path("api").and(routes);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

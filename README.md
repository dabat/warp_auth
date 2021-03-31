# a simple authentication server in Rust, using Warp

source article here: [https://blog.joco.dev/posts/warp_auth_server_tutorial](https://blog.joco.dev/posts/warp_auth_server_tutorial)

notes:

1.  he writes this using warp 0.2 and tokio 0.2, both have upgraded since.
    i tried running this on warp 0.3 and tokio 1.4,
    and it would not compile with
    error[E0601]: `main` function not found in crate `warp_auth`
    could not find anything about this missing main fn error
1.  added a `list` method using some help from the `list_customers` handler example in this readme
    https://github.com/andrewleverette/rust_warp_api#list-customers

things to learn more about:

1. Warp API
1. Arc
1. Mutex
1. function return `impl`

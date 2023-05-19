use std::net::ToSocketAddrs;
use std::io;
use std::env;
use warp::Filter;
use serde_json::{json};

// RESTful HTTP API for bex sessions.

#[tokio::main]
async fn main() -> io::Result<()> {

  let index_route = warp::path::end().map(|| {
    warp::reply::json(&json!({
        "endpoints": [
            "/shell [POST]",
            "/shell/hist [GET]",
            "/n/{id}/graph [GET]",
            "/n/{id}/table [GET]"] })) });

  let shell_route = warp::post()
    .and(warp::path!("shell"))
    .and(warp::body::json())
    .map(|input: serde_json::Value| {
        // Handle the shell POST request here
        // ...

        // Temporary response
        warp::reply::json(&json!({"message": "Shell POST endpoint"}))
    });

  let hist_route = warp::get()
    .and(warp::path!("shell" / "hist"))
    .map(|| {
        // Handle the shell/hist GET request here
        // ...

        // Temporary response
        warp::reply::json(&json!({"message": "Shell/Hist GET endpoint"}))
    });

  let n_id_graph_route = warp::get()
    .and(warp::path!("n" / u32 / "graph"))
    .map(|id: u32| {
        // Handle the /n/{id}/graph GET request here
        // ...

        // Temporary response
        warp::reply::json(&json!({"message": format!("N/ID/Graph GET endpoint for ID {}", id)}))
    });

  let n_id_table_route = warp::get()
    .and(warp::path!("n" / u32 / "table"))
    .map(|id: u32| {
        // Handle the /n/{id}/table GET request here
        // ...

        // Temporary response
        warp::reply::json(&json!({"message": format!("N/ID/Table GET endpoint for ID {}", id)}))
    });

  let routes = index_route
    .or(shell_route)
    .or(hist_route)
    .or(n_id_graph_route)
    .or(n_id_table_route)
    .with(warp::log("API"));

  let addr = "localhost:8000";
  println!("Starting server on http://{addr}/");
  if env::var("RUST_LOG").is_err() {
    println!("[Use RUST_LOG environment variable to control logging.]");
    env::set_var("RUST_LOG", "info"); }
  pretty_env_logger::init();

  let sock = addr.to_socket_addrs()?.find(|a| a.is_ipv4()).unwrap();
  warp::serve(routes).run(sock).await;
  Ok(()) }

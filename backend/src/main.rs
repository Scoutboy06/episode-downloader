use warp::Filter;
use std::env;

#[tokio::main]
async fn main() {
    // Get the path to the frontend build folder from environment variable
    let frontend_path = env::var("FRONTEND_PATH").expect("Env FRONTEND_PATH not defined");
    println!("Frontend path: {}", frontend_path);

    // Serve static files from the frontend build folder
    let frontend_route = warp::fs::dir(frontend_path);

    // Start the warp server
    warp::serve(frontend_route)
        .run(([0, 0, 0, 0], 8080))    // Bind to all interfaces on port 8080
        .await;
}

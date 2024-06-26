use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World! from axum need to learn rust." }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6000").await.unwrap();
    println!("serving axum app at port 6000.");
    axum::serve(listener, app).await.unwrap();
    
}
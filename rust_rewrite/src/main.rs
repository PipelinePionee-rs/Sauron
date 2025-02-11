use tokio::net::TcpListener;
use axum::response::Html;
use axum::routing::get;
use axum::Router;


#[tokio::main]
async fn main() {
  println!("The Eye of Sauron is opening...");

  let routes_hello = Router::new().route(
    "/hello",
    get(|| async { Html("Hello Sauron!")}),
  );

	let listener = TcpListener::bind("localhost:8080").await.unwrap();
	println!("->> LISTENING on {:?}\n", listener.local_addr());
	axum::serve(listener, routes_hello.into_make_service())
		.await
		.unwrap();
}

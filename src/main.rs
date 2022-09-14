use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() ->  std::io::Result<()> {
    run(TcpListener::bind("http://127.0.0.1:0").unwrap())?.await
}

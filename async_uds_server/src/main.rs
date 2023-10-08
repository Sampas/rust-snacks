use tokio::net::UnixListener;

// Run tokio single-threaded
#[tokio::main(flavor="current_thread")]
async fn main() {
    let s = "/tmp/the_socket";
    let _ = std::fs::remove_file(s);
    println!("Listening...");
    let listener = UnixListener::bind(s).unwrap();
    loop {
        match listener.accept().await {
            Ok((_stream, _addr)) => {
                println!("new connection");
            }
            Err(_e) => {
                println!("accept() failed");
            }
        }
    }
}

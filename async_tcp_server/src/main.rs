use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("got connection from {:?}", addr);

        tokio::spawn(async move {
            let mut buf = [0; 10];

            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => break,
                    Ok(n) => n,
                    Err(error) => {
                        eprintln!("failed to read from socket; error = {:?}", error);
                        break;
                    }
                };

                println!("received {:?} bytes from {:?}", n, addr);

                if let Err(error) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; error = {:?}", error);
                    break;
                }
            }

            println!("{:?} disconnected", addr);
        });
    }
}

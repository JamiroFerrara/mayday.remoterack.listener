#![windows_subsystem = "windows"]

use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
use tokio::net::TcpStream;
use listener::updater::*;

pub fn main() -> Result<(), std::io::Error> {
    update();
    tokio_main()?;
    Ok(())
}

#[tokio::main]
pub async fn tokio_main() -> Result<(), std::io::Error> {
    loop {
        connect().await;
        println!("Reconnecting..")
    }
}

pub async fn connect() {
    let result = TcpStream::connect("172.105.66.226:8080").await;
    match result {
        Ok(mut stream) => {
            let result = stream.write("RBP001\n".as_bytes()).await;
            match result {
                Ok(_) => println!("Connected as RBP001.."),
                Err(e) => println!("Error sending name: {}", e),
            }

            let mut reader = BufReader::new(stream);

            loop {
                let mut line = String::new();
                match reader.read_line(&mut line).await {
                    Ok(reader) => {
                        if reader.eq(&0) {
                            println!("Connection closed by server");
                            break;
                        }

                        println!("Received: {:?}", line);

                        if line == "shutdown\n" {
                            shutdown();
                        } else if line == "reboot\n" {
                            reboot();
                        }
                    }
                    Err(e) => {
                        println!("Error reading line: {}", e);
                        break;
                    },
                }
            }
        },
        Err(e) => println!("Error connecting: {}", e),
    }

    //sleep for 1 second
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

// Error handled commands
fn shutdown() {
    match system_shutdown::shutdown() {
        Ok(_) => println!("Shutting down.."),
        Err(e) => println!("Error shutting down: {}", e),
    }
}

fn reboot() {
    match system_shutdown::reboot() {
        Ok(_) => println!("Rebooting.."),
        Err(e) => println!("Error rebooting: {}", e),
    }
}

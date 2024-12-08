mod services;
use std::future::IntoFuture;

use serde_json::from_slice;
use services::{Request, Response};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    let mut res: f64 = 0.0;
    while let Ok((mut socket, peer)) = listener.accept().await {
        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];

            if let Ok(bytes_read) = socket.read(&mut buffer).await {
                if let Ok(request) = from_slice::<Request>(&buffer[..bytes_read]) {
                    let result = match request {
                        Request::Factorial(n) => Response {
                            result: tokio::spawn(async move { services::factorial(n) })
                                .into_future()
                                .await
                                .unwrap(),
                        },
                        Request::Reciprocal(x) => Response {
                            result: tokio::spawn(async move { services::reciprocal(x) })
                                .into_future()
                                .await
                                .unwrap(),
                        },
                        Request::Addition(a, b) => Response {
                            result: tokio::spawn(async move { services::addition(a, b) })
                                .into_future()
                                .await
                                .unwrap(),
                        },
                    };
                    
                    res = result.result;
                    socket.write_all(&res.to_le_bytes()).await.unwrap();
                }
            }
        });
    }
}

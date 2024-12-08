
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{json, Number, Value};
use std::env;
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Factorial(i32),
    Reciprocal(f64),
    Addition(f64, f64),
}

async fn send_request(request: Request) -> f64 {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    let request_data = serde_json::to_vec(&request).unwrap();
    socket.write_all(&request_data).await.unwrap();

    let mut buffer = [0u8; 8];

    let bytes_read = socket.read(&mut buffer).await.unwrap();
    let value = f64::from_le_bytes(buffer); //f64 parsed in litte endian format
    return value;
}

#[tokio::main]
async fn main() {
    loop{
    let mut input = String::new();
    println!("Insert N value:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: i32 = input
        .trim() // Remove space
        .parse()
        .expect("Failed to convert the input");

    let mut e_approx = Decimal::new(0, 28);

    for i in 0..=number-1 {
        //n
        //print!("\niteration: {:}", i);
        let factorial = send_request(Request::Factorial(i)).await;
        //let factorial_dec = Decimal::from_f64_retain(factorial).unwrap();
        //print!("factorial dec: {:}\n", factorial_dec);
        let reciprocal = send_request(Request::Reciprocal(factorial)).await;
        //let reciprocal_dec = Decimal::from_f64_retain(reciprocal).unwrap();
        //print!("reciprocal dec: {:}\n", reciprocal_dec);
        let mut e_approx_f64 = e_approx.to_f64().unwrap();
        e_approx = Decimal::from_f64_retain(
            send_request(Request::Addition(e_approx_f64, reciprocal)).await,
        )
        .unwrap();
    }

    println!("The e value calculated is {:.32}", e_approx);
}
}

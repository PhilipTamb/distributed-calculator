use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Factorial(i32),
    Reciprocal(f64),
    Addition(f64, f64),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub result: f64,
}

pub fn factorial(n: i32) -> f64 {
    //println!("{:}", u128::MAX); //max number that u128 could rapresent 340282366920938463463374607431768211455 
    let mut factorial: u128 = 1; 
    for i in 1..(n + 1) {
        factorial *= i as u128;
    }
    return factorial as f64;
}

pub fn reciprocal(x: f64) -> f64 {
    1.0 / x
}

pub fn addition(a: f64, b: f64) -> f64 {
    a + b
}

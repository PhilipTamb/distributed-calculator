use std::process::{Child, Command};
use std::thread::sleep;
use std::time::Duration;

fn start_server() -> Child {
    Command::new("cargo")
        .current_dir("./server")
        .arg("run")
        .spawn()
        .expect("Impossibile avviare il server")
}

// fn start_client(num:i32) -> Child {
//     Command::new("cargo")
//         .current_dir("./client")
//         .arg("run")
//         .arg("--")
//         .arg(num)
//         .spawn()
//         .output()
//         .expect("Impossibile avviare il client")
// }

#[test]
fn test_e_computation() {
    let mut server = start_server();
    sleep(Duration::from_secs(2));

    let output = Command::new("cargo")
        .current_dir("./client")
        .arg("run")
        .arg("--release")
        .arg("--")
        .arg("5")
        .output()
        .expect("Errore nell'avvio del client");


    let result = String::from_utf8_lossy(&output.stdout);
    assert!(result.contains("Valore approssimato di e con N=5"));

    server.kill().unwrap();
}

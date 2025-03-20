use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use rs_udp_thermo::Termometer;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
    socket
        .set_read_timeout(Some(Duration::new(1, 0)))
        .expect("set_read_timeout call failed");

    let termometer = Arc::new(Termometer::new());
    termometer.start();
    let termo_clone: Arc<Termometer> = Arc::clone(&termometer);

    let (tx, rx) = channel::<f32>();

    let handle = thread::spawn(move || {
        loop {
            let mut buf = [0; 6];
            match socket.recv_from(&mut buf) {
                Ok((recieved, src)) => {
                    if termo_clone.is_working() {
                        let temperature = String::from_utf8_lossy(&buf[..recieved]);

                        println!(
                            "---> Received temperature via UDP: {} from: {}",
                            temperature, src
                        );
                        let t = temperature.to_string().parse::<f32>().unwrap();
                        let _ = tx.send(t);
                    } else {
                        println!("Shutting down UDP Serv");
                        return;
                    }
                }
                Err(e) => {
                    println!(
                        "recv_from function failed: {:?}. Maybe generator not working",
                        e
                    );
                }
            }
        }
    });

    while termometer.is_working() {
        let recieved = rx.recv().unwrap();
        termometer.set_temperature(recieved);

        println!("Temperature: {}", termometer.temperature());

        if termometer.is_overheated() {
            println!("Overheated! Stoppin...");
            termometer.stop();
        }

        thread::sleep(Duration::from_secs(1));
    }

    handle.join().unwrap();
}

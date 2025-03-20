use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    socket
        .connect("127.0.0.1:34254")
        .expect("connect function failed");

    loop {
        let temperature = format!("{:.3}", rand::random::<f32>() * 100.0);
        println!("Sending temperature to {}", temperature);
        socket
            .send(temperature.as_bytes())
            .expect("send function failed");
        thread::sleep(Duration::from_secs(1));
    }
}

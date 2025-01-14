use std::env;
use std::net::SocketAddr;
use std::thread;

use socket2::Domain;
use socket2::Protocol;
use socket2::Socket;
use socket2::Type;

fn main() {
    let port: u16 = env::var("PORT").unwrap().parse().unwrap();
    let worker_count: usize = 8;
    let backlog: i32 = 2048;

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    (0..worker_count)
        .fold(Vec::with_capacity(worker_count), |mut acc, _| {
            let worker = thread::spawn(move || {
                let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).unwrap();
                socket.set_reuse_port(true).unwrap();
                socket.bind(&addr.into()).unwrap();
                socket.listen(backlog).unwrap();
            });
            acc.push(worker);
            acc
        })
        .into_iter()
        .for_each(|x| x.join().unwrap());
}

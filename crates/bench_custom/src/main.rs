use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::thread;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Request;
use hyper::Response;
use hyper_util::rt::TokioIo;
use socket2::Domain;
use socket2::Protocol;
use socket2::Socket;
use socket2::Type;
use tokio::net::TcpListener;
use tokio::runtime;
use tokio::task::LocalSet;

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

fn main() {
    let port: u16 = env::var("PORT").unwrap().parse().unwrap();
    let thread_count: usize = 8;
    let backlog: i32 = 2048;

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    (0..thread_count)
        .fold(Vec::with_capacity(thread_count), |mut acc, _| {
            let worker = thread::spawn(move || {
                let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).unwrap();
                socket.set_reuse_port(true).unwrap();
                socket.bind(&addr.into()).unwrap();
                socket.listen(backlog).unwrap();
                let local = LocalSet::new();
                runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(local.run_until(async {
                        let listener = TcpListener::from_std(socket.into()).unwrap();
                        loop {
                            let (stream, _) = listener.accept().await.unwrap();
                            let io = TokioIo::new(stream);
                            local
                                .spawn_local(async move {
                                    http1::Builder::new()
                                        .keep_alive(true)
                                        .serve_connection(io, service_fn(hello))
                                        .await
                                        .unwrap();
                                })
                                .await
                                .unwrap();
                        }
                    }));
            });
            acc.push(worker);
            acc
        })
        .into_iter()
        .for_each(|x| x.join().unwrap());
}

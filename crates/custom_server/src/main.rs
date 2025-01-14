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
use mimalloc::MiMalloc;
use socket2::Domain;
use socket2::Protocol;
use socket2::Socket;
use socket2::Type;
use tokio::net::TcpListener;
use tokio::runtime;
use tokio::task::LocalSet;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

fn main() {
    let port: u16 = env::var("PORT").unwrap().parse().unwrap();
    let thread_count: usize = 8;
    (0..thread_count)
        .fold(Vec::with_capacity(thread_count), |mut acc, i| {
            let worker = thread::spawn(move || {
                let addr = SocketAddr::from(([127, 0, 0, 1], port));
                let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).unwrap();
                socket.set_reuse_port(true).unwrap();
                socket.set_reuse_address(true).unwrap();
                socket.bind(&addr.into()).unwrap();
                socket.listen(2048).unwrap();
                let runtime = runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                runtime.block_on(async {
                    let local = LocalSet::new();
                    local
                        .run_until(async {
                            let listener = TcpListener::from_std(socket.into()).unwrap();
                            loop {
                                let (stream, _) = listener.accept().await.unwrap();
                                eprintln!("{i}-thread accepted.");
                                let io = TokioIo::new(stream);
                                local
                                    .spawn_local(async move {
                                        if let Err(e) = http1::Builder::new()
                                            .keep_alive(true)
                                            .serve_connection(io, service_fn(hello))
                                            .await
                                        {
                                            eprintln!("custom - Error serving connection: {:?}", e);
                                        }
                                    })
                                    .await
                                    .unwrap();
                            }
                        })
                        .await;
                });
            });
            acc.push(worker);
            acc
        })
        .into_iter()
        .for_each(|x| x.join().unwrap());
}

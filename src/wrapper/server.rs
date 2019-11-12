use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Response, Server};

pub struct App {}

impl App {
    pub fn new() -> App {
        App {}
    }
}

pub struct HttpServer {
    addr: Option<std::net::SocketAddr>,
    app: Option<App>,
}

impl HttpServer {
    pub fn new() -> HttpServer {
        HttpServer {
            addr: None,
            app: None,
        }
    }

    pub fn bind(&mut self, addr: std::net::SocketAddr) -> &mut Self {
        self.addr = Some(addr);

        self
    }

    pub fn service(&mut self, app: App) -> &mut Self {
        self.app = Some(app);

        self
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        let addr = self.addr.take().unwrap();
        let server = Server::bind(&addr).serve(make_service_fn(|_| {
            async {
                Ok::<_, Error>(service_fn(|req| {
                    async { Ok::<_, Error>(Response::new(Body::from("hello"))) }
                }))
            }
        }));

        println!("Listing on http://{}", addr);
        server.await
    }
}

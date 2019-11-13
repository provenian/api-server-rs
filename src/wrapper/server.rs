use futures::prelude::*;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use path_tree::PathTree;

type FutureResult<O, E> = std::pin::Pin<Box<dyn Future<Output = Result<O, E>> + Send>>;
type Params<'a> = Vec<(&'a str, &'a str)>;
type Handler<I, O> = fn(Request<I>, Params) -> FutureResult<Response<O>, http::Error>;

#[derive(Clone)]
pub struct App {
    paths: PathTree<Handler<Body, Body>>,
}

fn internal_path(method: &Method, path: &str) -> String {
    format!("/{}/{}", method, path)
}

impl App {
    pub fn new() -> App {
        App {
            paths: PathTree::new(),
        }
    }

    pub fn service(
        &mut self,
        path: &str,
        method: Method,
        service: Handler<Body, Body>,
    ) -> &mut Self {
        self.paths
            .insert(internal_path(&method, path).as_str(), service);

        self
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

    pub async fn run(&mut self) -> Result<(), hyper::Error> {
        let addr = self.addr.take().unwrap();
        let app = self.app.take().unwrap();
        let server = Server::bind(&addr).serve(make_service_fn(|_| {
            let app = app.clone();

            async {
                Ok::<_, hyper::Error>(service_fn(move |req| {
                    let paths = app.paths.clone();
                    let p = internal_path(req.method(), req.uri().to_string().as_str());

                    async move {
                        match paths.find(p.as_str()) {
                            None => Response::builder().status(405).body(Body::from("")),
                            Some((f, ps)) => f(req, ps).await,
                        }
                    }
                }))
            }
        }));

        println!("Listing on http://{}", addr);
        server.await
    }
}

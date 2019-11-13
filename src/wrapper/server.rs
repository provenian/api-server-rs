use futures::prelude::*;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use path_tree::PathTree;
use std::sync::Arc;

type FutureResult<O, E> = std::pin::Pin<Box<dyn Future<Output = Result<O, E>> + Send>>;
type Params<'a> = Vec<(&'a str, &'a str)>;
type Handler<D, I, O> = fn(Request<I>, Params, &D) -> FutureResult<Response<O>, http::Error>;

#[derive(Clone)]
pub struct App<D> {
    paths: PathTree<Handler<D, Body, Body>>,
    data: Arc<D>,
}

fn internal_path(method: &Method, path: &str) -> String {
    format!("/{}/{}", method, path)
}

impl<D> App<D> {
    pub fn new(data: D) -> App<D> {
        App {
            paths: PathTree::new(),
            data: Arc::new(data),
        }
    }

    pub fn route(mut self, path: &str, method: Method, service: Handler<D, Body, Body>) -> Self {
        self.paths
            .insert(internal_path(&method, path).as_str(), service);

        self
    }
}

pub struct HttpServer<D> {
    addr: Option<std::net::SocketAddr>,
    app: Option<App<D>>,
}

impl<D: Clone + Sync + Send + 'static> HttpServer<D> {
    pub fn new() -> HttpServer<D> {
        HttpServer {
            addr: None,
            app: None,
        }
    }

    pub fn bind(&mut self, addr: std::net::SocketAddr) -> &mut Self {
        self.addr = Some(addr);

        self
    }

    pub fn service(&mut self, app: App<D>) -> &mut Self {
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
                    let data = app.data.clone();

                    async move {
                        match paths.find(p.as_str()) {
                            None => Response::builder().status(405).body(Body::from("")),
                            Some((f, ps)) => f(req, ps, data.as_ref()).await,
                        }
                    }
                }))
            }
        }));

        println!("Listing on http://{}", addr);
        server.await
    }
}

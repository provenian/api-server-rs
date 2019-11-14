use futures::prelude::*;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use path_tree::PathTree;
use std::pin::Pin;
use std::sync::Arc;

type FutureResult<O, E> = Pin<Box<dyn Future<Output = Result<O, E>> + Send>>;
pub type Params = Vec<(String, String)>;
type Handler<D, I, O> =
    Arc<dyn Fn(Request<I>, Params, Arc<D>) -> FutureResult<Response<O>, http::Error> + Sync + Send>;

pub struct App<D> {
    paths: PathTree<Handler<D, Body, Body>>,
    data: Arc<D>,
}

impl<D> Clone for App<D> {
    fn clone(&self) -> Self {
        App {
            paths: self.paths.clone(),
            data: self.data.clone(),
        }
    }
}

fn internal_path(method: &Method, path: &str) -> String {
    format!("/{}/{}", method, path)
}

impl<D: Sync + Send + 'static> App<D> {
    pub fn new(data: D) -> App<D> {
        App {
            paths: PathTree::new(),
            data: Arc::new(data),
        }
    }

    pub fn route<F, T>(mut self, path: &str, method: Method, f: F) -> Self
    where
        F: Fn(Request<Body>, Params, Arc<D>) -> T + Clone + Sync + Send + 'static,
        T: Future<Output = Result<Response<Body>, http::Error>> + Send + 'static,
    {
        self.paths.insert(
            internal_path(&method, path).as_str(),
            Arc::new(move |r, p, d| Box::pin(f(r, p, d))),
        );

        self
    }
}

pub struct HttpServer<D> {
    addr: Option<std::net::SocketAddr>,
    app: Option<App<D>>,
}

impl<D: Sync + Send + 'static> HttpServer<D> {
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
                            Some((f, ps)) => {
                                f(
                                    req,
                                    ps.iter()
                                        .map(|(x, y)| (x.to_string(), y.to_string()))
                                        .collect::<Vec<_>>(),
                                    data.clone(),
                                )
                                .await
                            }
                        }
                    }
                }))
            }
        }));

        println!("Listing on http://{}", addr);
        server.await
    }
}

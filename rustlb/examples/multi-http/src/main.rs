use futures::prelude::*;
use futures::stream::futures_unordered::FuturesUnordered;
use hyper::rt::{self, Future};
use hyper::{Client, StatusCode};

fn setup_requests() -> FuturesUnordered<Request> {
    let mut set = FuturesUnordered::new();

    let client = Client::new();

    let uri = "http://httpbin.org/status/200".parse().unwrap();
    let first = client.get(uri);
    set.push(first);

    let uri = "http://httpbin.org/status/404".parse().unwrap();
    let second = client.get(uri);
    set.push(second);

    let uri = "http://httpbin.org/status/418".parse().unwrap();
    let third = client.get(uri);
    set.push(third);

    set
}

fn main() {
    rt::run(rt::lazy(|| {
        // This is main future that the runtime will execute.
        //
        // The `lazy` is because we don't want any of this executing *right now*,
        // but rather once the runtime has started up all its resources.
        //
        // This is where we will setup our HTTP client requests.
        // still inside rt::run...

        setup_requests().for_each(|response| {
            let status = response.status();
            println!("Status: {}", status);
            Ok(())
        })
    }));
}

#![allow(dead_code)]

use std::future::Future;
/// Read more from [rust blog](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html)

struct Player {
    name: String,
}
/// Given a list of players, return an iterator
/// over their names.
fn player_names(players: &[Player]) -> impl Iterator<Item = &String> {
    players.iter().map(|p| &p.name)
}

#[derive(Clone)]
struct Widget;
struct MyContainer {
    items: Vec<Widget>,
}

/// Define a trait that returns a trait.
trait Container {
    fn items(&self) -> impl Iterator<Item = Widget>;
}

impl Container for MyContainer {
    fn items(&self) -> impl Iterator<Item = Widget> {
        self.items.iter().cloned()
    }
}

/// Also define a trait that returns a trait. But this time it returns an **async trait**.

pub struct Url;
struct HtmlBody;

/// Let's do this with the help of `trait_variant`:
#[trait_variant::make(HttpService: Send)]
pub trait LocalHttpService {
    async fn fetch(&self, url: Url) -> HtmlBody;
}

/// We can do this manually.
pub trait HttpServiceManually: Send {
    fn fetch(&self, url: Url) -> impl Future<Output = HtmlBody> + Send;
}

fn spawn_task(service: impl HttpService + 'static) {
    tokio::spawn(async move {
        let url = Url;
        let _body = service.fetch(url).await;
    });
}

fn spawn_task1(service: impl HttpServiceManually + 'static) {
    tokio::spawn(async move {
        let url = Url;
        let _body = service.fetch(url).await;
    });
}

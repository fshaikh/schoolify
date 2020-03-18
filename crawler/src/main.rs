mod appcontroller;
mod crawlers;
mod models;
mod platform;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    futures::executor::block_on(appcontroller::start());
}

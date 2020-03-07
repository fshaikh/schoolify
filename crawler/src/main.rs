
mod services;
mod appcontroller;
mod crawlers;
mod platform;
mod utils;
mod models;

#[tokio::main]
async fn main() {
    futures::executor::block_on(appcontroller::start());
}

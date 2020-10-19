mod appcontroller;
mod cache;
mod crawlers;
mod dal;
mod models;
mod platform;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    futures::executor::block_on(appcontroller::start());
    // tokio_executor::blocking::run(appcontroller::start());
}

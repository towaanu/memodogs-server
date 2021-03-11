use warp::Filter;

mod config;
mod app_logger;

#[tokio::main]
async fn main() {
    app_logger::init_logger();
    let config = config::get_config();
    let index = warp::path::end().map(|| "Hello world !");

    log::info!("Server running on port {}", config.port);
    warp::serve(index).run(([0, 0, 0, 0], config.port)).await;
}

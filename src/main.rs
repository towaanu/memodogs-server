use std::sync::Arc;
use warp::Filter;

mod app_logger;
mod config;
mod filters;
mod routes;

#[tokio::main]
async fn main() {
    app_logger::init_logger();
    let config = Arc::new(config::get_config());
    let index = warp::path::end().map(|| "Hello world !");

    let api = warp::path("api")
        .and(warp::get())
        .and(routes::random_images::random_images(config.clone()))
        .with(warp::log("memodogs::api"));

    let server_routes = index.or(api);

    log::info!("Server running on port {}", config.port);
    warp::serve(server_routes)
        .run(([0, 0, 0, 0], config.port))
        .await;
}

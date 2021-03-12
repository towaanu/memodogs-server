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

    let cors = warp::cors()
        .allow_origin(config.cors_origin.as_str())
        .allow_method("GET");

    let api = warp::path("api")
        .and(warp::get())
        .and(routes::random_images::random_images(config.clone()))
        .with(cors.clone())
        .with(warp::log("memodogs::api"));

    let images_static = warp::path("static")
        .and(warp::fs::dir(config.images_path.clone()))
        .with(cors)
        .with(warp::log("memodogs::static"));

    let server_routes = index.or(api).or(images_static);

    log::info!("Server running on port {}", config.port);
    warp::serve(server_routes)
        .run(([0, 0, 0, 0], config.port))
        .await;
}

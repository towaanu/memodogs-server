use warp::Filter;

mod config;

#[tokio::main]
async fn main() {
    let config = config::get_config();
    let index = warp::path::end().map(|| "Hello world !");
    warp::serve(index).run(([0, 0, 0, 0], config.port)).await;
}

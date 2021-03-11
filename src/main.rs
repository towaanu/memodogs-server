use warp::Filter;

#[tokio::main]
async fn main() {
    let index = warp::path::end().map(|| "Hello world !");
    warp::serve(index).run(([0, 0, 0, 0], 3030)).await;
}

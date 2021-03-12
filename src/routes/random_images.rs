use crate::config::Config;
use crate::filters::with_config;
use std::sync::Arc;
use warp::Filter;

pub fn random_images(
    config: Arc<Config>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    random_images_list(config.clone())
}

pub fn random_images_list(
    config: Arc<Config>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("random-images" / usize)
        .and(warp::get())
        .and(with_config(config))
        .and_then(handlers::list_random_images)
}

mod handlers {
    use crate::config::Config;
    use futures::future;
    use futures::stream::StreamExt;
    use std::convert::Infallible;
    use std::path::Path;
    use std::sync::Arc;
    use tokio::fs::read_dir;
    use tokio_stream::wrappers::ReadDirStream;
    use url::Url;

    pub async fn list_random_images(
        count: usize,
        config: Arc<Config>,
    ) -> Result<impl warp::Reply, Infallible> {
        let images_path = Path::new(&config.images_path).join("dogs");

        let dir_stream = ReadDirStream::new(read_dir(images_path).await.expect("Images not found"));
        let image_urls: Vec<String> = dir_stream
            .take(count)
            .filter(|entry| future::ready(entry.is_ok()))
            .map(|entry| entry.unwrap())
            .filter(|entry| {
                if let Some(ext) = entry.path().extension() {
                    future::ready(ext == "jpg")
                } else {
                    future::ready(false)
                }
            })
            .map(|entry| entry.file_name().into_string().unwrap())
            .map(|file_name| {
                let file_url = Url::parse(&config.static_base_url).unwrap();
                file_url
                    .join("dogs")
                    .unwrap()
                    .join(&file_name)
                    .unwrap()
                    .into_string()
            })
            .collect()
            .await;
        Ok(warp::reply::json(&image_urls))
    }
}

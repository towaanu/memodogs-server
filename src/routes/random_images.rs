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
    warp::path!("random-images" / String / usize)
        .and(warp::get())
        .and(with_config(config))
        .and_then(handlers::list_random_images)
}

mod handlers {
    use crate::config::Config;
    use futures::future;
    use futures::stream::StreamExt;
    use std::path::Path;
    use std::sync::Arc;
    use tokio::fs::read_dir;
    use tokio_stream::wrappers::ReadDirStream;
    use url::Url;

    pub async fn list_random_images(
        images_topic: String,
        count: usize,
        config: Arc<Config>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let images_path = Path::new(&config.images_path).join(&images_topic);

        let dir_stream = ReadDirStream::new(
            read_dir(images_path)
                .await
                .map_err(|_e| warp::reject::not_found())?,
        );

        let base_file_url = Url::parse(&config.static_base_url).map_err(|_| {
            log::error!("Bad static images url {}", config.static_base_url);
            warp::reject::not_found()
        })?;

        let image_urls: Vec<String> = dir_stream
            .filter_map(|entry| future::ready(entry.ok()))
            .filter(|entry| {
                if let Some(ext) = entry.path().extension() {
                    future::ready(ext == "jpg")
                } else {
                    future::ready(false)
                }
            })
            .take(count)
            .map(|entry| entry.file_name().into_string().unwrap())
            .map(|file_name| {
                let mut file_url = base_file_url.clone();
                file_url
                    .path_segments_mut()
                    .unwrap()
                    .extend(&[&images_topic, &file_name]);
                file_url.into_string()
            })
            .collect()
            .await;
        Ok(warp::reply::json(&image_urls))
    }
}

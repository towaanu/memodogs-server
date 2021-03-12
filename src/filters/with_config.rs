use crate::config::Config;
use std::sync::Arc;
use warp::Filter;

pub fn with_config(
    config: Arc<Config>,
) -> impl Filter<Extract = (Arc<Config>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || config.clone())
}

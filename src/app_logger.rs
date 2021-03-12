use env_logger::{Builder, Target};
use log::LevelFilter;

pub fn init_logger() {
    let mut builder = Builder::new();
    builder
        .filter_level(LevelFilter::Info)
        .parse_env("RUST_LOG")
        .target(Target::Stdout)
        .init()
}

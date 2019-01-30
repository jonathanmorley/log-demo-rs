use log;
use log_demo;

fn main() {
    let log_level_filter = log::LevelFilter::Trace;
    env_logger::Builder::new().filter_level(log_level_filter).init();
    log_demo::log();
}

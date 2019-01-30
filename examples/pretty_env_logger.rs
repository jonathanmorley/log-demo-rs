use log;
use log_demo;

fn main() {
    let log_level_filter = log::LevelFilter::Trace;
    pretty_env_logger::formatted_builder().filter_level(log_level_filter).init();
    log_demo::log();
}

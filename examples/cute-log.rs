use log;
use log_demo;

fn main() {
    let log_level_filter = log::LevelFilter::Trace;
    cute_log::init_with_max_level(log_level_filter).unwrap();
    log_demo::log();
}

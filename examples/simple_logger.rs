use log;
use log_demo;

fn main() {
    let log_level = log::Level::Trace;
    simple_logger::init_with_level(log_level).unwrap();
    log_demo::log();
}

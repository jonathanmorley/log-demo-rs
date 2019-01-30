use log;
use log_demo;

fn main() {
    let log_level = log::Level::Trace;

    loggerv::Logger::new()
        .max_level(log_level)
        .level(true)
        .init()
        .unwrap();

    log_demo::log();
}

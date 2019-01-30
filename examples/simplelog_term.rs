use log;
use log_demo;

fn main() {
    let log_level_filter = log::LevelFilter::Trace;
    simplelog::TermLogger::init(log_level_filter, simplelog::Config::default()).unwrap();
    log_demo::log();
}

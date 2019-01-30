use log_demo;

fn main() {
    stderrlog::new()
        .module("log_demo")
        .verbosity(4)
        .init()
        .unwrap();
    log_demo::log();
}

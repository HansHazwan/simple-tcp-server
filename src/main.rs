use std::env;

fn main() {
    env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();

    log::info!("Hello, World from logger");
}

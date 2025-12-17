use log::{info, debug, error};

pub fn init() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
}

pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_debug(message: &str) {
    debug!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}


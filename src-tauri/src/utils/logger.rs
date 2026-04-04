use log;

/// Initialize logging
pub fn init_logger() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();
}

/// Log application info
pub fn log_info(message: &str) {
    log::info!("{}", message);
}

/// Log application error
pub fn log_error(message: &str) {
    log::error!("{}", message);
}

/// Log application warning
pub fn log_warn(message: &str) {
    log::warn!("{}", message);
}

/// Log application debug
pub fn log_debug(message: &str) {
    log::debug!("{}", message);
}

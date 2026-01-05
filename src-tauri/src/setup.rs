use tracing::Level;

pub fn setup_logging() {
    let builder = tracing_subscriber::fmt().with_max_level(Level::DEBUG);

    builder.init();
}

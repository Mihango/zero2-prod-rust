use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, Registry};
use tracing_subscriber::fmt::MakeWriter;

pub fn get_subscriber<Sink>(
    name: String,
    env_filer: String,
    sink: Sink,
) -> impl Subscriber + Sync + Send
    where
        Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static {
    // We are falling back to printing all spans at info-level or above if the RUST_LOG env var is not set.
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filer));
    let formatting_layer = BunyanFormattingLayer::new(
        name, sink, );
    // the 'with' method is provided by SubscriberExt an extension trait for Subscriber exposed
    // by tracing-subscriber
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect all log events to our subscriber
    LogTracer::init().expect("Failed to set logger");
    // set global default can be used by applications to specify what subscriber should be used to process
    // spans
    set_global_default(subscriber).expect("Failed to set subscriber");
}

use opentelemetry::KeyValue;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, runtime, trace::TracerProvider};
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

use crate::Config;

pub fn init_telemetry(config: &Config) {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug,sqlx=warn"));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    let fmt_layer = if config.json_logs {
        fmt_layer.json().boxed()
    } else {
        fmt_layer.boxed()
    };

    let registry = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer);

    if let Some(ref otlp_endpoint) = config.otlp_endpoint {
        let tracer_provider = init_tracer_provider(otlp_endpoint, &config.service_name);
        let otel_layer = tracing_opentelemetry::layer()
            .with_tracer(tracer_provider.tracer(config.service_name.clone()));
        registry.with(otel_layer).init();
        tracing::info!(
            otlp_endpoint = %otlp_endpoint,
            service_name = %config.service_name,
            "OpenTelemetry tracing initialized"
        );
    } else {
        registry.init();
        tracing::info!("Telemetry initialized without OpenTelemetry exporter");
    }
}

fn init_tracer_provider(endpoint: &str, service_name: &str) -> TracerProvider {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()
        .expect("Failed to create OTLP exporter");

    TracerProvider::builder()
        .with_batch_exporter(exporter, runtime::Tokio)
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            service_name.to_string(),
        )]))
        .build()
}

pub fn shutdown_telemetry() {
    opentelemetry::global::shutdown_tracer_provider();
}

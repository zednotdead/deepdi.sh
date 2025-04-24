use color_eyre::Result;
use opentelemetry::propagation::TextMapCompositePropagator;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_sdk::metrics::{MeterProviderBuilder, PeriodicReader, SdkMeterProvider};
use opentelemetry_sdk::propagation::{BaggagePropagator, TraceContextPropagator};
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_sdk::Resource;
use tracing::level_filters::LevelFilter;
use tracing_opentelemetry::MetricsLayer;
use tracing_subscriber::{layer::SubscriberExt, Registry};
use tracing_subscriber::{EnvFilter, Layer};

#[coverage(off)]
fn resource() -> Resource {
    Resource::builder()
        .with_service_name("deepdi.sh-backend")
        .build()
}

#[coverage(off)]
fn init_meter_provider() -> Result<SdkMeterProvider> {
    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_temporality(opentelemetry_sdk::metrics::Temporality::default())
        .build()?;

    let reader = PeriodicReader::builder(exporter)
        .with_interval(std::time::Duration::from_secs(30))
        .build();

    let meter_provider = MeterProviderBuilder::default()
        .with_resource(resource())
        .with_reader(reader)
        .build();

    opentelemetry::global::set_meter_provider(meter_provider.clone());

    Ok(meter_provider)
}

#[coverage(off)]
fn init_tracer_provider() -> Result<SdkTracerProvider> {
    let otlp_exporter_tracer = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()?;

    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(otlp_exporter_tracer)
        .with_resource(resource())
        .build();

    Ok(tracer_provider)
}

#[coverage(off)]
fn init_logger_provider() -> Result<SdkLoggerProvider> {
    let log_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .build()?;

    let log_provider = SdkLoggerProvider::builder()
        .with_batch_exporter(log_exporter)
        .with_resource(resource())
        .build();

    Ok(log_provider)
}

#[coverage(off)]
pub fn init_tracing() -> Result<()> {
    let propagator = TextMapCompositePropagator::new(vec![
        Box::new(TraceContextPropagator::new()),
        Box::new(BaggagePropagator::new()),
    ]);

    opentelemetry::global::set_text_map_propagator(propagator);

    let meter_provider = init_meter_provider()?;
    let tracer_provider = init_tracer_provider()?;
    let logger_provider = init_logger_provider()?;

    let tracer = tracer_provider.tracer("deepdi.sh-backend");

    let otel_traces = tracing_opentelemetry::layer()
        .with_tracer(tracer)
        .with_error_records_to_exceptions(true);

    let otel_log = OpenTelemetryTracingBridge::new(&logger_provider).with_filter(
        EnvFilter::builder()
            .with_default_directive(LevelFilter::DEBUG.into())
            .with_env_var(EnvFilter::DEFAULT_ENV)
            .parse("")?
            .add_directive("h2=info".parse()?)
            .add_directive("tonic=error".parse()?)
            .add_directive("tower=error".parse()?)
            .add_directive("opentelemetry_sdk=error".parse()?)
            .add_directive("opentelemetry-otlp=error".parse()?)
            .add_directive("hyper_util=error".parse()?)
            .add_directive("sqlx::query=error".parse()?),
    );

    let otel_metrics = MetricsLayer::new(meter_provider);

    let subscriber = Registry::default()
        .with(otel_log)
        .with(otel_traces)
        .with(otel_metrics);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

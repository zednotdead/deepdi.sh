use color_eyre::Result;
use opentelemetry::propagation::TextMapCompositePropagator;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::metrics::{PeriodicReader, SdkMeterProvider};
use opentelemetry_sdk::propagation::{BaggagePropagator, TraceContextPropagator};
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_sdk::Resource;
use tracing::level_filters::LevelFilter;
use tracing_opentelemetry::MetricsLayer;
use tracing_subscriber::{layer::SubscriberExt, Registry};

#[cfg_attr(coverage_nightly, coverage(off))]
pub fn init_tracing() -> Result<()> {
    let propagator = TextMapCompositePropagator::new(vec![
        Box::new(TraceContextPropagator::new()),
        Box::new(BaggagePropagator::new()),
    ]);

    opentelemetry::global::set_text_map_propagator(propagator);

    let otlp_exporter_tracer = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()?;

    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(otlp_exporter_tracer)
        .with_resource(
            Resource::builder()
                .with_service_name("deepdi.sh-backend")
                .build(),
        )
        .build();

    let tracer = tracer_provider.tracer("deepdi.sh-backend");

    let telemetry = tracing_opentelemetry::layer()
        .with_tracer(tracer)
        .with_error_records_to_exceptions(true);

    let metrics_exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .build()?;

    let metrics_reader = PeriodicReader::builder(metrics_exporter).build();
    let metrics_provider = SdkMeterProvider::builder()
        .with_reader(metrics_reader)
        .build();

    let otel_metrics = MetricsLayer::new(metrics_provider);

    let filter = tracing_subscriber::filter::EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .parse("otel::tracing=trace,otel=debug,h2=info")?;

    let subscriber = Registry::default()
        .with(filter)
        .with(telemetry)
        .with(otel_metrics)
        .with(tracing_logfmt::layer());

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

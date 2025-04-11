import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-proto';
import { registerOTel } from '@vercel/otel'
 
const traceExporter = new OTLPTraceExporter({
  url: 'http://localhost:4318/v1/traces',
});

export function register() {
  registerOTel({ serviceName: 'next-app', traceExporter })
}

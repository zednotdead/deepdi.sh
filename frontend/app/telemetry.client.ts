import {
  BatchSpanProcessor,
} from '@opentelemetry/sdk-trace-base';
import { WebTracerProvider } from '@opentelemetry/sdk-trace-web';
import { ZoneContextManager } from '@opentelemetry/context-zone';
import { registerInstrumentations } from '@opentelemetry/instrumentation';
import { W3CTraceContextPropagator } from '@opentelemetry/core';
import { getWebAutoInstrumentations } from '@opentelemetry/auto-instrumentations-web';
import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-proto';
import { diag, DiagConsoleLogger, DiagLogLevel } from '@opentelemetry/api';
import { ATTR_SERVICE_NAME } from '@opentelemetry/semantic-conventions';
import { resourceFromAttributes } from '@opentelemetry/resources';

diag.setLogger(new DiagConsoleLogger(), DiagLogLevel.DEBUG);

const traceExporter = new OTLPTraceExporter({
  url: '/v1/traces',
  headers: {
    'Content-Type': 'application/json',
  },
});

const provider = new WebTracerProvider({
  resource: resourceFromAttributes({
    [ATTR_SERVICE_NAME]: 'deepdi.sh-frontend-web',
  }),
  spanProcessors: [new BatchSpanProcessor(traceExporter)],
});

provider.register({
  // Changing default contextManager to use ZoneContextManager - supports asynchronous operations - optional
  contextManager: new ZoneContextManager(),
  propagator: new W3CTraceContextPropagator(),
});

// Registering instrumentations
registerInstrumentations({
  tracerProvider: provider,
  instrumentations: [
    getWebAutoInstrumentations(),
  ],
});

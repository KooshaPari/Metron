# metrickit - Metrics Collection and Reporting

Prometheus-compatible metrics with zero-overhead collection.

## Features

- **Prometheus Compatible**: Full Prometheus data model support
- **Zero-overhead Collection**: Metrics created only when enabled
- **Multiple Exporters**: Prometheus, StatsD, JSON
- **Cardinality Safety**: Built-in protection against high cardinality labels
- **Async Support**: Non-blocking metric updates

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      HEXAGONAL ARCHITECTURE                  │
├─────────────────────────────────────────────────────────────┤
│  Domain Layer                                                │
│  ├── Metric (entity)                                        │
│  ├── Counter, Gauge, Histogram, Summary (value objects)      │
│  └── Meter trait (port)                                     │
├─────────────────────────────────────────────────────────────┤
│  Application Layer                                           │
│  ├── MetricRegistry (use case)                              │
│  └── MetricExporter (use case)                              │
├─────────────────────────────────────────────────────────────┤
│  Adapters                                                    │
│  ├── PrometheusExporter, StatsDExporter                     │
│  └── InMemoryRegistry                                       │
└─────────────────────────────────────────────────────────────┘
```

## Usage

```rust
use metrickit::{Registry, Counter, Histogram};

let registry = Registry::default();
let counter = registry.counter("requests_total", "Total requests");

counter.inc();
```

## License

MIT OR Apache-2.0

//! What flume sends of its telemetry, and how (216 to 220; design 4, 8,
//! 10, 11): chosen once as a command begins, from the managed layer's
//! `[telemetry]` and OpenTelemetry's own variables, which only turn a
//! signal off; kept in memory while the command runs; and sent once,
//! after its answer, to the table's collector alone, within what remains
//! of three seconds, a failure silent and nothing kept to send again.
//! The core hands its signals across `flume_seams::Telemetry` and names
//! no crate of these (core.2, core.13, core.56).

use flume_seams::telemetry::{self as keys, Value, METRIC_KEYS};
use flume_seams::{Attributes, Sends, Signal, Telemetry, TelemetryTable, TELEMETRY_SERVICE};
use opentelemetry::logs::{AnyValue, LogRecord as _, Logger as _, LoggerProvider as _, Severity};
use opentelemetry::metrics::MeterProvider as _;
use opentelemetry::trace::{
    SpanContext, SpanId, SpanKind, Status, TraceFlags, TraceId, TraceState,
};
use opentelemetry::{InstrumentationScope, KeyValue};
use opentelemetry_sdk::logs::{LogBatch, LogExporter, SdkLoggerProvider};
use opentelemetry_sdk::metrics::exporter::PushMetricExporter;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::trace::{SpanData, SpanEvents, SpanExporter, SpanLinks};
use opentelemetry_sdk::Resource;
use std::future::Future;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// How long after its answer a command sends what it holds; what is not
/// sent by then is lost (216).
pub const BOUND: Duration = Duration::from_secs(3);

/// The signals OpenTelemetry's own variables in `env` leave on, as its
/// specification reads them and sluice reads them: `OTEL_SDK_DISABLED` at
/// `true` turns every signal off, and `OTEL_TRACES_EXPORTER`,
/// `OTEL_METRICS_EXPORTER` or `OTEL_LOGS_EXPORTER` at `none` that one,
/// each value read trimmed and in any case; no other variable is read,
/// and none turns a signal on (216; core.56).
fn left_on(env: &[(String, String)]) -> Sends {
    let mut on = Sends::ALL;
    let mut disabled = false;
    for (name, value) in env {
        let is = |word: &str| value.trim().eq_ignore_ascii_case(word);
        match name.as_str() {
            "OTEL_SDK_DISABLED" => disabled = is("true"),
            "OTEL_TRACES_EXPORTER" => on.traces = !is("none"),
            "OTEL_METRICS_EXPORTER" => on.metrics = !is("none"),
            "OTEL_LOGS_EXPORTER" => on.logs = !is("none"),
            _ => {}
        }
    }
    if disabled {
        Sends::default()
    } else {
        on
    }
}

/// What a command sends, chosen once as it begins (216; design 4): the
/// signals both the table and OpenTelemetry's variables leave on, none
/// where the table is absent or does not send; the table's collector; and
/// the resource it sends as.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chosen {
    sends: Sends,
    collector: Option<String>,
    resource: Vec<(String, String)>,
}

impl Chosen {
    /// What `table`, the managed layer's `[telemetry]`, and `env`, this
    /// process's environment, leave a command on the host named `host` to
    /// send: `service.name`, `service.version` and `host.name`, unless the
    /// table's attributes name one, then those attributes; no variable of
    /// the caller's read into the resource (216; sluice record 0106).
    pub fn choose(table: Option<&TelemetryTable>, host: &str, env: &[(String, String)]) -> Chosen {
        let Some(table) = table.filter(|table| table.send && table.collector.is_some()) else {
            return Chosen {
                sends: Sends::default(),
                collector: None,
                resource: vec![],
            };
        };
        let left = left_on(env);
        let sends = Sends {
            traces: table.traces && left.traces,
            metrics: table.metrics && left.metrics,
            logs: table.logs && left.logs,
        };
        let service = Some(table.service.clone())
            .filter(|service| !service.is_empty())
            .unwrap_or_else(|| TELEMETRY_SERVICE.into());
        let mut resource = vec![
            ("service.name".to_string(), service),
            ("service.version".to_string(), crate::VERSION.to_string()),
        ];
        if !table.attributes.contains_key("host.name") {
            resource.push(("host.name".to_string(), host.to_string()));
        }
        resource.extend(
            table
                .attributes
                .iter()
                .map(|(key, value)| (key.clone(), value.clone())),
        );
        Chosen {
            sends,
            collector: table.collector.clone(),
            resource,
        }
    }

    /// The signals the command sends.
    pub fn sends(&self) -> Sends {
        self.sends
    }

    /// The collector it sends to, none where it sends nothing.
    pub fn collector(&self) -> Option<&str> {
        self.collector.as_deref()
    }

    /// The resource it sends as, in order.
    pub fn resource(&self) -> &[(String, String)] {
        &self.resource
    }

    fn sdk_resource(&self) -> Resource {
        Resource::builder_empty()
            .with_attributes(
                self.resource
                    .iter()
                    .map(|(key, value)| KeyValue::new(key.clone(), value.clone())),
            )
            .build()
    }
}

/// The binary's telemetry: what it sends, and every signal the core
/// handed it while the command runs, kept in memory until [`finish`]
/// sends them (216; design 11).
#[derive(Debug)]
pub struct Held {
    chosen: Chosen,
    signals: Mutex<Vec<Signal>>,
}

impl Held {
    pub fn new(chosen: Chosen) -> Held {
        Held {
            chosen,
            signals: Mutex::default(),
        }
    }

    /// Every signal held, taken.
    fn take(&self) -> Vec<Signal> {
        std::mem::take(
            &mut *self
                .signals
                .lock()
                .unwrap_or_else(|poison| poison.into_inner()),
        )
    }
}

impl Telemetry for Held {
    fn sends(&self) -> Sends {
        self.chosen.sends
    }

    /// Keeps the signal where any goes; drops it otherwise.
    fn send(&self, signal: Signal) {
        if self.chosen.sends.any() {
            self.signals
                .lock()
                .unwrap_or_else(|poison| poison.into_inner())
                .push(signal);
        }
    }
}

/// The command's telemetry, chosen once, as the first engine it builds is
/// built.
static HELD: OnceLock<Arc<Held>> = OnceLock::new();

/// The command's telemetry, chosen at its first asking from `table` and
/// this process's environment as it stands then, and the same at every
/// later asking (216; design 4).
pub fn held(table: Option<&TelemetryTable>) -> Arc<Held> {
    HELD.get_or_init(|| {
        let env: Vec<(String, String)> = std::env::vars().collect();
        Arc::new(Held::new(Chosen::choose(table, &host_name(), &env)))
    })
    .clone()
}

/// Sends what the command's telemetry holds, once, after its answer was
/// written: every `OTEL_EXPORTER_OTLP_*` taken out of this process's
/// environment first, so no caller's collector, header or timeout reaches
/// an exporter; each signal within what remains of [`BOUND`]; a failure
/// silent and nothing kept (216; core.13; sluice record 0106).
pub fn finish() {
    let Some(held) = HELD.get() else {
        return;
    };
    let signals = held.take();
    let chosen = &held.chosen;
    let Some(collector) = chosen.collector() else {
        return;
    };
    if signals.is_empty() || !chosen.sends.any() {
        return;
    }
    let deadline = Instant::now() + BOUND;
    for (name, _) in std::env::vars_os() {
        if name.to_string_lossy().starts_with("OTEL_EXPORTER_OTLP_") {
            std::env::remove_var(name);
        }
    }
    use opentelemetry_otlp::{
        LogExporter as OtlpLogs, MetricExporter, Protocol, SpanExporter as OtlpSpans,
        WithExportConfig,
    };
    let left = || deadline.saturating_duration_since(Instant::now());
    let spans = chosen.sends.traces.then(|| {
        OtlpSpans::builder()
            .with_http()
            .with_endpoint(format!("{collector}/v1/traces"))
            .with_protocol(Protocol::HttpBinary)
            .with_timeout(left())
            .build()
            .ok()
    });
    let logs = chosen.sends.logs.then(|| {
        OtlpLogs::builder()
            .with_http()
            .with_endpoint(format!("{collector}/v1/logs"))
            .with_protocol(Protocol::HttpBinary)
            .with_timeout(left())
            .build()
            .ok()
    });
    let metrics = chosen.sends.metrics.then(|| {
        MetricExporter::builder()
            .with_http()
            .with_endpoint(format!("{collector}/v1/metrics"))
            .with_protocol(Protocol::HttpBinary)
            .with_temporality(opentelemetry_sdk::metrics::Temporality::Delta)
            .with_timeout(left())
            .build()
            .ok()
    });
    export(
        &signals,
        chosen,
        Exporters {
            spans: spans.flatten(),
            logs: logs.flatten(),
            metrics: metrics.flatten(),
        },
        deadline,
    );
}

/// The exporters a command sends through, one per signal it sends.
pub struct Exporters<S, L, M> {
    pub spans: Option<S>,
    pub logs: Option<L>,
    pub metrics: Option<M>,
}

/// Exports `signals` as `chosen` says through `exporters` until
/// `deadline`: the spans in one batch with the held ids, parents and
/// times; the records and warnings in one, each record under its span;
/// the metrics derived from them by one flush, delta and named by
/// [`METRIC_KEYS`] alone. What the deadline leaves no time for is
/// dropped, and a failure is silent (216 to 218, 220; design 10, 11).
pub fn export<S, L, M>(
    signals: &[Signal],
    chosen: &Chosen,
    exporters: Exporters<S, L, M>,
    deadline: Instant,
) where
    S: SpanExporter,
    L: LogExporter,
    M: PushMetricExporter,
{
    let resource = chosen.sdk_resource();
    let scope = InstrumentationScope::builder("flume")
        .with_version(crate::VERSION)
        .build();
    let open = || Instant::now() < deadline;
    if let (Some(mut exporter), true) = (exporters.spans, chosen.sends.traces) {
        let batch: Vec<SpanData> = signals
            .iter()
            .filter_map(|signal| span_data(signal, &scope))
            .collect();
        if !batch.is_empty() && open() {
            exporter.set_resource(&resource);
            let _ = block_on(exporter.export(batch));
        }
    }
    if let (Some(mut exporter), true) = (exporters.logs, chosen.sends.logs) {
        let provider = SdkLoggerProvider::builder().build();
        let logger = provider.logger("flume");
        let records: Vec<_> = signals
            .iter()
            .filter_map(|signal| {
                let mut record = logger.create_log_record();
                match signal {
                    Signal::Record {
                        trace,
                        span,
                        at,
                        event,
                        attributes,
                    } => {
                        record.set_body(AnyValue::String(event.clone().into()));
                        record.set_timestamp(time(*at));
                        record.set_observed_timestamp(time(*at));
                        record.set_severity_number(Severity::Info);
                        record.set_severity_text("INFO");
                        if let (Ok(trace), Ok(span)) =
                            (TraceId::from_hex(trace), SpanId::from_hex(span))
                        {
                            record.set_trace_context(trace, span, Some(TraceFlags::SAMPLED));
                        }
                        for (key, value) in attributes {
                            record.add_attribute(key.clone(), any_value(value));
                        }
                    }
                    Signal::Warning { at, words } => {
                        record.set_body(AnyValue::String(scrub(words).into()));
                        record.set_timestamp(time(*at));
                        record.set_observed_timestamp(time(*at));
                        record.set_severity_number(Severity::Warn);
                        record.set_severity_text("WARN");
                    }
                    _ => return None,
                }
                Some(record)
            })
            .collect();
        if !records.is_empty() && open() {
            let pairs: Vec<_> = records.iter().map(|record| (record, &scope)).collect();
            exporter.set_resource(&resource);
            let _ = block_on(exporter.export(LogBatch::new(&pairs)));
        }
    }
    if let (Some(exporter), true) = (exporters.metrics, chosen.sends.metrics) {
        if open() && signals.iter().any(measured) {
            let provider = SdkMeterProvider::builder()
                .with_periodic_exporter(exporter)
                .with_resource(resource)
                .build();
            measure(&provider, signals);
            let _ = provider.shutdown();
        }
    }
}

/// Whether `signal` makes a metric (220; design 10).
fn measured(signal: &Signal) -> bool {
    match signal {
        Signal::Span { attributes, .. } => {
            attributes.contains_key(keys::START_END) || attributes.contains_key(keys::WAIT_ON)
        }
        Signal::Record { event, .. } => event == "stall" || event == "stuck",
        Signal::Host(_) => true,
        Signal::Warning { .. } => false,
    }
}

/// Records each metric `signals` make on `provider`'s meter: a start's
/// time in `flume.start.duration` and a wait's in `flume.wait.duration`,
/// each in seconds; the stalls and stuck stages counted; and, from the
/// host's reading, `flume.places`, `flume.stream.agents` and
/// `flume.stream.room`; each named by [`METRIC_KEYS`] alone (220; design
/// 8, 10; core.56).
fn measure(provider: &SdkMeterProvider, signals: &[Signal]) {
    let meter = provider.meter("flume");
    let starts = meter
        .f64_histogram("flume.start.duration")
        .with_unit("s")
        .with_description("Each start of a stage, from its start to its end")
        .build();
    let waits = meter
        .f64_histogram("flume.wait.duration")
        .with_unit("s")
        .with_description("Each wait of a place, from the settle that read it to its end")
        .build();
    let stalls = meter
        .u64_counter("flume.stalls")
        .with_description("The places read stalled")
        .build();
    let stuck = meter
        .u64_counter("flume.stuck")
        .with_description("The stages read stuck")
        .build();
    let places = meter
        .u64_gauge("flume.places")
        .with_description("The places standing on the host")
        .build();
    let agents = meter
        .u64_gauge("flume.stream.agents")
        .with_description("A stream's agents running")
        .build();
    let room = meter
        .u64_gauge("flume.stream.room")
        .with_description("The agents that may yet start in a stream")
        .build();
    for signal in signals {
        match signal {
            Signal::Span {
                from,
                to,
                attributes,
                ..
            } => {
                let seconds = to.saturating_sub(*from) as f64;
                let named = metric_attributes(attributes);
                if attributes.contains_key(keys::START_END) {
                    starts.record(seconds, &named);
                } else if attributes.contains_key(keys::WAIT_ON) {
                    waits.record(seconds, &named);
                }
            }
            Signal::Record {
                event, attributes, ..
            } => match event.as_str() {
                "stall" => stalls.add(1, &metric_attributes(attributes)),
                "stuck" => stuck.add(1, &metric_attributes(attributes)),
                _ => {}
            },
            Signal::Host(reading) => {
                for standing in &reading.places {
                    let mut named = vec![
                        KeyValue::new(keys::KIND, standing.kind.clone()),
                        KeyValue::new(keys::STATE, standing.state.clone()),
                    ];
                    if let Some(on) = &standing.on {
                        named.push(KeyValue::new(keys::WAIT_ON, on.clone()));
                    }
                    places.record(standing.count, &named);
                }
                for stream in &reading.streams {
                    let named = [KeyValue::new(keys::STREAM, stream.stream.clone())];
                    agents.record(stream.running.into(), &named);
                    if let Some(left) = stream.room {
                        room.record(left.into(), &named);
                    }
                }
            }
            Signal::Warning { .. } => {}
        }
    }
}

/// The attributes of `attributes` a metric may carry, [`METRIC_KEYS`]
/// alone (220; core.56).
fn metric_attributes(attributes: &Attributes) -> Vec<KeyValue> {
    attributes
        .iter()
        .filter(|(key, _)| METRIC_KEYS.contains(&key.as_str()))
        .map(|(key, value)| key_value(key, value))
        .collect()
}

/// A span the held state named, with its own ids, parent and times.
fn span_data(signal: &Signal, scope: &InstrumentationScope) -> Option<SpanData> {
    let Signal::Span {
        trace,
        span,
        parent,
        name,
        from,
        to,
        attributes,
    } = signal
    else {
        return None;
    };
    let context = SpanContext::new(
        TraceId::from_hex(trace).ok()?,
        SpanId::from_hex(span).ok()?,
        TraceFlags::SAMPLED,
        false,
        TraceState::default(),
    );
    Some(SpanData {
        span_context: context,
        parent_span_id: parent
            .as_deref()
            .and_then(|parent| SpanId::from_hex(parent).ok())
            .unwrap_or(SpanId::INVALID),
        parent_span_is_remote: false,
        span_kind: SpanKind::Internal,
        name: name.clone().into(),
        start_time: time(*from),
        end_time: time(*to),
        attributes: attributes
            .iter()
            .map(|(key, value)| key_value(key, value))
            .collect(),
        dropped_attributes_count: 0,
        events: SpanEvents::default(),
        links: SpanLinks::default(),
        status: Status::Unset,
        instrumentation_scope: scope.clone(),
    })
}

/// A moment by the host's clock, in seconds since the epoch.
fn time(seconds: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(seconds)
}

fn key_value(key: &str, value: &Value) -> KeyValue {
    match value {
        Value::Text(text) => KeyValue::new(key.to_string(), text.clone()),
        Value::Number(number) => KeyValue::new(key.to_string(), *number),
        Value::Flag(flag) => KeyValue::new(key.to_string(), *flag),
    }
}

fn any_value(value: &Value) -> AnyValue {
    match value {
        Value::Text(text) => AnyValue::String(text.clone().into()),
        Value::Number(number) => AnyValue::Int(*number),
        Value::Flag(flag) => AnyValue::Boolean(*flag),
    }
}

/// `words`, a line of flume's own log, with each word that begins `/` or
/// `~/` replaced by `<path>` and each shaped as a credential by `<set>`
/// (218; core.50, core.56).
pub fn scrub(words: &str) -> String {
    let mut scrubbed = String::with_capacity(words.len());
    let mut rest = words;
    while !rest.is_empty() {
        let blank = rest.len() - rest.trim_start().len();
        scrubbed.push_str(&rest[..blank]);
        rest = &rest[blank..];
        let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
        let word = &rest[..end];
        if word.starts_with('/') || word.starts_with("~/") {
            scrubbed.push_str("<path>");
        } else if flume_core::shaped_as_credential("", word) {
            scrubbed.push_str(flume_seams::SET);
        } else {
            scrubbed.push_str(word);
        }
        rest = &rest[end..];
    }
    scrubbed
}

/// The host's own name, as the resource's `host.name` gives it.
fn host_name() -> String {
    let mut buffer = [0u8; 256];
    // SAFETY: the buffer is writable for its whole length, and gethostname
    // writes at most that many bytes.
    let read = unsafe { libc::gethostname(buffer.as_mut_ptr().cast(), buffer.len()) };
    if read != 0 {
        return String::new();
    }
    let end = buffer.iter().position(|byte| *byte == 0).unwrap_or(0);
    String::from_utf8_lossy(&buffer[..end]).into_owned()
}

/// Runs `future` on this thread to its end: the exporters' futures send
/// through the blocking client, so each is ready at its first poll.
fn block_on<F: Future>(future: F) -> F::Output {
    use std::task::{Context, Poll, Wake, Waker};
    struct Unpark(std::thread::Thread);
    impl Wake for Unpark {
        fn wake(self: Arc<Self>) {
            self.0.unpark();
        }
    }
    let waker = Waker::from(Arc::new(Unpark(std::thread::current())));
    let mut context = Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
        std::thread::park();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flume_seams::{HostReading, PlacesStanding, StreamAgents};
    use opentelemetry_sdk::logs::InMemoryLogExporter;
    use opentelemetry_sdk::metrics::data::{AggregatedMetrics, MetricData, ResourceMetrics};
    use opentelemetry_sdk::metrics::{InMemoryMetricExporterBuilder, Temporality};
    use opentelemetry_sdk::trace::InMemorySpanExporter;
    use std::collections::BTreeMap;

    const TRACE: &str = "0af7651916cd43dd8448eb211c80319c";
    const PLACE_SPAN: &str = "b7ad6b7169203331";
    const START_SPAN: &str = "00f067aa0ba902b7";
    const WAIT_SPAN: &str = "1111111111111111";

    fn env(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect()
    }

    fn sending() -> TelemetryTable {
        TelemetryTable {
            send: true,
            collector: Some("http://localhost:4318".into()),
            ..TelemetryTable::default()
        }
    }

    fn sends(table: Option<&TelemetryTable>, pairs: &[(&str, &str)]) -> Sends {
        Chosen::choose(table, "host-a", &env(pairs)).sends()
    }

    #[test]
    fn the_signals_the_table_and_otels_variables_leave_on() {
        // 216: a signal goes only while the table sends it and
        // OpenTelemetry's variables leave it on; no variable turns one on,
        // and nothing goes without a table that sends.
        let none = Sends::default();
        assert_eq!(sends(None, &[]), none);
        assert_eq!(
            sends(None, &[("OTEL_TRACES_EXPORTER", "otlp")]),
            none,
            "no table, nothing sent"
        );
        let quiet = TelemetryTable::default();
        assert_eq!(sends(Some(&quiet), &[]), none, "send unset");
        assert_eq!(sends(Some(&sending()), &[]), Sends::ALL);
        let off = TelemetryTable {
            traces: false,
            ..sending()
        };
        assert_eq!(
            sends(Some(&off), &[("OTEL_TRACES_EXPORTER", "otlp")]),
            Sends {
                traces: false,
                ..Sends::ALL
            },
            "a variable never turns a signal on"
        );
        for value in ["none", "NONE", " None "] {
            assert_eq!(
                sends(Some(&sending()), &[("OTEL_TRACES_EXPORTER", value)]),
                Sends {
                    traces: false,
                    ..Sends::ALL
                },
                "{value:?}"
            );
        }
        assert_eq!(
            sends(Some(&sending()), &[("OTEL_METRICS_EXPORTER", "none")]),
            Sends {
                metrics: false,
                ..Sends::ALL
            }
        );
        assert_eq!(
            sends(Some(&sending()), &[("OTEL_LOGS_EXPORTER", "none")]),
            Sends {
                logs: false,
                ..Sends::ALL
            }
        );
        for value in ["true", " TRUE "] {
            assert_eq!(
                sends(Some(&sending()), &[("OTEL_SDK_DISABLED", value)]),
                none,
                "{value:?}"
            );
        }
        for value in ["1", "false", ""] {
            assert_eq!(
                sends(Some(&sending()), &[("OTEL_SDK_DISABLED", value)]),
                Sends::ALL,
                "{value:?}"
            );
        }
        let chosen = Chosen::choose(
            Some(&sending()),
            "host-a",
            &env(&[("OTEL_EXPORTER_OTLP_ENDPOINT", "http://localhost:9999")]),
        );
        assert_eq!(chosen.collector(), Some("http://localhost:4318"));
    }

    #[test]
    fn the_resource_reads_no_variable_of_the_callers() {
        // 216; sluice record 0106: the resource is flume's service, its
        // version and the host's name, unless the table's attributes name
        // one, then those attributes; the caller's OTEL_RESOURCE_ATTRIBUTES
        // and OTEL_SERVICE_NAME are not read.
        let caller = env(&[
            ("OTEL_RESOURCE_ATTRIBUTES", "team=ops,host.name=elsewhere"),
            ("OTEL_SERVICE_NAME", "someone-else"),
        ]);
        let table = TelemetryTable {
            attributes: BTreeMap::from([("deployment.environment".into(), "lab".into())]),
            ..sending()
        };
        let chosen = Chosen::choose(Some(&table), "host-a", &caller);
        assert_eq!(
            chosen.resource(),
            [
                ("service.name".to_string(), "flume".to_string()),
                ("service.version".to_string(), crate::VERSION.to_string()),
                ("host.name".to_string(), "host-a".to_string()),
                ("deployment.environment".to_string(), "lab".to_string()),
            ]
        );
        let named = TelemetryTable {
            service: "flume-lab".into(),
            attributes: BTreeMap::from([("host.name".into(), "bench".into())]),
            ..sending()
        };
        let chosen = Chosen::choose(Some(&named), "host-a", &caller);
        assert_eq!(
            chosen.resource(),
            [
                ("service.name".to_string(), "flume-lab".to_string()),
                ("service.version".to_string(), crate::VERSION.to_string()),
                ("host.name".to_string(), "bench".to_string()),
            ]
        );
        assert!(Chosen::choose(None, "host-a", &caller)
            .resource()
            .is_empty());
    }

    fn text(value: &str) -> Value {
        Value::Text(value.into())
    }

    /// A place's facts, its intent, mark, conversation and account among
    /// them.
    fn place() -> Attributes {
        Attributes::from([
            (keys::REPOSITORY.into(), text("acme/widgets")),
            (keys::KIND.into(), text("change")),
            (keys::KIND_VERSION.into(), Value::Number(3)),
            (keys::PLACE.into(), text("accounts")),
            (keys::BASE.into(), text("main")),
            (keys::STREAM.into(), text("work")),
            (keys::INTENT.into(), text("accounts")),
            (keys::MARK.into(), text("FL-7")),
        ])
    }

    /// The signals of a start that ended short, a wait on its caller, a
    /// stall, the place's span, a warning and the host's reading.
    fn signals() -> Vec<Signal> {
        let mut start = place();
        start.extend([
            (keys::STAGE.into(), text("code")),
            (keys::START.into(), Value::Number(1)),
            (keys::PROGRAM.into(), text("claude")),
            (keys::SESSION_ID.into(), text("s-1")),
            (keys::ACCOUNT.into(), text("work/work-1")),
            (keys::START_END.into(), text("short")),
            (keys::START_NEEDS.into(), text("answer")),
        ]);
        let mut wait = place();
        wait.extend([
            (keys::STAGE.into(), text("code")),
            (keys::WAIT_ON.into(), text("caller")),
            (keys::WAIT_MOVE.into(), text("answer")),
            (keys::WAIT_HELD.into(), Value::Flag(false)),
            (keys::WAIT_STALLED.into(), Value::Flag(true)),
        ]);
        let mut stall = place();
        stall.insert(keys::EVENT_NAME.into(), text("stall"));
        vec![
            Signal::Span {
                trace: TRACE.into(),
                span: START_SPAN.into(),
                parent: Some(PLACE_SPAN.into()),
                name: "code".into(),
                from: 1_000,
                to: 4_600,
                attributes: start,
            },
            Signal::Record {
                trace: TRACE.into(),
                span: WAIT_SPAN.into(),
                at: 4_900,
                event: "stall".into(),
                attributes: stall,
            },
            Signal::Span {
                trace: TRACE.into(),
                span: WAIT_SPAN.into(),
                parent: Some(PLACE_SPAN.into()),
                name: "wait on caller".into(),
                from: 4_600,
                to: 7_000,
                attributes: wait,
            },
            Signal::Span {
                trace: TRACE.into(),
                span: PLACE_SPAN.into(),
                parent: None,
                name: "place".into(),
                from: 900,
                to: 10_000,
                attributes: place(),
            },
            Signal::Warning {
                at: 10_000,
                words: "the place /home/someone/places/change--accounts was not removed: ghp_0123456789abcdefghijABCDEFGHIJ012345".into(),
            },
            Signal::Host(HostReading {
                at: 10_000,
                places: vec![PlacesStanding {
                    kind: "change".into(),
                    state: "working".into(),
                    on: None,
                    count: 2,
                }],
                streams: vec![StreamAgents {
                    stream: "work".into(),
                    running: 2,
                    room: Some(1),
                }],
            }),
        ]
    }

    /// `signals` exported through in-memory exporters under a table that
    /// sends every signal.
    fn exported(
        signals: &[Signal],
    ) -> (
        InMemorySpanExporter,
        InMemoryLogExporter,
        Vec<ResourceMetrics>,
    ) {
        let spans = InMemorySpanExporter::default();
        let logs = InMemoryLogExporter::default();
        let metrics = InMemoryMetricExporterBuilder::new()
            .with_temporality(Temporality::Delta)
            .build();
        let chosen = Chosen::choose(Some(&sending()), "host-a", &[]);
        export(
            signals,
            &chosen,
            Exporters {
                spans: Some(spans.clone()),
                logs: Some(logs.clone()),
                metrics: Some(metrics.clone()),
            },
            Instant::now() + BOUND,
        );
        (
            spans,
            logs,
            metrics.get_finished_metrics().unwrap_or_default(),
        )
    }

    #[test]
    fn spans_go_with_the_held_ids_and_times() {
        // 217: each span goes with the trace, span and parent the held
        // state named and the times it kept, the place's with no parent;
        // each record under its span with its event as its body.
        let (spans, logs, _) = exported(&signals());
        let spans = spans.get_finished_spans().expect("the spans");
        assert_eq!(spans.len(), 3);
        let by_name = |name: &str| {
            spans
                .iter()
                .find(|span| span.name == name)
                .unwrap_or_else(|| panic!("no span {name}"))
        };
        let start = by_name("code");
        assert_eq!(start.span_context.trace_id().to_string(), TRACE);
        assert_eq!(start.span_context.span_id().to_string(), START_SPAN);
        assert_eq!(start.parent_span_id.to_string(), PLACE_SPAN);
        assert_eq!(start.start_time, UNIX_EPOCH + Duration::from_secs(1_000));
        assert_eq!(start.end_time, UNIX_EPOCH + Duration::from_secs(4_600));
        assert!(start
            .attributes
            .contains(&KeyValue::new(keys::START_END, "short")));
        assert!(start.attributes.contains(&KeyValue::new(keys::START, 1i64)));
        let place = by_name("place");
        assert_eq!(place.parent_span_id, SpanId::INVALID);
        assert_eq!(place.start_time, UNIX_EPOCH + Duration::from_secs(900));

        let logs = logs.get_emitted_logs().expect("the logs");
        assert_eq!(logs.len(), 2);
        let stall = &logs[0].record;
        assert_eq!(
            stall.body(),
            Some(&AnyValue::String("stall".to_string().into()))
        );
        let context = stall.trace_context().expect("under its span");
        assert_eq!(
            (context.trace_id.to_string(), context.span_id.to_string()),
            (TRACE.to_string(), WAIT_SPAN.to_string())
        );
        let resource = &logs[0].resource;
        assert_eq!(
            resource.get(&opentelemetry::Key::new("service.name")),
            Some(opentelemetry::Value::from("flume"))
        );
    }

    /// Each metric's name and every attribute key it carries.
    fn metric_keys(metrics: &[ResourceMetrics]) -> BTreeMap<String, Vec<String>> {
        let mut found: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for resource in metrics {
            for scope in resource.scope_metrics() {
                for metric in scope.metrics() {
                    let keys = found.entry(metric.name().to_string()).or_default();
                    let mut add = |attributes: Vec<&KeyValue>| {
                        keys.extend(attributes.iter().map(|kv| kv.key.to_string()));
                    };
                    match metric.data() {
                        AggregatedMetrics::F64(MetricData::Histogram(histogram)) => {
                            assert_eq!(histogram.temporality(), Temporality::Delta);
                            for point in histogram.data_points() {
                                add(point.attributes().collect());
                            }
                        }
                        AggregatedMetrics::U64(MetricData::Sum(sum)) => {
                            assert_eq!(sum.temporality(), Temporality::Delta);
                            for point in sum.data_points() {
                                add(point.attributes().collect());
                            }
                        }
                        AggregatedMetrics::U64(MetricData::Gauge(gauge)) => {
                            for point in gauge.data_points() {
                                add(point.attributes().collect());
                            }
                        }
                        other => panic!("{other:?}"),
                    }
                    keys.sort();
                    keys.dedup();
                }
            }
        }
        found
    }

    #[test]
    fn metrics_name_no_place_intent_mark_conversation_or_account() {
        // 220; core.56: each start's and wait's time, the stalls counted
        // and the host's reading, delta, each named by the metric keys
        // alone: none names a place, intent, mark, conversation or account.
        let (_, _, metrics) = exported(&signals());
        let found = metric_keys(&metrics);
        assert_eq!(
            found.keys().map(String::as_str).collect::<Vec<_>>(),
            [
                "flume.places",
                "flume.stalls",
                "flume.start.duration",
                "flume.stream.agents",
                "flume.stream.room",
                "flume.wait.duration",
            ]
        );
        assert_eq!(
            found["flume.start.duration"],
            [
                "flume.kind",
                "flume.repository",
                "flume.stage",
                "flume.start.end",
                "flume.stream"
            ]
        );
        assert_eq!(
            found["flume.wait.duration"],
            [
                "flume.kind",
                "flume.repository",
                "flume.stage",
                "flume.stream",
                "flume.wait.move",
                "flume.wait.on"
            ]
        );
        assert_eq!(found["flume.places"], ["flume.kind", "flume.state"]);
        assert_eq!(found["flume.stream.room"], ["flume.stream"]);
        for keys in found.values() {
            for key in keys {
                assert!(METRIC_KEYS.contains(&key.as_str()), "{key}");
            }
        }
        let shown = format!("{metrics:?}");
        for word in ["accounts", "FL-7", "s-1", "work-1"] {
            assert!(!shown.contains(word), "{word} in a metric");
        }
    }

    #[test]
    fn a_warning_is_scrubbed_of_paths_and_credentials() {
        // 218; core.50: a warning goes in flume's words, each word that
        // begins `/` or `~/` replaced by <path> and each shaped as a
        // credential by <set>.
        assert_eq!(
            scrub("the place /home/someone/places/fix--one was not tidied: ~/notes at  ghp_0123456789abcdefghijABCDEFGHIJ012345 kept"),
            "the place <path> was not tidied: <path> at  <set> kept"
        );
        assert_eq!(scrub("nothing to hide"), "nothing to hide");
        let (_, logs, _) = exported(&signals());
        let logs = logs.get_emitted_logs().expect("the logs");
        let warning = &logs[1].record;
        assert_eq!(
            warning.body(),
            Some(&AnyValue::String(
                "the place <path> was not removed: <set>".to_string().into()
            ))
        );
        assert_eq!(warning.severity_number(), Some(Severity::Warn));
    }
}

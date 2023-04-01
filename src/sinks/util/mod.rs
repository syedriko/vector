pub mod adaptive_concurrency;
pub mod batch;
pub mod buffer;
pub mod builder;
pub mod compressor;
pub mod encoding;
pub mod http;
pub mod normalizer;
pub mod partitioner;
pub mod processed_event;
pub mod request_builder;
pub mod retries;
pub mod service;
pub mod sink;
pub mod socket_bytes_sink;
pub mod statistic;
pub mod tcp;
#[cfg(test)]
pub mod test;
pub mod udp;
#[cfg(all(any(feature = "sinks-socket", feature = "sinks-statsd"), unix))]
pub mod unix;
pub mod uri;

use std::borrow::Cow;

pub use batch::{
    Batch, BatchConfig, BatchSettings, BatchSize, BulkSizeBasedDefaultBatchSettings, Merged,
    NoDefaultsBatchSettings, PushResult, RealtimeEventBasedDefaultBatchSettings,
    RealtimeSizeBasedDefaultBatchSettings, SinkBatchSettings, Unmerged,
};
pub use buffer::{
    json::{BoxedRawValue, JsonArrayBuffer},
    partition::Partition,
    vec::{EncodedLength, VecBuffer},
    Buffer, Compression, PartitionBuffer, PartitionInnerBuffer,
};
pub use builder::SinkBuilderExt;
use bytes::Bytes;
pub use compressor::Compressor;
use encoding::{EncodingConfig, EncodingConfiguration};
pub use normalizer::Normalizer;
pub use request_builder::{IncrementalRequestBuilder, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde::de;
pub use service::{
    Concurrency, ServiceBuilderExt, TowerBatchedSink, TowerPartitionSink, TowerRequestConfig,
    TowerRequestLayer, TowerRequestSettings,
};
pub use sink::{BatchSink, PartitionBatchSink, StreamSink};
use snafu::Snafu;
pub use uri::UriSerde;
use chrono::{DateTime, SecondsFormat, Local};
use value::Value;
use crate::event::{Event, LogEvent, EventFinalizers};

#[derive(Debug, Snafu)]
enum SinkBuildError {
    #[snafu(display("Missing host in address field"))]
    MissingHost,
    #[snafu(display("Missing port in address field"))]
    MissingPort,
}

#[derive(Debug)]
pub struct EncodedEvent<I> {
    pub item: I,
    pub finalizers: EventFinalizers,
    pub byte_size: usize,
}

impl<I> EncodedEvent<I> {
    /// Create a trivial input with no metadata. This method will be
    /// removed when all sinks are converted.
    pub fn new(item: I, byte_size: usize) -> Self {
        Self {
            item,
            finalizers: Default::default(),
            byte_size,
        }
    }

    // This should be:
    // ```impl<F, I: From<F>> From<EncodedEvent<F>> for EncodedEvent<I>```
    // however, the compiler rejects that due to conflicting
    // implementations of `From` due to the generic
    // ```impl<T> From<T> for T```
    pub fn from<F>(that: EncodedEvent<F>) -> Self
    where
        I: From<F>,
    {
        Self {
            item: I::from(that.item),
            finalizers: that.finalizers,
            byte_size: that.byte_size,
        }
    }

    /// Remap the item using an adapter
    pub fn map<T>(self, doit: impl Fn(I) -> T) -> EncodedEvent<T> {
        EncodedEvent {
            item: doit(self.item),
            finalizers: self.finalizers,
            byte_size: self.byte_size,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SyslogRFC {
    Rfc3164,
    Rfc5424
}

impl Default for SyslogRFC {
    fn default() -> Self {
        SyslogRFC::Rfc5424
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
enum Facility {
    Fixed(u8),
    Field(String)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
enum Severity {
    Fixed(u8),
    Field(String)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SyslogConf {
    #[serde(default)]
    rfc: SyslogRFC,
    #[serde(default = "default_facility")]
    #[serde(deserialize_with = "deserialize_facility")]
    facility: Facility,
    #[serde(default = "default_severity")]
    #[serde(deserialize_with = "deserialize_severity")]
    severity: Severity,
    #[serde(default)]
    tag: String,
    trim_prefix: Option<String>,
    #[serde(default)]
    payload_key: String,
    #[serde(default)]
    add_log_source: bool,
    // rfc5424 only
    #[serde(default)]
    app_name: String,
    #[serde(default)]
    proc_id: String,
    #[serde(default)]
    msg_id: String
}

fn default_facility() -> Facility {
    Facility::Fixed(0)
}

fn deserialize_facility<'de, D>(d: D) -> Result<Facility, D::Error>
    where D: de::Deserializer<'de>
{
    let value: String = String::deserialize(d)?;
    let num_value = value.parse::<u8>();
    match num_value {
        Ok(num) => {
            if num > 23 {
                return Err(de::Error::invalid_value(de::Unexpected::Unsigned(num as u64), &"facility number too large"));
            } else {
                return Ok(Facility::Fixed(num));
            }
        }
        Err(_) => {
            if let Some(field_name) = value.strip_prefix("$.message.") {
                return Ok(Facility::Field(field_name.to_string()));
            } else {
                let num = match value.to_uppercase().as_str() {
                    "KERN" => 0,
                    "USER" => 1,
                    "MAIL" => 2,
                    "DAEMON" => 3,
                    "AUTH" => 4,
                    "SYSLOG" => 5,
                    "LPR" => 6,
                    "NEWS" => 7,
                    "UUCP" => 8,
                    "CRON" => 9,
                    "AUTHPRIV" => 10,
                    "FTP" => 11,
                    "NTP" => 12,
                    "SECURITY" => 13,
                    "CONSOLE" => 14,
                    "SOLARIS-CRON" => 15,
                    "LOCAL0" => 16,
                    "LOCAL1" => 17,
                    "LOCAL2" => 18,
                    "LOCAL3" => 19,
                    "LOCAL4" => 20,
                    "LOCAL5" => 21,
                    "LOCAL6" => 22,
                    "LOCAL7" => 23,
                    _ => 24,
                };
                if num > 23 {
                    return Err(de::Error::invalid_value(de::Unexpected::Unsigned(num as u64), &"unknown facility"));
                } else {
                    return Ok(Facility::Fixed(num))
                }
            }
        }
    }
}

fn default_severity() -> Severity {
    Severity::Fixed(0)
}

fn deserialize_severity<'de, D>(d: D) -> Result<Severity, D::Error>
    where D: de::Deserializer<'de>
{
    let value: String = String::deserialize(d)?;
    let num_value = value.parse::<u8>();
    match num_value {
        Ok(num) => {
            if num > 7 {
                return Err(de::Error::invalid_value(de::Unexpected::Unsigned(num as u64), &"severity number too large"))
            } else {
                return Ok(Severity::Fixed(num))
            }
        }
        Err(_) => {
            if let Some(field_name) = value.strip_prefix("$.message.") {
                return Ok(Severity::Field(field_name.to_string()));
            } else {
                let num = match value.to_uppercase().as_str() {
                    "EMERGENCY" => 0,
                    "ALERT" => 1,
                    "CRITICAL" => 2,
                    "ERROR" => 3,
                    "WARNING" => 4,
                    "NOTICE" => 5,
                    "INFORMATIONAL" => 6,
                    "DEBUG" => 7,
                    _ => 8,
                };
                if num > 7 {
                    return Err(de::Error::invalid_value(de::Unexpected::Unsigned(num as u64), &"unknown severity"))
                } else {
                    return Ok(Severity::Fixed(num))
                }
            }
        }
    }
}

/**
 * Enum representing different ways to encode events as they are sent into a Sink.
 */
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Encoding {
    Text,
    Json,
    Syslog(SyslogConf),
}

fn add_log_source(log: &LogEvent, buf: &mut String) {
    buf.push_str("namespace_name=");
    buf.push_str(&String::from_utf8(
        log
        .get("kubernetes.namespace_name")
        .map(|h| h.coerce_to_bytes())
        .unwrap_or_default().to_vec()
    ).unwrap());
    buf.push_str(", container_name=");
    buf.push_str(&String::from_utf8(
        log
        .get("kubernetes.container_name")
        .map(|h| h.coerce_to_bytes())
        .unwrap_or_default().to_vec()
    ).unwrap());
    buf.push_str(", pod_name=");
    buf.push_str(&String::from_utf8(
        log
        .get("kubernetes.pod_name")
        .map(|h| h.coerce_to_bytes())
        .unwrap_or_default().to_vec()
    ).unwrap());
    buf.push_str(", message=");
}

fn get_num_facility(config_facility: &Facility, log: &LogEvent) -> u8 {
    match config_facility {
        Facility::Fixed(num) => return *num,
        Facility::Field(field_name) => {
            if let Some(field_value) = log.get(field_name.as_str()) {
                let field_value_string = String::from_utf8(field_value.coerce_to_bytes().to_vec()).unwrap_or_default();
                let num_value = field_value_string.parse::<u8>();
                match num_value {
                    Ok(num) => {
                        if num > 23 {
                            return 1 // USER
                        } else {
                            return num
                        }
                    }
                    Err(_) => {
                            let num = match field_value_string.to_uppercase().as_str() {
                                "KERN" => 0,
                                "USER" => 1,
                                "MAIL" => 2,
                                "DAEMON" => 3,
                                "AUTH" => 4,
                                "SYSLOG" => 5,
                                "LPR" => 6,
                                "NEWS" => 7,
                                "UUCP" => 8,
                                "CRON" => 9,
                                "AUTHPRIV" => 10,
                                "FTP" => 11,
                                "NTP" => 12,
                                "SECURITY" => 13,
                                "CONSOLE" => 14,
                                "SOLARIS-CRON" => 15,
                                "LOCAL0" => 16,
                                "LOCAL1" => 17,
                                "LOCAL2" => 18,
                                "LOCAL3" => 19,
                                "LOCAL4" => 20,
                                "LOCAL5" => 21,
                                "LOCAL6" => 22,
                                "LOCAL7" => 23,
                                _ => 24,
                            };
                            if num > 23 {
                                return 1 // USER
                            } else {
                                return num
                            }
                        }
                    }
            } else {
                return 1 // USER
            }
        }
    }
}

fn get_num_severity(config_severity: &Severity, log: &LogEvent) -> u8 {
    match config_severity {
        Severity::Fixed(num) => return *num,
        Severity::Field(field_name) => {
            if let Some(field_value) = log.get(field_name.as_str()) {
                let field_value_string = String::from_utf8(field_value.coerce_to_bytes().to_vec()).unwrap_or_default();
                let num_value = field_value_string.parse::<u8>();
                match num_value {
                    Ok(num) => {
                        if num > 7 {
                            return 6 // INFORMATIONAL
                        } else {
                            return num
                        }
                    }
                    Err(_) => {
                            let num = match field_value_string.to_uppercase().as_str() {
                                "EMERGENCY" => 0,
                                "ALERT" => 1,
                                "CRITICAL" => 2,
                                "ERROR" => 3,
                                "WARNING" => 4,
                                "NOTICE" => 5,
                                "INFORMATIONAL" => 6,
                                "DEBUG" => 7,
                                _ => 8,
                            };
                            if num > 7 {
                                return 6 // INFORMATIONAL
                            } else {
                                return num
                            }
                        }
                    }
            } else {
                return 6 // INFORMATIONAL
            }
        }
    }
}

fn get_field_or_config(config_name: &String, log: &LogEvent) -> String {
    if let Some(field_name) = config_name.strip_prefix("$.message.") {
        return get_field(field_name, log)
    } else {
        return config_name.clone()
    }
}

fn get_field(field_name: &str, log: &LogEvent) -> String {
    if let Some(field_value) = log.get(field_name) {
        return String::from_utf8(field_value.coerce_to_bytes().to_vec()).unwrap_or_default();
    } else {
        return "-".to_string()
    }
}

fn get_timestamp(log: &LogEvent) -> DateTime::<Local> {
    match log.get("@timestamp") {
        Some(value) => {
            if let Value::Timestamp(timestamp) = value {
                DateTime::<Local>::from(*timestamp)
            } else {
                Local::now()
            }
        },
        _ => Local::now()
    }
}

/**
* Encodes the given event into raw bytes that can be sent into a Sink, according to
* the given encoding. If there are any errors encoding the event, logs a warning
* and returns None.
**/
pub fn encode_log(mut event: Event, encoding: &EncodingConfig<Encoding>) -> Option<Bytes> {
    encoding.apply_rules(&mut event);
    let log = event.into_log();

    let b = match encoding.codec() {
        Encoding::Json => serde_json::to_vec(&log),
        Encoding::Text => {
            let bytes = log
                .get(crate::config::log_schema().message_key())
                .map(|v| v.coerce_to_bytes().to_vec())
                .unwrap_or_default();
            Ok(bytes)
        }
        Encoding::Syslog(config) => {
            let mut buf = String::from("<");
            let pri = get_num_facility(&config.facility, &log) * 8 + get_num_severity(&config.severity, &log);
            buf.push_str(&pri.to_string());
            buf.push_str(">");
            match config.rfc {
                SyslogRFC::Rfc3164 => {
                    let timestamp = get_timestamp(&log);
                    let formatted_timestamp = format!(" {} ", timestamp.format("%b %e %H:%M:%S"));
                    buf.push_str(&formatted_timestamp);
                    buf.push_str(&get_field("hostname", &log));
                    buf.push(' ');
                    buf.push_str(&get_field_or_config(&config.tag, &log));
                    buf.push_str(": ");
                    if config.add_log_source {
                        add_log_source(&log, &mut buf);
                    }
                },
                SyslogRFC::Rfc5424 => {
                    buf.push_str("1 ");
                    let timestamp = get_timestamp(&log);
                    buf.push_str(&timestamp.to_rfc3339_opts(SecondsFormat::Millis, true));
                    buf.push(' ');
                    buf.push_str(&get_field("hostname", &log));
                    buf.push(' ');
                    buf.push_str(&get_field_or_config(&&config.app_name, &log));
                    buf.push(' ');
                    buf.push_str(&get_field_or_config(&&config.proc_id, &log));
                    buf.push(' ');
                    buf.push_str(&get_field_or_config(&&config.msg_id, &log));
                    buf.push_str(" - "); // no structured data
                    if config.add_log_source {
                        add_log_source(&log, &mut buf);
                    }
                }
            }
            let mut payload = if config.payload_key.is_empty() {
                serde_json::to_vec(&log).unwrap_or_default()
            } else {
                get_field(&&config.payload_key, &log).as_bytes().to_vec()
            };
            let mut vec = buf.as_bytes().to_vec();
            vec.append(&mut payload);
            Ok(vec)
        }
    };

    b.map(|mut b| {
        b.push(b'\n');
        Bytes::from(b)
    })
    .map_err(|error| error!(message = "Unable to encode.", %error))
    .ok()
}

/// Joins namespace with name via delimiter if namespace is present.
pub fn encode_namespace<'a>(
    namespace: Option<&str>,
    delimiter: char,
    name: impl Into<Cow<'a, str>>,
) -> String {
    let name = name.into();
    namespace
        .map(|namespace| format!("{}{}{}", namespace, delimiter, name))
        .unwrap_or_else(|| name.into_owned())
}

/// Marker trait for types that can hold a batch of events
pub trait ElementCount {
    fn element_count(&self) -> usize;
}

impl<T> ElementCount for Vec<T> {
    fn element_count(&self) -> usize {
        self.len()
    }
}

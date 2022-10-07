use crate::encoding::BuildError;
use bytes::{BufMut, BytesMut};
use tokio_util::codec::Encoder;
use vector_config::configurable_component;
use vector_core::{config::{DataType, log_schema}, event::Event, schema};

/// Config used to build a `SyslogSerializer`.
#[configurable_component]
#[derive(Clone, Debug)]
pub struct SyslogSerializerConfig {
    /// Options for the Syslog serializer.
    #[configurable(derived)]
    #[serde(flatten)]
    pub syslog: SyslogSerializerOptions
}

impl SyslogSerializerConfig {
    /// Creates a new `SyslogSerializerConfig`.
    pub const fn new(syslog_options: SyslogSerializerOptions) -> Self {
        Self {
            syslog: syslog_options
        }
    }

    /// Build the `SyslogSerializer` from this configuration.
    pub fn build(&self) -> Result<SyslogSerializer, BuildError> {
        Ok(SyslogSerializer::new(self.syslog.clone()))
    }

    /// The data type of events that are accepted by `SyslogSerializer`.
    pub fn input_type() -> DataType {
        DataType::Log
    }

    /// The schema required by the serializer.
    pub fn schema_requirement() -> schema::Requirement {
        schema::Requirement::empty()
    }
}

/// Syslog serializer options.
#[configurable_component]
#[derive(Clone, Debug)]
//#[serde(rename_all = "snake_case")]
pub struct SyslogSerializerOptions {
    /// the syslog RFC to conform to
    #[configurable(derived)]
    pub rfc: Rfc
}

/// Socket mode.
#[configurable_component]
#[derive(Clone, Debug)]
//#[serde(rename_all = "snake_case")]
pub enum Rfc {
    /// syslog according to RFC 3164
    Rfc3164,

    /// syslog according to RFC 5424
    Rfc5424
}

/*
/// Socket mode.
#[configurable_component]
#[derive(Clone, Debug)]
#[serde(tag = "", rename_all = "snake_case")]
pub enum Mode {
    /// TCP.
    Tcp(#[configurable(transparent)] TcpMode),

    /// UDP.
    Udp(#[configurable(transparent)] UdpMode),

    /// Unix Domain Socket.
    #[cfg(unix)]
    Unix(#[configurable(transparent)] UnixMode),
}

/// TCP configuration.
#[configurable_component]
#[derive(Clone, Debug)]
pub struct TcpMode {
    #[serde(flatten)]
    config: TcpSinkConfig,

    #[serde(flatten)]
    encoding: EncodingConfigWithFraming,
}
*/

/// Serializer that converts an `Event` to bytes using the Syslog format.
#[derive(Debug, Clone)]
pub struct SyslogSerializer {
    /// Options for the Syslog serializer.
    pub syslog: SyslogSerializerOptions
}

impl SyslogSerializer {
    /// Creates a new `SyslogSerializer`.
    pub const fn new(syslog_options: SyslogSerializerOptions) -> Self {
        Self { syslog: syslog_options}
    }
}

impl Encoder<Event> for SyslogSerializer {
    type Error = vector_common::Error;

    fn encode(&mut self, event: Event, buffer: &mut BytesMut) -> Result<(), Self::Error> {
        let message_key = log_schema().message_key();

        let log = event.as_log();

        if let Some(bytes) = log
            .get_by_meaning(message_key)
            .or_else(|| log.get(message_key))
            .map(|value| value.coerce_to_bytes())
        {
            buffer.put(bytes);
        }

        Ok(())
    }
}

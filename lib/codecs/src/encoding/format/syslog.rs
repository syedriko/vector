use vector_core::{config::log_schema, event::Event, schema};

use bytes::{BufMut, BytesMut};
use serde::{Deserialize, Serialize};
use tokio_util::codec::Encoder;
use value::Kind;

/// Config used to build a `SyslogSerializer`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SyslogSerializerConfig;

impl SyslogSerializerConfig {
    /// Creates a new `SyslogSerializerConfig`.
    pub const fn new() -> Self {
        Self
    }

    /// Build the `SyslogSerializer` from this configuration.
    pub const fn build(&self) -> SyslogSerializer {
        SyslogSerializer
    }

    /// The schema required by the serializer.
    pub fn schema_requirement(&self) -> schema::Requirement {
        schema::Requirement::empty().require_meaning(log_schema().message_key(), Kind::any())
    }
}

/// Serializer that converts an `Event` to bytes by extracting the message key.
#[derive(Debug, Clone)]
pub struct SyslogSerializer;

impl SyslogSerializer {
    /// Creates a new `SyslogSerializer`.
    pub const fn new() -> Self {
        Self
    }
}

impl Encoder<Event> for SyslogSerializer {
    type Error = vector_core::Error;

    fn encode(&mut self, event: Event, buffer: &mut BytesMut) -> Result<(), Self::Error> {
        let message_key = log_schema().message_key();

        let bytes = match event {
            Event::Log(log) => log
                .get_by_meaning(message_key)
                .or_else(|| log.get(message_key)) // backward compatibility
                .map(|value| value.coerce_to_bytes()),
            Event::Metric(_) => None,
            Event::Trace(_) => None,
        };

        if let Some(bytes) = bytes {
            buffer.put(bytes);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{Bytes, BytesMut};

    #[test]
    fn serialize_bytes() {
        let input = Event::from("foo");
        let mut serializer = SyslogSerializer;

        let mut buffer = BytesMut::new();
        serializer.encode(input, &mut buffer).unwrap();

        assert_eq!(buffer.freeze(), Bytes::from("foo"));
    }
}

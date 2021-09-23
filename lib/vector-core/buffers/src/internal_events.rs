use internal_event::InternalEvent;
use metrics::{counter, decrement_gauge, gauge, increment_gauge};

pub enum BufferMaxSize {
    Events(usize),
    Bytes(usize),
}
pub struct BufferCreated {
    pub max_size: BufferMaxSize,
    pub id: String,
}

impl InternalEvent for BufferCreated {
    fn emit_logs(&self) {}

    fn emit_metrics(&self) {
        match self.max_size {
            BufferMaxSize::Events(size) => {
                gauge!("buffer_max_event_size", size as f64, "component_id" => self.id.clone());
            }
            BufferMaxSize::Bytes(size) => {
                gauge!("buffer_max_byte_size", size as f64, "component_id" => self.id.clone());
            }
        }
    }
}

pub struct EventsReceived {
    pub count: usize,
    pub byte_size: usize,
}

impl InternalEvent for EventsReceived {
    fn emit_logs(&self) {}

    fn emit_metrics(&self) {
        counter!("buffer_received_events_total", self.count as u64);
        counter!("buffer_received_bytes_total", self.byte_size as u64);
        increment_gauge!("buffer_events", self.count as f64);
        increment_gauge!("buffer_byte_size", self.byte_size as f64);
    }
}

pub struct EventsSent {
    pub count: usize,
    pub byte_size: usize,
}

impl InternalEvent for EventsSent {
    fn emit_logs(&self) {}

    fn emit_metrics(&self) {
        counter!("buffer_sent_events_total", self.count as u64);
        counter!("buffer_sent_bytes_total", self.byte_size as u64);
        decrement_gauge!("buffer_events", self.count as f64);
        decrement_gauge!("buffer_byte_size", self.byte_size as f64);
    }
}

pub struct EventsDropped {
    pub count: usize,
}

impl InternalEvent for EventsDropped {
    fn emit_logs(&self) {}

    fn emit_metrics(&self) {
        counter!("buffer_discarded_events_total", self.count as u64);
    }
}

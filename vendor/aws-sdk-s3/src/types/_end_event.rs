// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A message that indicates the request is complete and no more messages will be sent. You should not assume that the request is complete until the client receives an <code>EndEvent</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EndEvent {}
impl EndEvent {
    /// Creates a new builder-style object to manufacture [`EndEvent`](crate::types::EndEvent).
    pub fn builder() -> crate::types::builders::EndEventBuilder {
        crate::types::builders::EndEventBuilder::default()
    }
}

/// A builder for [`EndEvent`](crate::types::EndEvent).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct EndEventBuilder {}
impl EndEventBuilder {
    /// Consumes the builder and constructs a [`EndEvent`](crate::types::EndEvent).
    pub fn build(self) -> crate::types::EndEvent {
        crate::types::EndEvent {}
    }
}

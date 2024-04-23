// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteMessageInput {
    /// <p>The URL of the Amazon SQS queue from which messages are deleted.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub queue_url: ::std::option::Option<::std::string::String>,
    /// <p>The receipt handle associated with the message to delete.</p>
    pub receipt_handle: ::std::option::Option<::std::string::String>,
}
impl DeleteMessageInput {
    /// <p>The URL of the Amazon SQS queue from which messages are deleted.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn queue_url(&self) -> ::std::option::Option<&str> {
        self.queue_url.as_deref()
    }
    /// <p>The receipt handle associated with the message to delete.</p>
    pub fn receipt_handle(&self) -> ::std::option::Option<&str> {
        self.receipt_handle.as_deref()
    }
}
impl DeleteMessageInput {
    /// Creates a new builder-style object to manufacture [`DeleteMessageInput`](crate::operation::delete_message::DeleteMessageInput).
    pub fn builder() -> crate::operation::delete_message::builders::DeleteMessageInputBuilder {
        crate::operation::delete_message::builders::DeleteMessageInputBuilder::default()
    }
}

/// A builder for [`DeleteMessageInput`](crate::operation::delete_message::DeleteMessageInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteMessageInputBuilder {
    pub(crate) queue_url: ::std::option::Option<::std::string::String>,
    pub(crate) receipt_handle: ::std::option::Option<::std::string::String>,
}
impl DeleteMessageInputBuilder {
    /// <p>The URL of the Amazon SQS queue from which messages are deleted.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    /// This field is required.
    pub fn queue_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.queue_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The URL of the Amazon SQS queue from which messages are deleted.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn set_queue_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.queue_url = input;
        self
    }
    /// <p>The URL of the Amazon SQS queue from which messages are deleted.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn get_queue_url(&self) -> &::std::option::Option<::std::string::String> {
        &self.queue_url
    }
    /// <p>The receipt handle associated with the message to delete.</p>
    /// This field is required.
    pub fn receipt_handle(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.receipt_handle = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The receipt handle associated with the message to delete.</p>
    pub fn set_receipt_handle(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.receipt_handle = input;
        self
    }
    /// <p>The receipt handle associated with the message to delete.</p>
    pub fn get_receipt_handle(&self) -> &::std::option::Option<::std::string::String> {
        &self.receipt_handle
    }
    /// Consumes the builder and constructs a [`DeleteMessageInput`](crate::operation::delete_message::DeleteMessageInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_message::DeleteMessageInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::delete_message::DeleteMessageInput {
            queue_url: self.queue_url,
            receipt_handle: self.receipt_handle,
        })
    }
}

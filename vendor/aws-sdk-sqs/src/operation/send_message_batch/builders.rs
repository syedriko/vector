// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::send_message_batch::_send_message_batch_output::SendMessageBatchOutputBuilder;

pub use crate::operation::send_message_batch::_send_message_batch_input::SendMessageBatchInputBuilder;

impl SendMessageBatchInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::send_message_batch::SendMessageBatchOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::send_message_batch::SendMessageBatchError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.send_message_batch();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SendMessageBatch`.
///
/// <p>You can use <code>SendMessageBatch</code> to send up to 10 messages to the specified queue by assigning either identical or different values to each message (or by not assigning values at all). This is a batch version of <code> <code>SendMessage</code>.</code> For a FIFO queue, multiple messages within a single batch are enqueued in the order they are sent.</p>
/// <p>The result of sending each message is reported individually in the response. Because the batch request can result in a combination of successful and unsuccessful actions, you should check for batch errors even when the call returns an HTTP status code of <code>200</code>.</p>
/// <p>The maximum allowed individual message size and the maximum total payload size (the sum of the individual lengths of all of the batched messages) are both 256 KiB (262,144 bytes).</p> <important>
/// <p>A message can include only XML, JSON, and unformatted text. The following Unicode characters are allowed:</p>
/// <p> <code>#x9</code> | <code>#xA</code> | <code>#xD</code> | <code>#x20</code> to <code>#xD7FF</code> | <code>#xE000</code> to <code>#xFFFD</code> | <code>#x10000</code> to <code>#x10FFFF</code> </p>
/// <p>Any characters not included in this list will be rejected. For more information, see the <a href="http://www.w3.org/TR/REC-xml/#charsets">W3C specification for characters</a>.</p>
/// </important>
/// <p>If you don't specify the <code>DelaySeconds</code> parameter for an entry, Amazon SQS uses the default value for the queue.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SendMessageBatchFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::send_message_batch::builders::SendMessageBatchInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::send_message_batch::SendMessageBatchOutput,
        crate::operation::send_message_batch::SendMessageBatchError,
    > for SendMessageBatchFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::send_message_batch::SendMessageBatchOutput,
            crate::operation::send_message_batch::SendMessageBatchError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SendMessageBatchFluentBuilder {
    /// Creates a new `SendMessageBatch`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SendMessageBatch as a reference.
    pub fn as_input(&self) -> &crate::operation::send_message_batch::builders::SendMessageBatchInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::send_message_batch::SendMessageBatchOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::send_message_batch::SendMessageBatchError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::send_message_batch::SendMessageBatch::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::send_message_batch::SendMessageBatch::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::send_message_batch::SendMessageBatchOutput,
        crate::operation::send_message_batch::SendMessageBatchError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The URL of the Amazon SQS queue to which batched messages are sent.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn queue_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.queue_url(input.into());
        self
    }
    /// <p>The URL of the Amazon SQS queue to which batched messages are sent.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn set_queue_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_queue_url(input);
        self
    }
    /// <p>The URL of the Amazon SQS queue to which batched messages are sent.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn get_queue_url(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_queue_url()
    }
    /// Appends an item to `Entries`.
    ///
    /// To override the contents of this collection use [`set_entries`](Self::set_entries).
    ///
    /// <p>A list of <code> <code>SendMessageBatchRequestEntry</code> </code> items.</p>
    pub fn entries(mut self, input: crate::types::SendMessageBatchRequestEntry) -> Self {
        self.inner = self.inner.entries(input);
        self
    }
    /// <p>A list of <code> <code>SendMessageBatchRequestEntry</code> </code> items.</p>
    pub fn set_entries(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SendMessageBatchRequestEntry>>) -> Self {
        self.inner = self.inner.set_entries(input);
        self
    }
    /// <p>A list of <code> <code>SendMessageBatchRequestEntry</code> </code> items.</p>
    pub fn get_entries(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SendMessageBatchRequestEntry>> {
        self.inner.get_entries()
    }
}

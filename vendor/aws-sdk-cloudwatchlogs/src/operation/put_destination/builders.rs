// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_destination::_put_destination_output::PutDestinationOutputBuilder;

pub use crate::operation::put_destination::_put_destination_input::PutDestinationInputBuilder;

impl PutDestinationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_destination::PutDestinationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_destination::PutDestinationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_destination();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutDestination`.
///
/// <p>Creates or updates a destination. This operation is used only to create destinations for cross-account subscriptions.</p>
/// <p>A destination encapsulates a physical resource (such as an Amazon Kinesis stream). With a destination, you can subscribe to a real-time stream of log events for a different account, ingested using <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutLogEvents.html">PutLogEvents</a>.</p>
/// <p>Through an access policy, a destination controls what is written to it. By default, <code>PutDestination</code> does not set any access policy with the destination, which means a cross-account user cannot call <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutSubscriptionFilter.html">PutSubscriptionFilter</a> against this destination. To enable this, the destination owner must call <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutDestinationPolicy.html">PutDestinationPolicy</a> after <code>PutDestination</code>.</p>
/// <p>To perform a <code>PutDestination</code> operation, you must also have the <code>iam:PassRole</code> permission.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutDestinationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_destination::builders::PutDestinationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_destination::PutDestinationOutput,
        crate::operation::put_destination::PutDestinationError,
    > for PutDestinationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_destination::PutDestinationOutput,
            crate::operation::put_destination::PutDestinationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutDestinationFluentBuilder {
    /// Creates a new `PutDestination`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutDestination as a reference.
    pub fn as_input(&self) -> &crate::operation::put_destination::builders::PutDestinationInputBuilder {
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
        crate::operation::put_destination::PutDestinationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_destination::PutDestinationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_destination::PutDestination::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_destination::PutDestination::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_destination::PutDestinationOutput,
        crate::operation::put_destination::PutDestinationError,
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
    /// <p>A name for the destination.</p>
    pub fn destination_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.destination_name(input.into());
        self
    }
    /// <p>A name for the destination.</p>
    pub fn set_destination_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_destination_name(input);
        self
    }
    /// <p>A name for the destination.</p>
    pub fn get_destination_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_destination_name()
    }
    /// <p>The ARN of an Amazon Kinesis stream to which to deliver matching log events.</p>
    pub fn target_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_arn(input.into());
        self
    }
    /// <p>The ARN of an Amazon Kinesis stream to which to deliver matching log events.</p>
    pub fn set_target_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_arn(input);
        self
    }
    /// <p>The ARN of an Amazon Kinesis stream to which to deliver matching log events.</p>
    pub fn get_target_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_arn()
    }
    /// <p>The ARN of an IAM role that grants CloudWatch Logs permissions to call the Amazon Kinesis <code>PutRecord</code> operation on the destination stream.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The ARN of an IAM role that grants CloudWatch Logs permissions to call the Amazon Kinesis <code>PutRecord</code> operation on the destination stream.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The ARN of an IAM role that grants CloudWatch Logs permissions to call the Amazon Kinesis <code>PutRecord</code> operation on the destination stream.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_arn()
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>An optional list of key-value pairs to associate with the resource.</p>
    /// <p>For more information about tagging, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> </p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>An optional list of key-value pairs to associate with the resource.</p>
    /// <p>For more information about tagging, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> </p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>An optional list of key-value pairs to associate with the resource.</p>
    /// <p>For more information about tagging, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> </p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}

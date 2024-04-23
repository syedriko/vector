// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_log_events::_get_log_events_output::GetLogEventsOutputBuilder;

pub use crate::operation::get_log_events::_get_log_events_input::GetLogEventsInputBuilder;

impl GetLogEventsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_log_events::GetLogEventsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_log_events::GetLogEventsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_log_events();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetLogEvents`.
///
/// <p>Lists log events from the specified log stream. You can list all of the log events or filter using a time range.</p>
/// <p>By default, this operation returns as many log events as can fit in a response size of 1MB (up to 10,000 log events). You can get additional log events by specifying one of the tokens in a subsequent call. This operation can return empty results while there are more log events available through the token.</p>
/// <p>If you are using CloudWatch cross-account observability, you can use this operation in a monitoring account and view data from the linked source accounts. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Unified-Cross-Account.html">CloudWatch cross-account observability</a>.</p>
/// <p>You can specify the log group to search by using either <code>logGroupIdentifier</code> or <code>logGroupName</code>. You must include one of these two parameters, but you can't include both. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetLogEventsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_log_events::builders::GetLogEventsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_log_events::GetLogEventsOutput,
        crate::operation::get_log_events::GetLogEventsError,
    > for GetLogEventsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_log_events::GetLogEventsOutput,
            crate::operation::get_log_events::GetLogEventsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetLogEventsFluentBuilder {
    /// Creates a new `GetLogEvents`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetLogEvents as a reference.
    pub fn as_input(&self) -> &crate::operation::get_log_events::builders::GetLogEventsInputBuilder {
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
        crate::operation::get_log_events::GetLogEventsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_log_events::GetLogEventsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_log_events::GetLogEvents::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_log_events::GetLogEvents::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_log_events::GetLogEventsOutput,
        crate::operation::get_log_events::GetLogEventsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::get_log_events::paginator::GetLogEventsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::get_log_events::paginator::GetLogEventsPaginator {
        crate::operation::get_log_events::paginator::GetLogEventsPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the log group.</p> <note>
    /// <p> You must include either <code>logGroupIdentifier</code> or <code>logGroupName</code>, but not both. </p>
    /// </note>
    pub fn log_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.log_group_name(input.into());
        self
    }
    /// <p>The name of the log group.</p> <note>
    /// <p> You must include either <code>logGroupIdentifier</code> or <code>logGroupName</code>, but not both. </p>
    /// </note>
    pub fn set_log_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_log_group_name(input);
        self
    }
    /// <p>The name of the log group.</p> <note>
    /// <p> You must include either <code>logGroupIdentifier</code> or <code>logGroupName</code>, but not both. </p>
    /// </note>
    pub fn get_log_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_log_group_name()
    }
    /// <p>Specify either the name or ARN of the log group to view events from. If the log group is in a source account and you are using a monitoring account, you must use the log group ARN.</p> <note>
    /// <p> You must include either <code>logGroupIdentifier</code> or <code>logGroupName</code>, but not both. </p>
    /// </note>
    pub fn log_group_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.log_group_identifier(input.into());
        self
    }
    /// <p>Specify either the name or ARN of the log group to view events from. If the log group is in a source account and you are using a monitoring account, you must use the log group ARN.</p> <note>
    /// <p> You must include either <code>logGroupIdentifier</code> or <code>logGroupName</code>, but not both. </p>
    /// </note>
    pub fn set_log_group_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_log_group_identifier(input);
        self
    }
    /// <p>Specify either the name or ARN of the log group to view events from. If the log group is in a source account and you are using a monitoring account, you must use the log group ARN.</p> <note>
    /// <p> You must include either <code>logGroupIdentifier</code> or <code>logGroupName</code>, but not both. </p>
    /// </note>
    pub fn get_log_group_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_log_group_identifier()
    }
    /// <p>The name of the log stream.</p>
    pub fn log_stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.log_stream_name(input.into());
        self
    }
    /// <p>The name of the log stream.</p>
    pub fn set_log_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_log_stream_name(input);
        self
    }
    /// <p>The name of the log stream.</p>
    pub fn get_log_stream_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_log_stream_name()
    }
    /// <p>The start of the time range, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp equal to this time or later than this time are included. Events with a timestamp earlier than this time are not included.</p>
    pub fn start_time(mut self, input: i64) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>The start of the time range, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp equal to this time or later than this time are included. Events with a timestamp earlier than this time are not included.</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>The start of the time range, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp equal to this time or later than this time are included. Events with a timestamp earlier than this time are not included.</p>
    pub fn get_start_time(&self) -> &::std::option::Option<i64> {
        self.inner.get_start_time()
    }
    /// <p>The end of the time range, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp equal to or later than this time are not included.</p>
    pub fn end_time(mut self, input: i64) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p>The end of the time range, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp equal to or later than this time are not included.</p>
    pub fn set_end_time(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p>The end of the time range, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp equal to or later than this time are not included.</p>
    pub fn get_end_time(&self) -> &::std::option::Option<i64> {
        self.inner.get_end_time()
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of log events returned. If you don't specify a limit, the default is as many log events as can fit in a response size of 1 MB (up to 10,000 log events).</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of log events returned. If you don't specify a limit, the default is as many log events as can fit in a response size of 1 MB (up to 10,000 log events).</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The maximum number of log events returned. If you don't specify a limit, the default is as many log events as can fit in a response size of 1 MB (up to 10,000 log events).</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        self.inner.get_limit()
    }
    /// <p>If the value is true, the earliest log events are returned first. If the value is false, the latest log events are returned first. The default value is false.</p>
    /// <p>If you are using a previous <code>nextForwardToken</code> value as the <code>nextToken</code> in this operation, you must specify <code>true</code> for <code>startFromHead</code>.</p>
    pub fn start_from_head(mut self, input: bool) -> Self {
        self.inner = self.inner.start_from_head(input);
        self
    }
    /// <p>If the value is true, the earliest log events are returned first. If the value is false, the latest log events are returned first. The default value is false.</p>
    /// <p>If you are using a previous <code>nextForwardToken</code> value as the <code>nextToken</code> in this operation, you must specify <code>true</code> for <code>startFromHead</code>.</p>
    pub fn set_start_from_head(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_start_from_head(input);
        self
    }
    /// <p>If the value is true, the earliest log events are returned first. If the value is false, the latest log events are returned first. The default value is false.</p>
    /// <p>If you are using a previous <code>nextForwardToken</code> value as the <code>nextToken</code> in this operation, you must specify <code>true</code> for <code>startFromHead</code>.</p>
    pub fn get_start_from_head(&self) -> &::std::option::Option<bool> {
        self.inner.get_start_from_head()
    }
    /// <p>Specify <code>true</code> to display the log event fields with all sensitive data unmasked and visible. The default is <code>false</code>.</p>
    /// <p>To use this operation with this parameter, you must be signed into an account with the <code>logs:Unmask</code> permission.</p>
    pub fn unmask(mut self, input: bool) -> Self {
        self.inner = self.inner.unmask(input);
        self
    }
    /// <p>Specify <code>true</code> to display the log event fields with all sensitive data unmasked and visible. The default is <code>false</code>.</p>
    /// <p>To use this operation with this parameter, you must be signed into an account with the <code>logs:Unmask</code> permission.</p>
    pub fn set_unmask(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_unmask(input);
        self
    }
    /// <p>Specify <code>true</code> to display the log event fields with all sensitive data unmasked and visible. The default is <code>false</code>.</p>
    /// <p>To use this operation with this parameter, you must be signed into an account with the <code>logs:Unmask</code> permission.</p>
    pub fn get_unmask(&self) -> &::std::option::Option<bool> {
        self.inner.get_unmask()
    }
}

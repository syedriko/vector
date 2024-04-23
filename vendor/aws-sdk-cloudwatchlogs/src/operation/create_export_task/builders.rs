// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_export_task::_create_export_task_output::CreateExportTaskOutputBuilder;

pub use crate::operation::create_export_task::_create_export_task_input::CreateExportTaskInputBuilder;

impl CreateExportTaskInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_export_task::CreateExportTaskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_export_task::CreateExportTaskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_export_task();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateExportTask`.
///
/// <p>Creates an export task so that you can efficiently export data from a log group to an Amazon S3 bucket. When you perform a <code>CreateExportTask</code> operation, you must use credentials that have permission to write to the S3 bucket that you specify as the destination.</p>
/// <p>Exporting log data to S3 buckets that are encrypted by KMS is supported. Exporting log data to Amazon S3 buckets that have S3 Object Lock enabled with a retention period is also supported.</p>
/// <p>Exporting to S3 buckets that are encrypted with AES-256 is supported. </p>
/// <p>This is an asynchronous call. If all the required information is provided, this operation initiates an export task and responds with the ID of the task. After the task has started, you can use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeExportTasks.html">DescribeExportTasks</a> to get the status of the export task. Each account can only have one active (<code>RUNNING</code> or <code>PENDING</code>) export task at a time. To cancel an export task, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_CancelExportTask.html">CancelExportTask</a>.</p>
/// <p>You can export logs from multiple log groups or multiple time ranges to the same S3 bucket. To separate log data for each export task, specify a prefix to be used as the Amazon S3 key prefix for all exported objects.</p> <note>
/// <p>Time-based sorting on chunks of log data inside an exported file is not guaranteed. You can sort the exported log field data by using Linux utilities.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateExportTaskFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_export_task::builders::CreateExportTaskInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_export_task::CreateExportTaskOutput,
        crate::operation::create_export_task::CreateExportTaskError,
    > for CreateExportTaskFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_export_task::CreateExportTaskOutput,
            crate::operation::create_export_task::CreateExportTaskError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateExportTaskFluentBuilder {
    /// Creates a new `CreateExportTask`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateExportTask as a reference.
    pub fn as_input(&self) -> &crate::operation::create_export_task::builders::CreateExportTaskInputBuilder {
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
        crate::operation::create_export_task::CreateExportTaskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_export_task::CreateExportTaskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_export_task::CreateExportTask::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_export_task::CreateExportTask::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_export_task::CreateExportTaskOutput,
        crate::operation::create_export_task::CreateExportTaskError,
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
    /// <p>The name of the export task.</p>
    pub fn task_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.task_name(input.into());
        self
    }
    /// <p>The name of the export task.</p>
    pub fn set_task_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_task_name(input);
        self
    }
    /// <p>The name of the export task.</p>
    pub fn get_task_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_task_name()
    }
    /// <p>The name of the log group.</p>
    pub fn log_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.log_group_name(input.into());
        self
    }
    /// <p>The name of the log group.</p>
    pub fn set_log_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_log_group_name(input);
        self
    }
    /// <p>The name of the log group.</p>
    pub fn get_log_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_log_group_name()
    }
    /// <p>Export only log streams that match the provided prefix. If you don't specify a value, no prefix filter is applied.</p>
    pub fn log_stream_name_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.log_stream_name_prefix(input.into());
        self
    }
    /// <p>Export only log streams that match the provided prefix. If you don't specify a value, no prefix filter is applied.</p>
    pub fn set_log_stream_name_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_log_stream_name_prefix(input);
        self
    }
    /// <p>Export only log streams that match the provided prefix. If you don't specify a value, no prefix filter is applied.</p>
    pub fn get_log_stream_name_prefix(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_log_stream_name_prefix()
    }
    /// <p>The start time of the range for the request, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp earlier than this time are not exported.</p>
    pub fn from(mut self, input: i64) -> Self {
        self.inner = self.inner.from(input);
        self
    }
    /// <p>The start time of the range for the request, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp earlier than this time are not exported.</p>
    pub fn set_from(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_from(input);
        self
    }
    /// <p>The start time of the range for the request, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp earlier than this time are not exported.</p>
    pub fn get_from(&self) -> &::std::option::Option<i64> {
        self.inner.get_from()
    }
    /// <p>The end time of the range for the request, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp later than this time are not exported.</p>
    /// <p>You must specify a time that is not earlier than when this log group was created.</p>
    pub fn to(mut self, input: i64) -> Self {
        self.inner = self.inner.to(input);
        self
    }
    /// <p>The end time of the range for the request, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp later than this time are not exported.</p>
    /// <p>You must specify a time that is not earlier than when this log group was created.</p>
    pub fn set_to(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_to(input);
        self
    }
    /// <p>The end time of the range for the request, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp later than this time are not exported.</p>
    /// <p>You must specify a time that is not earlier than when this log group was created.</p>
    pub fn get_to(&self) -> &::std::option::Option<i64> {
        self.inner.get_to()
    }
    /// <p>The name of S3 bucket for the exported log data. The bucket must be in the same Amazon Web Services Region.</p>
    pub fn destination(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.destination(input.into());
        self
    }
    /// <p>The name of S3 bucket for the exported log data. The bucket must be in the same Amazon Web Services Region.</p>
    pub fn set_destination(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_destination(input);
        self
    }
    /// <p>The name of S3 bucket for the exported log data. The bucket must be in the same Amazon Web Services Region.</p>
    pub fn get_destination(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_destination()
    }
    /// <p>The prefix used as the start of the key for every object exported. If you don't specify a value, the default is <code>exportedlogs</code>.</p>
    pub fn destination_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.destination_prefix(input.into());
        self
    }
    /// <p>The prefix used as the start of the key for every object exported. If you don't specify a value, the default is <code>exportedlogs</code>.</p>
    pub fn set_destination_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_destination_prefix(input);
        self
    }
    /// <p>The prefix used as the start of the key for every object exported. If you don't specify a value, the default is <code>exportedlogs</code>.</p>
    pub fn get_destination_prefix(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_destination_prefix()
    }
}

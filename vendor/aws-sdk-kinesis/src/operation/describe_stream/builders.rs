// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_stream::_describe_stream_output::DescribeStreamOutputBuilder;

pub use crate::operation::describe_stream::_describe_stream_input::DescribeStreamInputBuilder;

impl DescribeStreamInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_stream::DescribeStreamOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_stream::DescribeStreamError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_stream();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeStream`.
///
/// <p>Describes the specified Kinesis data stream.</p> <note>
/// <p>This API has been revised. It's highly recommended that you use the <code>DescribeStreamSummary</code> API to get a summarized description of the specified Kinesis data stream and the <code>ListShards</code> API to list the shards in a specified data stream and obtain information about each shard. </p>
/// </note> <note>
/// <p>When invoking this API, you must use either the <code>StreamARN</code> or the <code>StreamName</code> parameter, or both. It is recommended that you use the <code>StreamARN</code> input parameter when you invoke this API.</p>
/// </note>
/// <p>The information returned includes the stream name, Amazon Resource Name (ARN), creation time, enhanced metric configuration, and shard map. The shard map is an array of shard objects. For each shard object, there is the hash key and sequence number ranges that the shard spans, and the IDs of any earlier shards that played in a role in creating the shard. Every record ingested in the stream is identified by a sequence number, which is assigned when the record is put into the stream.</p>
/// <p>You can limit the number of shards returned by each call. For more information, see <a href="https://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-retrieve-shards.html">Retrieving Shards from a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p>
/// <p>There are no guarantees about the chronological order shards returned. To process shards in chronological order, use the ID of the parent shard to track the lineage to the oldest shard.</p>
/// <p>This operation has a limit of 10 transactions per second per account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeStreamFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_stream::builders::DescribeStreamInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_stream::DescribeStreamOutput,
        crate::operation::describe_stream::DescribeStreamError,
    > for DescribeStreamFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_stream::DescribeStreamOutput,
            crate::operation::describe_stream::DescribeStreamError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeStreamFluentBuilder {
    /// Creates a new `DescribeStream`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeStream as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_stream::builders::DescribeStreamInputBuilder {
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
        crate::operation::describe_stream::DescribeStreamOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_stream::DescribeStreamError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_stream::DescribeStream::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_stream::DescribeStream::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_stream::DescribeStreamOutput,
        crate::operation::describe_stream::DescribeStreamError,
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
    /// <p>The name of the stream to describe.</p>
    pub fn stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stream_name(input.into());
        self
    }
    /// <p>The name of the stream to describe.</p>
    pub fn set_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stream_name(input);
        self
    }
    /// <p>The name of the stream to describe.</p>
    pub fn get_stream_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stream_name()
    }
    /// <p>The maximum number of shards to return in a single call. The default value is 100. If you specify a value greater than 100, at most 100 results are returned.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of shards to return in a single call. The default value is 100. If you specify a value greater than 100, at most 100 results are returned.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The maximum number of shards to return in a single call. The default value is 100. If you specify a value greater than 100, at most 100 results are returned.</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        self.inner.get_limit()
    }
    /// <p>The shard ID of the shard to start with.</p>
    /// <p>Specify this parameter to indicate that you want to describe the stream starting with the shard whose ID immediately follows <code>ExclusiveStartShardId</code>.</p>
    /// <p>If you don't specify this parameter, the default behavior for <code>DescribeStream</code> is to describe the stream starting with the first shard in the stream.</p>
    pub fn exclusive_start_shard_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.exclusive_start_shard_id(input.into());
        self
    }
    /// <p>The shard ID of the shard to start with.</p>
    /// <p>Specify this parameter to indicate that you want to describe the stream starting with the shard whose ID immediately follows <code>ExclusiveStartShardId</code>.</p>
    /// <p>If you don't specify this parameter, the default behavior for <code>DescribeStream</code> is to describe the stream starting with the first shard in the stream.</p>
    pub fn set_exclusive_start_shard_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_exclusive_start_shard_id(input);
        self
    }
    /// <p>The shard ID of the shard to start with.</p>
    /// <p>Specify this parameter to indicate that you want to describe the stream starting with the shard whose ID immediately follows <code>ExclusiveStartShardId</code>.</p>
    /// <p>If you don't specify this parameter, the default behavior for <code>DescribeStream</code> is to describe the stream starting with the first shard in the stream.</p>
    pub fn get_exclusive_start_shard_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_exclusive_start_shard_id()
    }
    /// <p>The ARN of the stream.</p>
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stream_arn(input.into());
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stream_arn(input);
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn get_stream_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stream_arn()
    }
}

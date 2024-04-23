// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_stream_encryption::_start_stream_encryption_output::StartStreamEncryptionOutputBuilder;

pub use crate::operation::start_stream_encryption::_start_stream_encryption_input::StartStreamEncryptionInputBuilder;

impl StartStreamEncryptionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_stream_encryption::StartStreamEncryptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_stream_encryption::StartStreamEncryptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_stream_encryption();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartStreamEncryption`.
///
/// <p>Enables or updates server-side encryption using an Amazon Web Services KMS key for a specified stream. </p> <note>
/// <p>When invoking this API, you must use either the <code>StreamARN</code> or the <code>StreamName</code> parameter, or both. It is recommended that you use the <code>StreamARN</code> input parameter when you invoke this API.</p>
/// </note>
/// <p>Starting encryption is an asynchronous operation. Upon receiving the request, Kinesis Data Streams returns immediately and sets the status of the stream to <code>UPDATING</code>. After the update is complete, Kinesis Data Streams sets the status of the stream back to <code>ACTIVE</code>. Updating or applying encryption normally takes a few seconds to complete, but it can take minutes. You can continue to read and write data to your stream while its status is <code>UPDATING</code>. Once the status of the stream is <code>ACTIVE</code>, encryption begins for records written to the stream. </p>
/// <p>API Limits: You can successfully apply a new Amazon Web Services KMS key for server-side encryption 25 times in a rolling 24-hour period.</p>
/// <p>Note: It can take up to 5 seconds after the stream is in an <code>ACTIVE</code> status before all records written to the stream are encrypted. After you enable encryption, you can verify that encryption is applied by inspecting the API response from <code>PutRecord</code> or <code>PutRecords</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartStreamEncryptionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_stream_encryption::builders::StartStreamEncryptionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_stream_encryption::StartStreamEncryptionOutput,
        crate::operation::start_stream_encryption::StartStreamEncryptionError,
    > for StartStreamEncryptionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_stream_encryption::StartStreamEncryptionOutput,
            crate::operation::start_stream_encryption::StartStreamEncryptionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartStreamEncryptionFluentBuilder {
    /// Creates a new `StartStreamEncryption`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartStreamEncryption as a reference.
    pub fn as_input(&self) -> &crate::operation::start_stream_encryption::builders::StartStreamEncryptionInputBuilder {
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
        crate::operation::start_stream_encryption::StartStreamEncryptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_stream_encryption::StartStreamEncryptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_stream_encryption::StartStreamEncryption::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_stream_encryption::StartStreamEncryption::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_stream_encryption::StartStreamEncryptionOutput,
        crate::operation::start_stream_encryption::StartStreamEncryptionError,
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
    /// <p>The name of the stream for which to start encrypting records.</p>
    pub fn stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stream_name(input.into());
        self
    }
    /// <p>The name of the stream for which to start encrypting records.</p>
    pub fn set_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stream_name(input);
        self
    }
    /// <p>The name of the stream for which to start encrypting records.</p>
    pub fn get_stream_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stream_name()
    }
    /// <p>The encryption type to use. The only valid value is <code>KMS</code>.</p>
    pub fn encryption_type(mut self, input: crate::types::EncryptionType) -> Self {
        self.inner = self.inner.encryption_type(input);
        self
    }
    /// <p>The encryption type to use. The only valid value is <code>KMS</code>.</p>
    pub fn set_encryption_type(mut self, input: ::std::option::Option<crate::types::EncryptionType>) -> Self {
        self.inner = self.inner.set_encryption_type(input);
        self
    }
    /// <p>The encryption type to use. The only valid value is <code>KMS</code>.</p>
    pub fn get_encryption_type(&self) -> &::std::option::Option<crate::types::EncryptionType> {
        self.inner.get_encryption_type()
    }
    /// <p>The GUID for the customer-managed Amazon Web Services KMS key to use for encryption. This value can be a globally unique identifier, a fully specified Amazon Resource Name (ARN) to either an alias or a key, or an alias name prefixed by "alias/".You can also use a master key owned by Kinesis Data Streams by specifying the alias <code>aws/kinesis</code>.</p>
    /// <ul>
    /// <li> <p>Key ARN example: <code>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p>Alias ARN example: <code>arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</code> </p> </li>
    /// <li> <p>Globally unique key ID example: <code>12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p>Alias name example: <code>alias/MyAliasName</code> </p> </li>
    /// <li> <p>Master key owned by Kinesis Data Streams: <code>alias/aws/kinesis</code> </p> </li>
    /// </ul>
    pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_id(input.into());
        self
    }
    /// <p>The GUID for the customer-managed Amazon Web Services KMS key to use for encryption. This value can be a globally unique identifier, a fully specified Amazon Resource Name (ARN) to either an alias or a key, or an alias name prefixed by "alias/".You can also use a master key owned by Kinesis Data Streams by specifying the alias <code>aws/kinesis</code>.</p>
    /// <ul>
    /// <li> <p>Key ARN example: <code>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p>Alias ARN example: <code>arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</code> </p> </li>
    /// <li> <p>Globally unique key ID example: <code>12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p>Alias name example: <code>alias/MyAliasName</code> </p> </li>
    /// <li> <p>Master key owned by Kinesis Data Streams: <code>alias/aws/kinesis</code> </p> </li>
    /// </ul>
    pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_id(input);
        self
    }
    /// <p>The GUID for the customer-managed Amazon Web Services KMS key to use for encryption. This value can be a globally unique identifier, a fully specified Amazon Resource Name (ARN) to either an alias or a key, or an alias name prefixed by "alias/".You can also use a master key owned by Kinesis Data Streams by specifying the alias <code>aws/kinesis</code>.</p>
    /// <ul>
    /// <li> <p>Key ARN example: <code>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p>Alias ARN example: <code>arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</code> </p> </li>
    /// <li> <p>Globally unique key ID example: <code>12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p>Alias name example: <code>alias/MyAliasName</code> </p> </li>
    /// <li> <p>Master key owned by Kinesis Data Streams: <code>alias/aws/kinesis</code> </p> </li>
    /// </ul>
    pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_id()
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

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_bucket_policy_status::_get_bucket_policy_status_output::GetBucketPolicyStatusOutputBuilder;

pub use crate::operation::get_bucket_policy_status::_get_bucket_policy_status_input::GetBucketPolicyStatusInputBuilder;

impl GetBucketPolicyStatusInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_bucket_policy_status::GetBucketPolicyStatusOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_bucket_policy_status::GetBucketPolicyStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_bucket_policy_status();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetBucketPolicyStatus`.
///
/// <note>
/// <p>This operation is not supported by directory buckets.</p>
/// </note>
/// <p>Retrieves the policy status for an Amazon S3 bucket, indicating whether the bucket is public. In order to use this operation, you must have the <code>s3:GetBucketPolicyStatus</code> permission. For more information about Amazon S3 permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-with-s3-actions.html">Specifying Permissions in a Policy</a>.</p>
/// <p> For more information about when Amazon S3 considers a bucket public, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html#access-control-block-public-access-policy-status">The Meaning of "Public"</a>. </p>
/// <p>The following operations are related to <code>GetBucketPolicyStatus</code>:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html">Using Amazon S3 Block Public Access</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetPublicAccessBlock.html">GetPublicAccessBlock</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutPublicAccessBlock.html">PutPublicAccessBlock</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeletePublicAccessBlock.html">DeletePublicAccessBlock</a> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetBucketPolicyStatusFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_bucket_policy_status::builders::GetBucketPolicyStatusInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_bucket_policy_status::GetBucketPolicyStatusOutput,
        crate::operation::get_bucket_policy_status::GetBucketPolicyStatusError,
    > for GetBucketPolicyStatusFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_bucket_policy_status::GetBucketPolicyStatusOutput,
            crate::operation::get_bucket_policy_status::GetBucketPolicyStatusError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetBucketPolicyStatusFluentBuilder {
    /// Creates a new `GetBucketPolicyStatus`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetBucketPolicyStatus as a reference.
    pub fn as_input(&self) -> &crate::operation::get_bucket_policy_status::builders::GetBucketPolicyStatusInputBuilder {
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
        crate::operation::get_bucket_policy_status::GetBucketPolicyStatusOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_bucket_policy_status::GetBucketPolicyStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_bucket_policy_status::GetBucketPolicyStatus::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_bucket_policy_status::GetBucketPolicyStatus::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_bucket_policy_status::GetBucketPolicyStatusOutput,
        crate::operation::get_bucket_policy_status::GetBucketPolicyStatusError,
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
    /// <p>The name of the Amazon S3 bucket whose policy status you want to retrieve.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The name of the Amazon S3 bucket whose policy status you want to retrieve.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>The name of the Amazon S3 bucket whose policy status you want to retrieve.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bucket()
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.expected_bucket_owner(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_expected_bucket_owner(input);
        self
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn get_expected_bucket_owner(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_expected_bucket_owner()
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_resource_policy::_delete_resource_policy_output::DeleteResourcePolicyOutputBuilder;

pub use crate::operation::delete_resource_policy::_delete_resource_policy_input::DeleteResourcePolicyInputBuilder;

impl DeleteResourcePolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_resource_policy::DeleteResourcePolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_resource_policy::DeleteResourcePolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_resource_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteResourcePolicy`.
///
/// <p>Deletes a resource policy from this account. This revokes the access of the identities in that policy to put log events to this account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteResourcePolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_resource_policy::builders::DeleteResourcePolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_resource_policy::DeleteResourcePolicyOutput,
        crate::operation::delete_resource_policy::DeleteResourcePolicyError,
    > for DeleteResourcePolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_resource_policy::DeleteResourcePolicyOutput,
            crate::operation::delete_resource_policy::DeleteResourcePolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteResourcePolicyFluentBuilder {
    /// Creates a new `DeleteResourcePolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteResourcePolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_resource_policy::builders::DeleteResourcePolicyInputBuilder {
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
        crate::operation::delete_resource_policy::DeleteResourcePolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_resource_policy::DeleteResourcePolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_resource_policy::DeleteResourcePolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_resource_policy::DeleteResourcePolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_resource_policy::DeleteResourcePolicyOutput,
        crate::operation::delete_resource_policy::DeleteResourcePolicyError,
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
    /// <p>The name of the policy to be revoked. This parameter is required.</p>
    pub fn policy_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_name(input.into());
        self
    }
    /// <p>The name of the policy to be revoked. This parameter is required.</p>
    pub fn set_policy_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_name(input);
        self
    }
    /// <p>The name of the policy to be revoked. This parameter is required.</p>
    pub fn get_policy_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_name()
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::add_permission::_add_permission_output::AddPermissionOutputBuilder;

pub use crate::operation::add_permission::_add_permission_input::AddPermissionInputBuilder;

impl AddPermissionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::add_permission::AddPermissionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::add_permission::AddPermissionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.add_permission();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AddPermission`.
///
/// <p>Adds a statement to a topic's access control policy, granting access for the specified Amazon Web Services accounts to the specified actions.</p> <note>
/// <p>To remove the ability to change topic permissions, you must deny permissions to the <code>AddPermission</code>, <code>RemovePermission</code>, and <code>SetTopicAttributes</code> actions in your IAM policy.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AddPermissionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::add_permission::builders::AddPermissionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::add_permission::AddPermissionOutput,
        crate::operation::add_permission::AddPermissionError,
    > for AddPermissionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::add_permission::AddPermissionOutput,
            crate::operation::add_permission::AddPermissionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AddPermissionFluentBuilder {
    /// Creates a new `AddPermission`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AddPermission as a reference.
    pub fn as_input(&self) -> &crate::operation::add_permission::builders::AddPermissionInputBuilder {
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
        crate::operation::add_permission::AddPermissionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::add_permission::AddPermissionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::add_permission::AddPermission::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::add_permission::AddPermission::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::add_permission::AddPermissionOutput,
        crate::operation::add_permission::AddPermissionError,
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
    /// <p>The ARN of the topic whose access control policy you wish to modify.</p>
    pub fn topic_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.topic_arn(input.into());
        self
    }
    /// <p>The ARN of the topic whose access control policy you wish to modify.</p>
    pub fn set_topic_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_topic_arn(input);
        self
    }
    /// <p>The ARN of the topic whose access control policy you wish to modify.</p>
    pub fn get_topic_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_topic_arn()
    }
    /// <p>A unique identifier for the new policy statement.</p>
    pub fn label(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.label(input.into());
        self
    }
    /// <p>A unique identifier for the new policy statement.</p>
    pub fn set_label(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_label(input);
        self
    }
    /// <p>A unique identifier for the new policy statement.</p>
    pub fn get_label(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_label()
    }
    /// Appends an item to `AWSAccountId`.
    ///
    /// To override the contents of this collection use [`set_aws_account_id`](Self::set_aws_account_id).
    ///
    /// <p>The Amazon Web Services account IDs of the users (principals) who will be given access to the specified actions. The users must have Amazon Web Services account, but do not need to be signed up for this service.</p>
    pub fn aws_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account IDs of the users (principals) who will be given access to the specified actions. The users must have Amazon Web Services account, but do not need to be signed up for this service.</p>
    pub fn set_aws_account_id(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The Amazon Web Services account IDs of the users (principals) who will be given access to the specified actions. The users must have Amazon Web Services account, but do not need to be signed up for this service.</p>
    pub fn get_aws_account_id(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_aws_account_id()
    }
    /// Appends an item to `ActionName`.
    ///
    /// To override the contents of this collection use [`set_action_name`](Self::set_action_name).
    ///
    /// <p>The action you want to allow for the specified principal(s).</p>
    /// <p>Valid values: Any Amazon SNS action name, for example <code>Publish</code>.</p>
    pub fn action_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.action_name(input.into());
        self
    }
    /// <p>The action you want to allow for the specified principal(s).</p>
    /// <p>Valid values: Any Amazon SNS action name, for example <code>Publish</code>.</p>
    pub fn set_action_name(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_action_name(input);
        self
    }
    /// <p>The action you want to allow for the specified principal(s).</p>
    /// <p>Valid values: Any Amazon SNS action name, for example <code>Publish</code>.</p>
    pub fn get_action_name(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_action_name()
    }
}

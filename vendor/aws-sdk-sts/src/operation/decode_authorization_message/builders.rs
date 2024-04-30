// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::decode_authorization_message::_decode_authorization_message_output::DecodeAuthorizationMessageOutputBuilder;

pub use crate::operation::decode_authorization_message::_decode_authorization_message_input::DecodeAuthorizationMessageInputBuilder;

impl DecodeAuthorizationMessageInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.decode_authorization_message();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DecodeAuthorizationMessage`.
///
/// <p>Decodes additional information about the authorization status of a request from an encoded message returned in response to an Amazon Web Services request.</p>
/// <p>For example, if a user is not authorized to perform an operation that he or she has requested, the request returns a <code>Client.UnauthorizedOperation</code> response (an HTTP 403 response). Some Amazon Web Services operations additionally return an encoded message that can provide details about this authorization failure. </p> <note>
/// <p>Only certain Amazon Web Services operations return an encoded authorization message. The documentation for an individual operation indicates whether that operation returns an encoded message in addition to returning an HTTP code.</p>
/// </note>
/// <p>The message is encoded because the details of the authorization status can contain privileged information that the user who requested the operation should not see. To decode an authorization status message, a user must be granted permissions through an IAM <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html">policy</a> to request the <code>DecodeAuthorizationMessage</code> (<code>sts:DecodeAuthorizationMessage</code>) action. </p>
/// <p>The decoded message includes the following type of information:</p>
/// <ul>
/// <li> <p>Whether the request was denied due to an explicit deny or due to the absence of an explicit allow. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_evaluation-logic.html#policy-eval-denyallow">Determining Whether a Request is Allowed or Denied</a> in the <i>IAM User Guide</i>. </p> </li>
/// <li> <p>The principal who made the request.</p> </li>
/// <li> <p>The requested action.</p> </li>
/// <li> <p>The requested resource.</p> </li>
/// <li> <p>The values of condition keys in the context of the user's request.</p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DecodeAuthorizationMessageFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput,
        crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
    > for DecodeAuthorizationMessageFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput,
            crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DecodeAuthorizationMessageFluentBuilder {
    /// Creates a new `DecodeAuthorizationMessage`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DecodeAuthorizationMessage as a reference.
    pub fn as_input(&self) -> &crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageInputBuilder {
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
        crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::decode_authorization_message::DecodeAuthorizationMessage::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::decode_authorization_message::DecodeAuthorizationMessage::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput,
        crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
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
    /// <p>The encoded message that was returned with the response.</p>
    pub fn encoded_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.encoded_message(input.into());
        self
    }
    /// <p>The encoded message that was returned with the response.</p>
    pub fn set_encoded_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_encoded_message(input);
        self
    }
    /// <p>The encoded message that was returned with the response.</p>
    pub fn get_encoded_message(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_encoded_message()
    }
}

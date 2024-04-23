// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the connection status of an outbound cross-cluster search connection.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OutboundCrossClusterSearchConnectionStatus {
    /// <p>The state code for outbound connection. This can be one of the following:</p>
    /// <ul>
    /// <li>VALIDATING: The outbound connection request is being validated.</li>
    /// <li>VALIDATION_FAILED: Validation failed for the connection request.</li>
    /// <li>PENDING_ACCEPTANCE: Outbound connection request is validated and is not yet accepted by destination domain owner.</li>
    /// <li>PROVISIONING: Outbound connection request is in process.</li>
    /// <li>ACTIVE: Outbound connection is active and ready to use.</li>
    /// <li>REJECTED: Outbound connection request is rejected by destination domain owner.</li>
    /// <li>DELETING: Outbound connection deletion is in progress.</li>
    /// <li>DELETED: Outbound connection is deleted and cannot be used further.</li>
    /// </ul>
    pub status_code: ::std::option::Option<crate::types::OutboundCrossClusterSearchConnectionStatusCode>,
    /// <p>Specifies verbose information for the outbound connection status.</p>
    pub message: ::std::option::Option<::std::string::String>,
}
impl OutboundCrossClusterSearchConnectionStatus {
    /// <p>The state code for outbound connection. This can be one of the following:</p>
    /// <ul>
    /// <li>VALIDATING: The outbound connection request is being validated.</li>
    /// <li>VALIDATION_FAILED: Validation failed for the connection request.</li>
    /// <li>PENDING_ACCEPTANCE: Outbound connection request is validated and is not yet accepted by destination domain owner.</li>
    /// <li>PROVISIONING: Outbound connection request is in process.</li>
    /// <li>ACTIVE: Outbound connection is active and ready to use.</li>
    /// <li>REJECTED: Outbound connection request is rejected by destination domain owner.</li>
    /// <li>DELETING: Outbound connection deletion is in progress.</li>
    /// <li>DELETED: Outbound connection is deleted and cannot be used further.</li>
    /// </ul>
    pub fn status_code(&self) -> ::std::option::Option<&crate::types::OutboundCrossClusterSearchConnectionStatusCode> {
        self.status_code.as_ref()
    }
    /// <p>Specifies verbose information for the outbound connection status.</p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl OutboundCrossClusterSearchConnectionStatus {
    /// Creates a new builder-style object to manufacture [`OutboundCrossClusterSearchConnectionStatus`](crate::types::OutboundCrossClusterSearchConnectionStatus).
    pub fn builder() -> crate::types::builders::OutboundCrossClusterSearchConnectionStatusBuilder {
        crate::types::builders::OutboundCrossClusterSearchConnectionStatusBuilder::default()
    }
}

/// A builder for [`OutboundCrossClusterSearchConnectionStatus`](crate::types::OutboundCrossClusterSearchConnectionStatus).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct OutboundCrossClusterSearchConnectionStatusBuilder {
    pub(crate) status_code: ::std::option::Option<crate::types::OutboundCrossClusterSearchConnectionStatusCode>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
}
impl OutboundCrossClusterSearchConnectionStatusBuilder {
    /// <p>The state code for outbound connection. This can be one of the following:</p>
    /// <ul>
    /// <li>VALIDATING: The outbound connection request is being validated.</li>
    /// <li>VALIDATION_FAILED: Validation failed for the connection request.</li>
    /// <li>PENDING_ACCEPTANCE: Outbound connection request is validated and is not yet accepted by destination domain owner.</li>
    /// <li>PROVISIONING: Outbound connection request is in process.</li>
    /// <li>ACTIVE: Outbound connection is active and ready to use.</li>
    /// <li>REJECTED: Outbound connection request is rejected by destination domain owner.</li>
    /// <li>DELETING: Outbound connection deletion is in progress.</li>
    /// <li>DELETED: Outbound connection is deleted and cannot be used further.</li>
    /// </ul>
    pub fn status_code(mut self, input: crate::types::OutboundCrossClusterSearchConnectionStatusCode) -> Self {
        self.status_code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state code for outbound connection. This can be one of the following:</p>
    /// <ul>
    /// <li>VALIDATING: The outbound connection request is being validated.</li>
    /// <li>VALIDATION_FAILED: Validation failed for the connection request.</li>
    /// <li>PENDING_ACCEPTANCE: Outbound connection request is validated and is not yet accepted by destination domain owner.</li>
    /// <li>PROVISIONING: Outbound connection request is in process.</li>
    /// <li>ACTIVE: Outbound connection is active and ready to use.</li>
    /// <li>REJECTED: Outbound connection request is rejected by destination domain owner.</li>
    /// <li>DELETING: Outbound connection deletion is in progress.</li>
    /// <li>DELETED: Outbound connection is deleted and cannot be used further.</li>
    /// </ul>
    pub fn set_status_code(mut self, input: ::std::option::Option<crate::types::OutboundCrossClusterSearchConnectionStatusCode>) -> Self {
        self.status_code = input;
        self
    }
    /// <p>The state code for outbound connection. This can be one of the following:</p>
    /// <ul>
    /// <li>VALIDATING: The outbound connection request is being validated.</li>
    /// <li>VALIDATION_FAILED: Validation failed for the connection request.</li>
    /// <li>PENDING_ACCEPTANCE: Outbound connection request is validated and is not yet accepted by destination domain owner.</li>
    /// <li>PROVISIONING: Outbound connection request is in process.</li>
    /// <li>ACTIVE: Outbound connection is active and ready to use.</li>
    /// <li>REJECTED: Outbound connection request is rejected by destination domain owner.</li>
    /// <li>DELETING: Outbound connection deletion is in progress.</li>
    /// <li>DELETED: Outbound connection is deleted and cannot be used further.</li>
    /// </ul>
    pub fn get_status_code(&self) -> &::std::option::Option<crate::types::OutboundCrossClusterSearchConnectionStatusCode> {
        &self.status_code
    }
    /// <p>Specifies verbose information for the outbound connection status.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies verbose information for the outbound connection status.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>Specifies verbose information for the outbound connection status.</p>
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// Consumes the builder and constructs a [`OutboundCrossClusterSearchConnectionStatus`](crate::types::OutboundCrossClusterSearchConnectionStatus).
    pub fn build(self) -> crate::types::OutboundCrossClusterSearchConnectionStatus {
        crate::types::OutboundCrossClusterSearchConnectionStatus {
            status_code: self.status_code,
            message: self.message,
        }
    }
}

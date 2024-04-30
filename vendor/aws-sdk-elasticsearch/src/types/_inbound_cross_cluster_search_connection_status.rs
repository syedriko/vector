// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the coonection status of an inbound cross-cluster search connection.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InboundCrossClusterSearchConnectionStatus {
    /// <p>The state code for inbound connection. This can be one of the following:</p>
    /// <ul>
    /// <li>PENDING_ACCEPTANCE: Inbound connection is not yet accepted by destination domain owner.</li>
    /// <li>APPROVED: Inbound connection is pending acceptance by destination domain owner.</li>
    /// <li>REJECTING: Inbound connection rejection is in process.</li>
    /// <li>REJECTED: Inbound connection is rejected.</li>
    /// <li>DELETING: Inbound connection deletion is in progress.</li>
    /// <li>DELETED: Inbound connection is deleted and cannot be used further.</li>
    /// </ul>
    pub status_code: ::std::option::Option<crate::types::InboundCrossClusterSearchConnectionStatusCode>,
    /// <p>Specifies verbose information for the inbound connection status.</p>
    pub message: ::std::option::Option<::std::string::String>,
}
impl InboundCrossClusterSearchConnectionStatus {
    /// <p>The state code for inbound connection. This can be one of the following:</p>
    /// <ul>
    /// <li>PENDING_ACCEPTANCE: Inbound connection is not yet accepted by destination domain owner.</li>
    /// <li>APPROVED: Inbound connection is pending acceptance by destination domain owner.</li>
    /// <li>REJECTING: Inbound connection rejection is in process.</li>
    /// <li>REJECTED: Inbound connection is rejected.</li>
    /// <li>DELETING: Inbound connection deletion is in progress.</li>
    /// <li>DELETED: Inbound connection is deleted and cannot be used further.</li>
    /// </ul>
    pub fn status_code(&self) -> ::std::option::Option<&crate::types::InboundCrossClusterSearchConnectionStatusCode> {
        self.status_code.as_ref()
    }
    /// <p>Specifies verbose information for the inbound connection status.</p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl InboundCrossClusterSearchConnectionStatus {
    /// Creates a new builder-style object to manufacture [`InboundCrossClusterSearchConnectionStatus`](crate::types::InboundCrossClusterSearchConnectionStatus).
    pub fn builder() -> crate::types::builders::InboundCrossClusterSearchConnectionStatusBuilder {
        crate::types::builders::InboundCrossClusterSearchConnectionStatusBuilder::default()
    }
}

/// A builder for [`InboundCrossClusterSearchConnectionStatus`](crate::types::InboundCrossClusterSearchConnectionStatus).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InboundCrossClusterSearchConnectionStatusBuilder {
    pub(crate) status_code: ::std::option::Option<crate::types::InboundCrossClusterSearchConnectionStatusCode>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
}
impl InboundCrossClusterSearchConnectionStatusBuilder {
    /// <p>The state code for inbound connection. This can be one of the following:</p>
    /// <ul>
    /// <li>PENDING_ACCEPTANCE: Inbound connection is not yet accepted by destination domain owner.</li>
    /// <li>APPROVED: Inbound connection is pending acceptance by destination domain owner.</li>
    /// <li>REJECTING: Inbound connection rejection is in process.</li>
    /// <li>REJECTED: Inbound connection is rejected.</li>
    /// <li>DELETING: Inbound connection deletion is in progress.</li>
    /// <li>DELETED: Inbound connection is deleted and cannot be used further.</li>
    /// </ul>
    pub fn status_code(mut self, input: crate::types::InboundCrossClusterSearchConnectionStatusCode) -> Self {
        self.status_code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state code for inbound connection. This can be one of the following:</p>
    /// <ul>
    /// <li>PENDING_ACCEPTANCE: Inbound connection is not yet accepted by destination domain owner.</li>
    /// <li>APPROVED: Inbound connection is pending acceptance by destination domain owner.</li>
    /// <li>REJECTING: Inbound connection rejection is in process.</li>
    /// <li>REJECTED: Inbound connection is rejected.</li>
    /// <li>DELETING: Inbound connection deletion is in progress.</li>
    /// <li>DELETED: Inbound connection is deleted and cannot be used further.</li>
    /// </ul>
    pub fn set_status_code(mut self, input: ::std::option::Option<crate::types::InboundCrossClusterSearchConnectionStatusCode>) -> Self {
        self.status_code = input;
        self
    }
    /// <p>The state code for inbound connection. This can be one of the following:</p>
    /// <ul>
    /// <li>PENDING_ACCEPTANCE: Inbound connection is not yet accepted by destination domain owner.</li>
    /// <li>APPROVED: Inbound connection is pending acceptance by destination domain owner.</li>
    /// <li>REJECTING: Inbound connection rejection is in process.</li>
    /// <li>REJECTED: Inbound connection is rejected.</li>
    /// <li>DELETING: Inbound connection deletion is in progress.</li>
    /// <li>DELETED: Inbound connection is deleted and cannot be used further.</li>
    /// </ul>
    pub fn get_status_code(&self) -> &::std::option::Option<crate::types::InboundCrossClusterSearchConnectionStatusCode> {
        &self.status_code
    }
    /// <p>Specifies verbose information for the inbound connection status.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies verbose information for the inbound connection status.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>Specifies verbose information for the inbound connection status.</p>
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// Consumes the builder and constructs a [`InboundCrossClusterSearchConnectionStatus`](crate::types::InboundCrossClusterSearchConnectionStatus).
    pub fn build(self) -> crate::types::InboundCrossClusterSearchConnectionStatus {
        crate::types::InboundCrossClusterSearchConnectionStatus {
            status_code: self.status_code,
            message: self.message,
        }
    }
}

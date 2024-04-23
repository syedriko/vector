// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateLogGroupOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for CreateLogGroupOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateLogGroupOutput {
    /// Creates a new builder-style object to manufacture [`CreateLogGroupOutput`](crate::operation::create_log_group::CreateLogGroupOutput).
    pub fn builder() -> crate::operation::create_log_group::builders::CreateLogGroupOutputBuilder {
        crate::operation::create_log_group::builders::CreateLogGroupOutputBuilder::default()
    }
}

/// A builder for [`CreateLogGroupOutput`](crate::operation::create_log_group::CreateLogGroupOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateLogGroupOutputBuilder {
    _request_id: Option<String>,
}
impl CreateLogGroupOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateLogGroupOutput`](crate::operation::create_log_group::CreateLogGroupOutput).
    pub fn build(self) -> crate::operation::create_log_group::CreateLogGroupOutput {
        crate::operation::create_log_group::CreateLogGroupOutput {
            _request_id: self._request_id,
        }
    }
}

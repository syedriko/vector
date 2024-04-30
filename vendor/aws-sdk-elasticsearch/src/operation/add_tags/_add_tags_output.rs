// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AddTagsOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for AddTagsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl AddTagsOutput {
    /// Creates a new builder-style object to manufacture [`AddTagsOutput`](crate::operation::add_tags::AddTagsOutput).
    pub fn builder() -> crate::operation::add_tags::builders::AddTagsOutputBuilder {
        crate::operation::add_tags::builders::AddTagsOutputBuilder::default()
    }
}

/// A builder for [`AddTagsOutput`](crate::operation::add_tags::AddTagsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AddTagsOutputBuilder {
    _request_id: Option<String>,
}
impl AddTagsOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`AddTagsOutput`](crate::operation::add_tags::AddTagsOutput).
    pub fn build(self) -> crate::operation::add_tags::AddTagsOutput {
        crate::operation::add_tags::AddTagsOutput {
            _request_id: self._request_id,
        }
    }
}

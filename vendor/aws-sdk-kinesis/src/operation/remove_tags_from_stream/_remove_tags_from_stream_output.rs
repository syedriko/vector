// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RemoveTagsFromStreamOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for RemoveTagsFromStreamOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RemoveTagsFromStreamOutput {
    /// Creates a new builder-style object to manufacture [`RemoveTagsFromStreamOutput`](crate::operation::remove_tags_from_stream::RemoveTagsFromStreamOutput).
    pub fn builder() -> crate::operation::remove_tags_from_stream::builders::RemoveTagsFromStreamOutputBuilder {
        crate::operation::remove_tags_from_stream::builders::RemoveTagsFromStreamOutputBuilder::default()
    }
}

/// A builder for [`RemoveTagsFromStreamOutput`](crate::operation::remove_tags_from_stream::RemoveTagsFromStreamOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RemoveTagsFromStreamOutputBuilder {
    _request_id: Option<String>,
}
impl RemoveTagsFromStreamOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`RemoveTagsFromStreamOutput`](crate::operation::remove_tags_from_stream::RemoveTagsFromStreamOutput).
    pub fn build(self) -> crate::operation::remove_tags_from_stream::RemoveTagsFromStreamOutput {
        crate::operation::remove_tags_from_stream::RemoveTagsFromStreamOutput {
            _request_id: self._request_id,
        }
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the output for <code>ListTagsForStream</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTagsForStreamOutput {
    /// <p>A list of tags associated with <code>StreamName</code>, starting with the first tag after <code>ExclusiveStartTagKey</code> and up to the specified <code>Limit</code>. </p>
    pub tags: ::std::vec::Vec<crate::types::Tag>,
    /// <p>If set to <code>true</code>, more tags are available. To request additional tags, set <code>ExclusiveStartTagKey</code> to the key of the last tag returned.</p>
    pub has_more_tags: bool,
    _request_id: Option<String>,
}
impl ListTagsForStreamOutput {
    /// <p>A list of tags associated with <code>StreamName</code>, starting with the first tag after <code>ExclusiveStartTagKey</code> and up to the specified <code>Limit</code>. </p>
    pub fn tags(&self) -> &[crate::types::Tag] {
        use std::ops::Deref;
        self.tags.deref()
    }
    /// <p>If set to <code>true</code>, more tags are available. To request additional tags, set <code>ExclusiveStartTagKey</code> to the key of the last tag returned.</p>
    pub fn has_more_tags(&self) -> bool {
        self.has_more_tags
    }
}
impl ::aws_types::request_id::RequestId for ListTagsForStreamOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListTagsForStreamOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForStreamOutput`](crate::operation::list_tags_for_stream::ListTagsForStreamOutput).
    pub fn builder() -> crate::operation::list_tags_for_stream::builders::ListTagsForStreamOutputBuilder {
        crate::operation::list_tags_for_stream::builders::ListTagsForStreamOutputBuilder::default()
    }
}

/// A builder for [`ListTagsForStreamOutput`](crate::operation::list_tags_for_stream::ListTagsForStreamOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListTagsForStreamOutputBuilder {
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) has_more_tags: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl ListTagsForStreamOutputBuilder {
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tags associated with <code>StreamName</code>, starting with the first tag after <code>ExclusiveStartTagKey</code> and up to the specified <code>Limit</code>. </p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of tags associated with <code>StreamName</code>, starting with the first tag after <code>ExclusiveStartTagKey</code> and up to the specified <code>Limit</code>. </p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>A list of tags associated with <code>StreamName</code>, starting with the first tag after <code>ExclusiveStartTagKey</code> and up to the specified <code>Limit</code>. </p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// <p>If set to <code>true</code>, more tags are available. To request additional tags, set <code>ExclusiveStartTagKey</code> to the key of the last tag returned.</p>
    /// This field is required.
    pub fn has_more_tags(mut self, input: bool) -> Self {
        self.has_more_tags = ::std::option::Option::Some(input);
        self
    }
    /// <p>If set to <code>true</code>, more tags are available. To request additional tags, set <code>ExclusiveStartTagKey</code> to the key of the last tag returned.</p>
    pub fn set_has_more_tags(mut self, input: ::std::option::Option<bool>) -> Self {
        self.has_more_tags = input;
        self
    }
    /// <p>If set to <code>true</code>, more tags are available. To request additional tags, set <code>ExclusiveStartTagKey</code> to the key of the last tag returned.</p>
    pub fn get_has_more_tags(&self) -> &::std::option::Option<bool> {
        &self.has_more_tags
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListTagsForStreamOutput`](crate::operation::list_tags_for_stream::ListTagsForStreamOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`tags`](crate::operation::list_tags_for_stream::builders::ListTagsForStreamOutputBuilder::tags)
    /// - [`has_more_tags`](crate::operation::list_tags_for_stream::builders::ListTagsForStreamOutputBuilder::has_more_tags)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::list_tags_for_stream::ListTagsForStreamOutput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::list_tags_for_stream::ListTagsForStreamOutput {
            tags: self.tags.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "tags",
                    "tags was not specified but it is required when building ListTagsForStreamOutput",
                )
            })?,
            has_more_tags: self.has_more_tags.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "has_more_tags",
                    "has_more_tags was not specified but it is required when building ListTagsForStreamOutput",
                )
            })?,
            _request_id: self._request_id,
        })
    }
}

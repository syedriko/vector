// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UntagQueueInput {
    /// <p>The URL of the queue.</p>
    pub queue_url: ::std::option::Option<::std::string::String>,
    /// <p>The list of tags to be removed from the specified queue.</p>
    pub tag_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl UntagQueueInput {
    /// <p>The URL of the queue.</p>
    pub fn queue_url(&self) -> ::std::option::Option<&str> {
        self.queue_url.as_deref()
    }
    /// <p>The list of tags to be removed from the specified queue.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tag_keys.is_none()`.
    pub fn tag_keys(&self) -> &[::std::string::String] {
        self.tag_keys.as_deref().unwrap_or_default()
    }
}
impl UntagQueueInput {
    /// Creates a new builder-style object to manufacture [`UntagQueueInput`](crate::operation::untag_queue::UntagQueueInput).
    pub fn builder() -> crate::operation::untag_queue::builders::UntagQueueInputBuilder {
        crate::operation::untag_queue::builders::UntagQueueInputBuilder::default()
    }
}

/// A builder for [`UntagQueueInput`](crate::operation::untag_queue::UntagQueueInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UntagQueueInputBuilder {
    pub(crate) queue_url: ::std::option::Option<::std::string::String>,
    pub(crate) tag_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl UntagQueueInputBuilder {
    /// <p>The URL of the queue.</p>
    /// This field is required.
    pub fn queue_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.queue_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The URL of the queue.</p>
    pub fn set_queue_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.queue_url = input;
        self
    }
    /// <p>The URL of the queue.</p>
    pub fn get_queue_url(&self) -> &::std::option::Option<::std::string::String> {
        &self.queue_url
    }
    /// Appends an item to `tag_keys`.
    ///
    /// To override the contents of this collection use [`set_tag_keys`](Self::set_tag_keys).
    ///
    /// <p>The list of tags to be removed from the specified queue.</p>
    pub fn tag_keys(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.tag_keys.unwrap_or_default();
        v.push(input.into());
        self.tag_keys = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of tags to be removed from the specified queue.</p>
    pub fn set_tag_keys(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.tag_keys = input;
        self
    }
    /// <p>The list of tags to be removed from the specified queue.</p>
    pub fn get_tag_keys(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.tag_keys
    }
    /// Consumes the builder and constructs a [`UntagQueueInput`](crate::operation::untag_queue::UntagQueueInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::untag_queue::UntagQueueInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::untag_queue::UntagQueueInput {
            queue_url: self.queue_url,
            tag_keys: self.tag_keys,
        })
    }
}

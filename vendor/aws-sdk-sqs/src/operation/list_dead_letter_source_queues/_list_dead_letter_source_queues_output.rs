// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A list of your dead letter source queues.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListDeadLetterSourceQueuesOutput {
    /// <p>A list of source queue URLs that have the <code>RedrivePolicy</code> queue attribute configured with a dead-letter queue.</p>
    pub queue_urls: ::std::vec::Vec<::std::string::String>,
    /// <p>Pagination token to include in the next request. Token value is <code>null</code> if there are no additional results to request, or if you did not set <code>MaxResults</code> in the request.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListDeadLetterSourceQueuesOutput {
    /// <p>A list of source queue URLs that have the <code>RedrivePolicy</code> queue attribute configured with a dead-letter queue.</p>
    pub fn queue_urls(&self) -> &[::std::string::String] {
        use std::ops::Deref;
        self.queue_urls.deref()
    }
    /// <p>Pagination token to include in the next request. Token value is <code>null</code> if there are no additional results to request, or if you did not set <code>MaxResults</code> in the request.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for ListDeadLetterSourceQueuesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListDeadLetterSourceQueuesOutput {
    /// Creates a new builder-style object to manufacture [`ListDeadLetterSourceQueuesOutput`](crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesOutput).
    pub fn builder() -> crate::operation::list_dead_letter_source_queues::builders::ListDeadLetterSourceQueuesOutputBuilder {
        crate::operation::list_dead_letter_source_queues::builders::ListDeadLetterSourceQueuesOutputBuilder::default()
    }
}

/// A builder for [`ListDeadLetterSourceQueuesOutput`](crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListDeadLetterSourceQueuesOutputBuilder {
    pub(crate) queue_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListDeadLetterSourceQueuesOutputBuilder {
    /// Appends an item to `queue_urls`.
    ///
    /// To override the contents of this collection use [`set_queue_urls`](Self::set_queue_urls).
    ///
    /// <p>A list of source queue URLs that have the <code>RedrivePolicy</code> queue attribute configured with a dead-letter queue.</p>
    pub fn queue_urls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.queue_urls.unwrap_or_default();
        v.push(input.into());
        self.queue_urls = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of source queue URLs that have the <code>RedrivePolicy</code> queue attribute configured with a dead-letter queue.</p>
    pub fn set_queue_urls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.queue_urls = input;
        self
    }
    /// <p>A list of source queue URLs that have the <code>RedrivePolicy</code> queue attribute configured with a dead-letter queue.</p>
    pub fn get_queue_urls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.queue_urls
    }
    /// <p>Pagination token to include in the next request. Token value is <code>null</code> if there are no additional results to request, or if you did not set <code>MaxResults</code> in the request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Pagination token to include in the next request. Token value is <code>null</code> if there are no additional results to request, or if you did not set <code>MaxResults</code> in the request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>Pagination token to include in the next request. Token value is <code>null</code> if there are no additional results to request, or if you did not set <code>MaxResults</code> in the request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListDeadLetterSourceQueuesOutput`](crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`queue_urls`](crate::operation::list_dead_letter_source_queues::builders::ListDeadLetterSourceQueuesOutputBuilder::queue_urls)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_dead_letter_source_queues::ListDeadLetterSourceQueuesOutput {
            queue_urls: self.queue_urls.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "queue_urls",
                    "queue_urls was not specified but it is required when building ListDeadLetterSourceQueuesOutput",
                )
            })?,
            next_token: self.next_token,
            _request_id: self._request_id,
        })
    }
}

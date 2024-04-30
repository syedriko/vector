// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListMessageMoveTasksOutput {
    /// <p>A list of message movement tasks and their attributes.</p>
    pub results: ::std::option::Option<::std::vec::Vec<crate::types::ListMessageMoveTasksResultEntry>>,
    _request_id: Option<String>,
}
impl ListMessageMoveTasksOutput {
    /// <p>A list of message movement tasks and their attributes.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.results.is_none()`.
    pub fn results(&self) -> &[crate::types::ListMessageMoveTasksResultEntry] {
        self.results.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for ListMessageMoveTasksOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListMessageMoveTasksOutput {
    /// Creates a new builder-style object to manufacture [`ListMessageMoveTasksOutput`](crate::operation::list_message_move_tasks::ListMessageMoveTasksOutput).
    pub fn builder() -> crate::operation::list_message_move_tasks::builders::ListMessageMoveTasksOutputBuilder {
        crate::operation::list_message_move_tasks::builders::ListMessageMoveTasksOutputBuilder::default()
    }
}

/// A builder for [`ListMessageMoveTasksOutput`](crate::operation::list_message_move_tasks::ListMessageMoveTasksOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListMessageMoveTasksOutputBuilder {
    pub(crate) results: ::std::option::Option<::std::vec::Vec<crate::types::ListMessageMoveTasksResultEntry>>,
    _request_id: Option<String>,
}
impl ListMessageMoveTasksOutputBuilder {
    /// Appends an item to `results`.
    ///
    /// To override the contents of this collection use [`set_results`](Self::set_results).
    ///
    /// <p>A list of message movement tasks and their attributes.</p>
    pub fn results(mut self, input: crate::types::ListMessageMoveTasksResultEntry) -> Self {
        let mut v = self.results.unwrap_or_default();
        v.push(input);
        self.results = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of message movement tasks and their attributes.</p>
    pub fn set_results(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ListMessageMoveTasksResultEntry>>) -> Self {
        self.results = input;
        self
    }
    /// <p>A list of message movement tasks and their attributes.</p>
    pub fn get_results(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ListMessageMoveTasksResultEntry>> {
        &self.results
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListMessageMoveTasksOutput`](crate::operation::list_message_move_tasks::ListMessageMoveTasksOutput).
    pub fn build(self) -> crate::operation::list_message_move_tasks::ListMessageMoveTasksOutput {
        crate::operation::list_message_move_tasks::ListMessageMoveTasksOutput {
            results: self.results,
            _request_id: self._request_id,
        }
    }
}

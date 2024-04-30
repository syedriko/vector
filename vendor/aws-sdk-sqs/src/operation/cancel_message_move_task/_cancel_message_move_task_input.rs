// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CancelMessageMoveTaskInput {
    /// <p>An identifier associated with a message movement task.</p>
    pub task_handle: ::std::option::Option<::std::string::String>,
}
impl CancelMessageMoveTaskInput {
    /// <p>An identifier associated with a message movement task.</p>
    pub fn task_handle(&self) -> ::std::option::Option<&str> {
        self.task_handle.as_deref()
    }
}
impl CancelMessageMoveTaskInput {
    /// Creates a new builder-style object to manufacture [`CancelMessageMoveTaskInput`](crate::operation::cancel_message_move_task::CancelMessageMoveTaskInput).
    pub fn builder() -> crate::operation::cancel_message_move_task::builders::CancelMessageMoveTaskInputBuilder {
        crate::operation::cancel_message_move_task::builders::CancelMessageMoveTaskInputBuilder::default()
    }
}

/// A builder for [`CancelMessageMoveTaskInput`](crate::operation::cancel_message_move_task::CancelMessageMoveTaskInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CancelMessageMoveTaskInputBuilder {
    pub(crate) task_handle: ::std::option::Option<::std::string::String>,
}
impl CancelMessageMoveTaskInputBuilder {
    /// <p>An identifier associated with a message movement task.</p>
    /// This field is required.
    pub fn task_handle(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.task_handle = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An identifier associated with a message movement task.</p>
    pub fn set_task_handle(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.task_handle = input;
        self
    }
    /// <p>An identifier associated with a message movement task.</p>
    pub fn get_task_handle(&self) -> &::std::option::Option<::std::string::String> {
        &self.task_handle
    }
    /// Consumes the builder and constructs a [`CancelMessageMoveTaskInput`](crate::operation::cancel_message_move_task::CancelMessageMoveTaskInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::cancel_message_move_task::CancelMessageMoveTaskInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::cancel_message_move_task::CancelMessageMoveTaskInput {
            task_handle: self.task_handle,
        })
    }
}

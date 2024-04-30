// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Input for GetTopicAttributes action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetTopicAttributesInput {
    /// <p>The ARN of the topic whose properties you want to get.</p>
    pub topic_arn: ::std::option::Option<::std::string::String>,
}
impl GetTopicAttributesInput {
    /// <p>The ARN of the topic whose properties you want to get.</p>
    pub fn topic_arn(&self) -> ::std::option::Option<&str> {
        self.topic_arn.as_deref()
    }
}
impl GetTopicAttributesInput {
    /// Creates a new builder-style object to manufacture [`GetTopicAttributesInput`](crate::operation::get_topic_attributes::GetTopicAttributesInput).
    pub fn builder() -> crate::operation::get_topic_attributes::builders::GetTopicAttributesInputBuilder {
        crate::operation::get_topic_attributes::builders::GetTopicAttributesInputBuilder::default()
    }
}

/// A builder for [`GetTopicAttributesInput`](crate::operation::get_topic_attributes::GetTopicAttributesInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetTopicAttributesInputBuilder {
    pub(crate) topic_arn: ::std::option::Option<::std::string::String>,
}
impl GetTopicAttributesInputBuilder {
    /// <p>The ARN of the topic whose properties you want to get.</p>
    /// This field is required.
    pub fn topic_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.topic_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the topic whose properties you want to get.</p>
    pub fn set_topic_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.topic_arn = input;
        self
    }
    /// <p>The ARN of the topic whose properties you want to get.</p>
    pub fn get_topic_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.topic_arn
    }
    /// Consumes the builder and constructs a [`GetTopicAttributesInput`](crate::operation::get_topic_attributes::GetTopicAttributesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_topic_attributes::GetTopicAttributesInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::get_topic_attributes::GetTopicAttributesInput { topic_arn: self.topic_arn })
    }
}

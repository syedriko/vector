// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeLogGroupsOutput {
    /// <p>The log groups.</p>
    /// <p>If the <code>retentionInDays</code> value is not included for a log group, then that log group's events do not expire.</p>
    pub log_groups: ::std::option::Option<::std::vec::Vec<crate::types::LogGroup>>,
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeLogGroupsOutput {
    /// <p>The log groups.</p>
    /// <p>If the <code>retentionInDays</code> value is not included for a log group, then that log group's events do not expire.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.log_groups.is_none()`.
    pub fn log_groups(&self) -> &[crate::types::LogGroup] {
        self.log_groups.as_deref().unwrap_or_default()
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeLogGroupsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeLogGroupsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeLogGroupsOutput`](crate::operation::describe_log_groups::DescribeLogGroupsOutput).
    pub fn builder() -> crate::operation::describe_log_groups::builders::DescribeLogGroupsOutputBuilder {
        crate::operation::describe_log_groups::builders::DescribeLogGroupsOutputBuilder::default()
    }
}

/// A builder for [`DescribeLogGroupsOutput`](crate::operation::describe_log_groups::DescribeLogGroupsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeLogGroupsOutputBuilder {
    pub(crate) log_groups: ::std::option::Option<::std::vec::Vec<crate::types::LogGroup>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeLogGroupsOutputBuilder {
    /// Appends an item to `log_groups`.
    ///
    /// To override the contents of this collection use [`set_log_groups`](Self::set_log_groups).
    ///
    /// <p>The log groups.</p>
    /// <p>If the <code>retentionInDays</code> value is not included for a log group, then that log group's events do not expire.</p>
    pub fn log_groups(mut self, input: crate::types::LogGroup) -> Self {
        let mut v = self.log_groups.unwrap_or_default();
        v.push(input);
        self.log_groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>The log groups.</p>
    /// <p>If the <code>retentionInDays</code> value is not included for a log group, then that log group's events do not expire.</p>
    pub fn set_log_groups(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LogGroup>>) -> Self {
        self.log_groups = input;
        self
    }
    /// <p>The log groups.</p>
    /// <p>If the <code>retentionInDays</code> value is not included for a log group, then that log group's events do not expire.</p>
    pub fn get_log_groups(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LogGroup>> {
        &self.log_groups
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
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
    /// Consumes the builder and constructs a [`DescribeLogGroupsOutput`](crate::operation::describe_log_groups::DescribeLogGroupsOutput).
    pub fn build(self) -> crate::operation::describe_log_groups::DescribeLogGroupsOutput {
        crate::operation::describe_log_groups::DescribeLogGroupsOutput {
            log_groups: self.log_groups,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
